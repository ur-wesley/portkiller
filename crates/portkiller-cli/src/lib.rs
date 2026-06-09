use clap::{Parser, Subcommand};
use portkiller_core::{
    fuzzy_search_ports, kill_pid, kill_port, lookup_ports, missing_ports, scan_ports, AppError,
    AppSettings, PortInfo, Store,
};

#[derive(Parser)]
#[command(name = "portkiller", version, about = "Windows port management CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List listening TCP ports
    List {
        #[arg(long)]
        json: bool,
    },
    /// Fuzzy search listening ports
    Search {
        query: String,
        #[arg(long)]
        json: bool,
        #[arg(long, default_value_t = 0)]
        limit: usize,
    },
    /// Look up one or more ports
    Port {
        #[arg(required = true)]
        ports: Vec<u16>,
        #[arg(long)]
        json: bool,
    },
    /// Kill process on a port or by PID
    Kill {
        port: Option<u16>,
        #[arg(long)]
        pid: Option<u32>,
        #[arg(long)]
        force: bool,
    },
    /// Manage favorite ports
    Favorites {
        #[command(subcommand)]
        action: FavoriteAction,
    },
    /// View or update settings
    Settings {
        #[command(subcommand)]
        action: SettingsAction,
    },
    /// Explicit tray mode (handled by Tauri binary)
    Tray,
}

#[derive(Subcommand)]
pub enum FavoriteAction {
    List,
    Add { port: u16 },
    Remove { port: u16 },
}

#[derive(Subcommand)]
pub enum SettingsAction {
    Show,
    Set { key: String, value: String },
}

pub fn run_from_args<I, T>(args: I) -> i32
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let cli = Cli::parse_from(args);
    match run(cli) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("error: {err}");
            err.exit_code()
        }
    }
}

pub fn run(cli: Cli) -> Result<(), AppError> {
    match cli.command {
        Some(Commands::List { json }) => cmd_list(json),
        Some(Commands::Search { query, json, limit }) => cmd_search(query, json, limit),
        Some(Commands::Port { ports, json }) => cmd_port(ports, json),
        Some(Commands::Kill { port, pid, force }) => cmd_kill(port, pid, force),
        Some(Commands::Favorites { action }) => cmd_favorites(action),
        Some(Commands::Settings { action }) => cmd_settings(action),
        Some(Commands::Tray) | None => Err(AppError::Other("tray mode".into())),
    }
}

fn cmd_list(json: bool) -> Result<(), AppError> {
    let ports = scan_ports()?;
    print_ports(&ports, json);
    Ok(())
}

fn cmd_search(query: String, json: bool, limit: usize) -> Result<(), AppError> {
    let ports = scan_ports()?;
    let matches = fuzzy_search_ports(&ports, &query, limit);
    print_ports(&matches, json);
    Ok(())
}

fn cmd_port(targets: Vec<u16>, json: bool) -> Result<(), AppError> {
    let ports = scan_ports()?;
    let missing = missing_ports(&ports, &targets);

    if targets.len() == 1 && !missing.is_empty() {
        return Err(AppError::PortNotFound(targets[0]));
    }

    let matches = lookup_ports(&ports, &targets);
    print_ports(&matches, json);

    for port in &missing {
        eprintln!("port {port}: not listening");
    }

    if let Some(&port) = missing.first() {
        return Err(AppError::PortNotFound(port));
    }

    Ok(())
}

fn print_ports(ports: &[PortInfo], json: bool) {
    if json {
        println!("{}", serde_json::to_string_pretty(ports).unwrap());
    } else {
        print_table(ports);
    }
}

fn print_table(ports: &[PortInfo]) {
    println!("{:<6} {:<8} {:<20} {:<16}", "PORT", "PID", "PROCESS", "ADDRESS");
    for p in ports {
        println!(
            "{:<6} {:<8} {:<20} {:<16}",
            p.port, p.pid, truncate(&p.process_name, 20), truncate(&p.address, 16)
        );
    }
}

fn truncate(value: &str, max: usize) -> String {
    if value.len() <= max {
        value.to_string()
    } else {
        format!("{}…", &value[..max.saturating_sub(1)])
    }
}

fn cmd_kill(port: Option<u16>, pid: Option<u32>, force: bool) -> Result<(), AppError> {
    match (port, pid) {
        (Some(port), None) => {
            let count = kill_port(port, force)?;
            if count == 0 {
                return Err(AppError::PortNotFound(port));
            }
            println!("killed {count} process(es) on port {port}");
            Ok(())
        }
        (None, Some(pid)) => {
            kill_pid(pid, force)?;
            println!("killed process {pid}");
            Ok(())
        }
        (Some(_), Some(_)) => Err(AppError::Other(
            "specify either a port or --pid, not both".into(),
        )),
        (None, None) => Err(AppError::Other("specify a port or --pid".into())),
    }
}

fn cmd_favorites(action: FavoriteAction) -> Result<(), AppError> {
    let store = Store::new()?;
    match action {
        FavoriteAction::List => {
            let settings = store.load()?;
            for port in settings.favorites {
                println!("{port}");
            }
            Ok(())
        }
        FavoriteAction::Add { port } => {
            store.add_favorite(port)?;
            println!("added favorite {port}");
            Ok(())
        }
        FavoriteAction::Remove { port } => {
            store.remove_favorite(port)?;
            println!("removed favorite {port}");
            Ok(())
        }
    }
}

fn cmd_settings(action: SettingsAction) -> Result<(), AppError> {
    let store = Store::new()?;
    match action {
        SettingsAction::Show => {
            let settings = store.load()?;
            println!("{}", serde_json::to_string_pretty(&settings).unwrap());
            Ok(())
        }
        SettingsAction::Set { key, value } => {
            let mut settings = store.load()?;
            apply_setting(&mut settings, &key, &value)?;
            store.save(&settings)?;
            println!("updated {key}");
            Ok(())
        }
    }
}

fn apply_setting(settings: &mut AppSettings, key: &str, value: &str) -> Result<(), AppError> {
    match key {
        "refresh_interval_secs" => {
            settings.refresh_interval_secs = value
                .parse()
                .map_err(|_| AppError::Settings("invalid refresh_interval_secs".into()))?;
        }
        "start_minimized" => {
            settings.start_minimized = parse_bool(value)?;
        }
        "autostart" => {
            settings.autostart = parse_bool(value)?;
        }
        "add_to_path" => {
            settings.add_to_path = parse_bool(value)?;
        }
        "locale" => {
            settings.locale = value.to_string();
        }
        other => return Err(AppError::Settings(format!("unknown setting: {other}"))),
    }
    Ok(())
}

fn parse_bool(value: &str) -> Result<bool, AppError> {
    match value.to_lowercase().as_str() {
        "true" | "1" | "yes" => Ok(true),
        "false" | "0" | "no" => Ok(false),
        _ => Err(AppError::Settings(format!("invalid bool: {value}"))),
    }
}
