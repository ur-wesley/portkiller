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
    is_in_path_with_store(&RegistryStore, dir)
}

pub fn add_to_path(dir: &str) -> Result<(), AppError> {
    add_to_path_with_store(&RegistryStore, dir)
}

pub fn remove_from_path(dir: &str) -> Result<(), AppError> {
    remove_from_path_with_store(&RegistryStore, dir)
}

trait UserPathStore {
    fn read(&self) -> Result<String, AppError>;
    fn write(&self, value: &str) -> Result<(), AppError>;
}

struct RegistryStore;

impl UserPathStore for RegistryStore {
    fn read(&self) -> Result<String, AppError> {
        read_user_path_from_registry()
    }

    fn write(&self, value: &str) -> Result<(), AppError> {
        write_user_path_to_registry(value)?;
        broadcast_environment_change();
        Ok(())
    }
}

fn is_in_path_with_store(store: &impl UserPathStore, dir: &str) -> Result<bool, AppError> {
    let dir = normalize_segment(dir);
    if dir.is_empty() {
        return Ok(false);
    }
    Ok(contains_path_segment(&store.read()?, &dir))
}

fn add_to_path_with_store(store: &impl UserPathStore, dir: &str) -> Result<(), AppError> {
    let dir = normalize_segment(dir);
    if dir.is_empty() {
        return Err(AppError::Other("install directory is empty".into()));
    }
    let current = store.read()?;
    if contains_path_segment(&current, &dir) {
        return Ok(());
    }
    store.write(&append_segment(&current, &dir))?;
    Ok(())
}

fn remove_from_path_with_store(store: &impl UserPathStore, dir: &str) -> Result<(), AppError> {
    let dir = normalize_segment(dir);
    let current = store.read()?;
    store.write(&remove_path_segment(&current, &dir))?;
    Ok(())
}

fn read_user_path_from_registry() -> Result<String, AppError> {
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

fn write_user_path_to_registry(value: &str) -> Result<(), AppError> {
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
    use std::cell::RefCell;

    use super::*;

    struct InMemoryStore(RefCell<String>);

    impl InMemoryStore {
        fn new(initial: &str) -> Self {
            Self(RefCell::new(initial.to_string()))
        }
    }

    impl UserPathStore for InMemoryStore {
        fn read(&self) -> Result<String, AppError> {
            Ok(self.0.borrow().clone())
        }

        fn write(&self, value: &str) -> Result<(), AppError> {
            *self.0.borrow_mut() = value.to_string();
            Ok(())
        }
    }

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
    fn append_segment_when_current_ends_with_semicolon() {
        assert_eq!(
            append_segment("C:\\Tools;", "C:\\PortKiller"),
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
    fn contains_path_segment_with_spaces_and_special_chars() {
        assert!(contains_path_segment(
            "C:\\Program Files\\Tools;C:\\Apps & More",
            "C:\\Apps & More"
        ));
        assert!(contains_path_segment(
            "C:\\Users\\me\\bin;%LOCALAPPDATA%\\bin",
            "%LOCALAPPDATA%\\bin"
        ));
    }

    #[test]
    fn remove_path_segment_removes_only_target() {
        assert_eq!(
            remove_path_segment("C:\\Tools;C:\\PortKiller", "C:\\PortKiller"),
            "C:\\Tools"
        );
    }

    #[test]
    fn remove_path_segment_from_middle() {
        assert_eq!(
            remove_path_segment("C:\\First;C:\\Target;C:\\Last", "C:\\Target"),
            "C:\\First;C:\\Last"
        );
    }

    #[test]
    fn remove_path_segment_from_first() {
        assert_eq!(
            remove_path_segment("C:\\Target;C:\\Last", "C:\\Target"),
            "C:\\Last"
        );
    }

    #[test]
    fn remove_path_segment_from_last() {
        assert_eq!(
            remove_path_segment("C:\\First;C:\\Target", "C:\\Target"),
            "C:\\First"
        );
    }

    #[test]
    fn remove_path_segment_when_absent_is_unchanged() {
        assert_eq!(
            remove_path_segment("C:\\Tools;C:\\Apps", "C:\\Missing"),
            "C:\\Tools;C:\\Apps"
        );
    }

    #[test]
    fn remove_path_segment_removes_all_duplicates() {
        assert_eq!(
            remove_path_segment(
                "C:\\Tools;C:\\PortKiller;C:\\Apps;C:\\PortKiller",
                "C:\\PortKiller"
            ),
            "C:\\Tools;C:\\Apps"
        );
    }

    #[test]
    fn normalize_segment_trims_trailing_backslash() {
        assert_eq!(normalize_segment("C:\\Tools\\"), "C:\\Tools");
        assert_eq!(normalize_segment("  C:\\Tools\\  "), "C:\\Tools");
    }

    #[test]
    fn add_to_path_preserves_existing_entries() {
        let store = InMemoryStore::new("C:\\Tools;C:\\Apps");
        add_to_path_with_store(&store, "C:\\PortKiller").unwrap();
        assert_eq!(store.read().unwrap(), "C:\\Tools;C:\\Apps;C:\\PortKiller");
    }

    #[test]
    fn add_to_path_on_empty_store() {
        let store = InMemoryStore::new("");
        add_to_path_with_store(&store, "C:\\PortKiller").unwrap();
        assert_eq!(store.read().unwrap(), "C:\\PortKiller");
    }

    #[test]
    fn add_to_path_is_idempotent() {
        let store = InMemoryStore::new("C:\\Tools;C:\\PortKiller");
        add_to_path_with_store(&store, "C:\\PortKiller").unwrap();
        assert_eq!(store.read().unwrap(), "C:\\Tools;C:\\PortKiller");
    }

    #[test]
    fn remove_from_path_restores_original() {
        let store = InMemoryStore::new("C:\\Tools;C:\\Apps;C:\\PortKiller");
        remove_from_path_with_store(&store, "C:\\PortKiller").unwrap();
        assert_eq!(store.read().unwrap(), "C:\\Tools;C:\\Apps");
    }

    #[test]
    fn remove_from_path_when_absent_leaves_others() {
        let store = InMemoryStore::new("C:\\Tools;C:\\Apps");
        remove_from_path_with_store(&store, "C:\\Missing").unwrap();
        assert_eq!(store.read().unwrap(), "C:\\Tools;C:\\Apps");
    }

    #[test]
    fn is_in_path_detects_segment() {
        let store = InMemoryStore::new("C:\\Tools;C:\\PortKiller");
        assert!(is_in_path_with_store(&store, "c:\\portkiller").unwrap());
        assert!(!is_in_path_with_store(&store, "C:\\Missing").unwrap());
    }

    #[test]
    fn add_to_path_rejects_empty_dir() {
        let store = InMemoryStore::new("C:\\Tools");
        let err = add_to_path_with_store(&store, "").unwrap_err();
        assert!(matches!(err, AppError::Other(_)));
        assert_eq!(store.read().unwrap(), "C:\\Tools");
    }

    #[test]
    fn round_trip_add_then_remove() {
        let store = InMemoryStore::new("C:\\Node;C:\\Python;C:\\Git\\cmd");
        add_to_path_with_store(&store, "C:\\PortKiller").unwrap();
        assert_eq!(
            store.read().unwrap(),
            "C:\\Node;C:\\Python;C:\\Git\\cmd;C:\\PortKiller"
        );
        remove_from_path_with_store(&store, "C:\\PortKiller").unwrap();
        assert_eq!(store.read().unwrap(), "C:\\Node;C:\\Python;C:\\Git\\cmd");
    }
}
