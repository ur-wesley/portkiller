use std::env;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;

use winreg::enums::*;
use winreg::RegKey;

use crate::models::{AppError, PathStatus};

const ENVIRONMENT: &str = "Environment";
const PATH_VALUE: &str = "Path";

pub fn get_install_dir() -> String {
    env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_string_lossy().into_owned()))
        .unwrap_or_default()
}

pub fn path_status() -> Result<PathStatus, AppError> {
    let install_dir = get_install_dir();
    let in_path = if install_dir.is_empty() {
        false
    } else {
        is_in_path(&install_dir)?
    };
    Ok(PathStatus { in_path, install_dir })
}

pub fn is_in_path(dir: &str) -> Result<bool, AppError> {
    let normalized = normalize_path(dir);
    Ok(read_user_path()?
        .split(';')
        .filter(|s| !s.is_empty())
        .any(|entry| normalize_path(entry) == normalized))
}

pub fn add_to_path(dir: &str) -> Result<(), AppError> {
    let normalized = normalize_path(dir);
    if normalized.is_empty() {
        return Err(AppError::Other("install directory is empty".into()));
    }
    if is_in_path(&normalized)? {
        return Ok(());
    }
    let mut entries = read_user_path_entries()?;
    entries.push(normalized);
    write_user_path(&entries)?;
    broadcast_environment_change();
    Ok(())
}

pub fn remove_from_path(dir: &str) -> Result<(), AppError> {
    let normalized = normalize_path(dir);
    let entries: Vec<String> = read_user_path_entries()?
        .into_iter()
        .filter(|entry| normalize_path(entry) != normalized)
        .collect();
    write_user_path(&entries)?;
    broadcast_environment_change();
    Ok(())
}

fn read_user_path() -> Result<String, AppError> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu.open_subkey(ENVIRONMENT).map_err(|e| AppError::Io(e.into()))?;
    let path: String = env.get_value(PATH_VALUE).unwrap_or_default();
    Ok(path)
}

fn read_user_path_entries() -> Result<Vec<String>, AppError> {
    Ok(read_user_path()?
        .split(';')
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect())
}

fn write_user_path(entries: &[String]) -> Result<(), AppError> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu
        .create_subkey(ENVIRONMENT)
        .map_err(|e| AppError::Io(e.into()))?;
    let joined = entries.join(";");
    env.set_value(PATH_VALUE, &joined)
        .map_err(|e| AppError::Io(e.into()))?;
    Ok(())
}

fn normalize_path(path: &str) -> String {
    Path::new(path)
        .canonicalize()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.trim_end_matches('\\').to_string())
}

fn broadcast_environment_change() {
    let param = "Environment";
    let wide: Vec<u16> = OsStr::new(param).encode_wide().chain(Some(0)).collect();
    unsafe {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
        };
        SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0,
            wide.as_ptr() as isize,
            SMTO_ABORTIFHUNG,
            1000,
            std::ptr::null_mut(),
        );
    }
}
