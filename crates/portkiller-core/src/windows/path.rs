use std::env;
use std::ffi::OsStr;
use std::io;
use std::os::windows::ffi::OsStrExt;

use winreg::enums::*;
use winreg::types::FromRegValue;
use winreg::{RegKey, RegValue};

use crate::models::{AppError, PathStatus};

const ENVIRONMENT: &str = "Environment";
const PATH_VALUE: &str = "Path";
const LEGACY_PATH_VALUE: &str = "PATH";

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
    let dir = normalize_segment(dir);
    if dir.is_empty() {
        return Ok(false);
    }
    Ok(contains_path_segment(&read_user_path_string()?, &dir))
}

pub fn add_to_path(dir: &str) -> Result<(), AppError> {
    let dir = normalize_segment(dir);
    if dir.is_empty() {
        return Err(AppError::Other("install directory is empty".into()));
    }
    let current = read_user_path_string()?;
    if contains_path_segment(&current, &dir) {
        return Ok(());
    }
    write_user_path_string(&append_segment(&current, &dir))?;
    broadcast_environment_change();
    Ok(())
}

pub fn remove_from_path(dir: &str) -> Result<(), AppError> {
    let dir = normalize_segment(dir);
    let current = read_user_path_string()?;
    write_user_path_string(&remove_path_segment(&current, &dir))?;
    broadcast_environment_change();
    Ok(())
}

fn read_user_path_string() -> Result<String, AppError> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let env = hkcu
        .open_subkey(ENVIRONMENT)
        .map_err(|e| AppError::Io(e.into()))?;

    match read_path_value(&env, PATH_VALUE)? {
        Some(value) if !value.is_empty() => Ok(value),
        _ => Ok(read_path_value(&env, LEGACY_PATH_VALUE)?.unwrap_or_default()),
    }
}

fn read_path_value(env: &RegKey, value_name: &str) -> Result<Option<String>, AppError> {
    match env.get_raw_value(value_name) {
        Ok(reg_value) => decode_path_string(&reg_value).map(Some),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(AppError::Io(e)),
    }
}

fn decode_path_string(reg_value: &RegValue) -> Result<String, AppError> {
    match reg_value.vtype {
        REG_SZ | REG_EXPAND_SZ => String::from_reg_value(reg_value).map_err(|e| AppError::Io(e)),
        _ => Err(AppError::Other("unsupported registry type for Path".into())),
    }
}

fn write_user_path_string(value: &str) -> Result<(), AppError> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu
        .create_subkey(ENVIRONMENT)
        .map_err(|e| AppError::Io(e.into()))?;

    let reg_value = RegValue {
        bytes: encode_utf16(value),
        vtype: REG_EXPAND_SZ,
    };

    env.set_raw_value(PATH_VALUE, &reg_value)
        .map_err(|e| AppError::Io(e))?;

    let _ = env.delete_value(LEGACY_PATH_VALUE);

    Ok(())
}

fn append_segment(current: &str, dir: &str) -> String {
    if current.is_empty() {
        return dir.to_string();
    }
    if current.ends_with(';') {
        format!("{current}{dir}")
    } else {
        format!("{current};{dir}")
    }
}

fn remove_path_segment(current: &str, dir: &str) -> String {
    split_path_segments(current)
        .into_iter()
        .filter(|segment| !segments_equal(segment, dir))
        .collect::<Vec<_>>()
        .join(";")
}

fn contains_path_segment(current: &str, dir: &str) -> bool {
    split_path_segments(current)
        .iter()
        .any(|segment| segments_equal(segment, dir))
}

fn split_path_segments(raw: &str) -> Vec<String> {
    raw.split(';')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(str::to_string)
        .collect()
}

fn segments_equal(left: &str, right: &str) -> bool {
    normalize_segment(left).eq_ignore_ascii_case(&normalize_segment(right))
}

fn normalize_segment(path: &str) -> String {
    path.trim().trim_end_matches('\\').to_string()
}

fn encode_utf16(value: &str) -> Vec<u8> {
    OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .flat_map(|unit| unit.to_le_bytes())
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_path_segments_ignores_empty_segments() {
        assert_eq!(
            split_path_segments(";C:\\Tools;;D:\\Apps;"),
            vec!["C:\\Tools", "D:\\Apps"]
        );
    }

    #[test]
    fn append_segment_on_empty() {
        assert_eq!(append_segment("", "C:\\PortKiller"), "C:\\PortKiller");
    }

    #[test]
    fn append_segment_on_existing() {
        assert_eq!(
            append_segment("C:\\Tools", "C:\\PortKiller"),
            "C:\\Tools;C:\\PortKiller"
        );
    }

    #[test]
    fn contains_path_segment_is_case_insensitive() {
        assert!(contains_path_segment(
            "C:\\Tools;C:\\PortKiller",
            "c:\\portkiller\\"
        ));
    }

    #[test]
    fn remove_path_segment_removes_only_target() {
        assert_eq!(
            remove_path_segment("C:\\Tools;C:\\PortKiller", "C:\\PortKiller"),
            "C:\\Tools"
        );
    }
}
