use portkiller_core::{
    add_to_path, kill_pid, kill_port, path_status, remove_from_path, scan_ports, AppError,
    AppSettings, PathStatus, PortInfo, Store,
};

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

#[tauri::command]
pub fn path_status_cmd() -> Result<PathStatus, String> {
    path_status().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn path_add() -> Result<(), String> {
    let dir = portkiller_core::get_install_dir();
    add_to_path(&dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn path_remove() -> Result<(), String> {
    let dir = portkiller_core::get_install_dir();
    remove_from_path(&dir).map_err(|e| e.to_string())
}

fn map_kill_error(err: AppError) -> String {
    match err {
        AppError::AccessDenied => "access_denied".into(),
        other => other.to_string(),
    }
}
