#[cfg(windows)]
mod path;

#[cfg(windows)]
pub use path::{add_to_path, get_install_dir, is_in_path, path_status, remove_from_path};

#[cfg(not(windows))]
use crate::models::{AppError, PathStatus};

#[cfg(not(windows))]
pub fn is_in_path(_dir: &str) -> Result<bool, AppError> {
    Ok(false)
}

#[cfg(not(windows))]
pub fn add_to_path(_dir: &str) -> Result<(), AppError> {
    Err(AppError::Other("PATH management is only supported on Windows".into()))
}

#[cfg(not(windows))]
pub fn remove_from_path(_dir: &str) -> Result<(), AppError> {
    Err(AppError::Other("PATH management is only supported on Windows".into()))
}

#[cfg(not(windows))]
pub fn get_install_dir() -> String {
    String::new()
}

#[cfg(not(windows))]
pub fn path_status() -> Result<PathStatus, AppError> {
    Ok(PathStatus {
        in_path: false,
        install_dir: String::new(),
    })
}
