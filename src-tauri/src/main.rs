#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let has_cli_subcommand = args.len() > 1
        && matches!(
            args[1].as_str(),
            "list" | "search" | "port" | "kill" | "favorites" | "settings" | "help" | "--help" | "-h" | "--version" | "-V"
        );

    if has_cli_subcommand {
        let code = portkiller_cli::run_from_args(args);
        if code != 0 {
            std::process::exit(code);
        }
        return;
    }

    portkiller_win_lib::run();
}
