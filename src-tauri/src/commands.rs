use portkiller_core::{kill_pid, kill_port, scan_ports, AppError, AppSettings, PortInfo, Store};

#[tauri::command]
pub fn list_ports() -> Result<Vec<PortInfo>, String> {
    scan_ports().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn kill_port_cmd(port: u16, force: bool) -> Result<u32, String> {
    kill_port(port, force).map_err(map_kill_error)
}

#[tauri::command]
pub fn kill_pid_cmd(pid: u32, force: bool) -> Result<(), String> {
    kill_pid(pid, force).map_err(map_kill_error)
}

#[tauri::command]
pub fn get_settings() -> Result<AppSettings, String> {
    Store::new()
        .map_err(|e| e.to_string())?
        .load()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_settings(settings: AppSettings) -> Result<(), String> {
    Store::new()
        .map_err(|e| e.to_string())?
        .save(&settings)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_favorite(port: u16) -> Result<Vec<u16>, String> {
    Store::new()
        .map_err(|e| e.to_string())?
        .toggle_favorite(port)
        .map_err(|e| e.to_string())
}

fn map_kill_error(err: AppError) -> String {
    match err {
        AppError::AccessDenied => "access_denied".into(),
        other => other.to_string(),
    }
}
