mod filter;

#[cfg(windows)]
mod killer;
#[cfg(windows)]
mod scanner;

pub use filter::{fuzzy_search_ports, lookup_ports, missing_ports};

#[cfg(windows)]
pub use killer::{kill_pid, kill_port};
#[cfg(windows)]
pub use scanner::scan_ports;

#[cfg(not(windows))]
use crate::models::AppError;

#[cfg(not(windows))]
pub fn scan_ports() -> Result<Vec<crate::models::PortInfo>, AppError> {
    Err(AppError::Other("port scanning is only supported on Windows".into()))
}

#[cfg(not(windows))]
pub fn kill_pid(_pid: u32, _force: bool) -> Result<(), AppError> {
    Err(AppError::Other("process kill is only supported on Windows".into()))
}

#[cfg(not(windows))]
pub fn kill_port(_port: u16, _force: bool) -> Result<u32, AppError> {
    Err(AppError::Other("process kill is only supported on Windows".into()))
}
