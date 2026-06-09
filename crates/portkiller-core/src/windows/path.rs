use std::env;
use std::process::Command;

use crate::models::{AppError, PathStatus};

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
    Ok(())
}

pub fn remove_from_path(dir: &str) -> Result<(), AppError> {
    let dir = normalize_segment(dir);
    let current = read_user_path_string()?;
    write_user_path_string(&remove_path_segment(&current, &dir))?;
    Ok(())
}

fn read_user_path_string() -> Result<String, AppError> {
    run_powershell("[Environment]::GetEnvironmentVariable('Path', 'User')")
}

fn write_user_path_string(value: &str) -> Result<(), AppError> {
    let escaped = ps_escape(value);
    run_powershell(&format!(
        "[Environment]::SetEnvironmentVariable('Path', '{escaped}', 'User')"
    ))?;
    Ok(())
}

fn run_powershell(script: &str) -> Result<String, AppError> {
    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .map_err(|e| AppError::Io(e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Other(format!("powershell failed: {stderr}")));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn ps_escape(value: &str) -> String {
    value.replace('\'', "''")
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

    #[test]
    fn ps_escape_doubles_single_quotes() {
        assert_eq!(ps_escape("C:\\it's"), "C:\\it''s");
    }
}
