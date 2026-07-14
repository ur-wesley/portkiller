#[cfg(windows)]
mod windows;
#[cfg(target_os = "macos")]
mod macos;

#[cfg(windows)]
pub use windows::{kill_pid, kill_port, scan_ports};
#[cfg(target_os = "macos")]
pub use macos::{kill_pid, kill_port, scan_ports};

#[cfg(not(any(windows, target_os = "macos")))]
use crate::models::AppError;

#[cfg(not(any(windows, target_os = "macos")))]
pub fn scan_ports() -> Result<Vec<crate::models::PortInfo>, AppError> {
    Err(AppError::Other(
        "port scanning is only supported on Windows and macOS".into(),
    ))
}

#[cfg(not(any(windows, target_os = "macos")))]
pub fn kill_pid(_pid: u32, _force: bool) -> Result<(), AppError> {
    Err(AppError::Other(
        "process kill is only supported on Windows and macOS".into(),
    ))
}

#[cfg(not(any(windows, target_os = "macos")))]
pub fn kill_port(_port: u16, _force: bool) -> Result<u32, AppError> {
    Err(AppError::Other(
        "process kill is only supported on Windows and macOS".into(),
    ))
}
