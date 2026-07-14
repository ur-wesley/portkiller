use std::thread;
use std::time::Duration;

use libc::{c_int, kill, SIGKILL, SIGTERM};

use crate::models::AppError;

use super::scanner::scan_ports;

pub fn kill_pid(pid: u32, force: bool) -> Result<(), AppError> {
    if pid == 0 {
        return Err(AppError::ProcessNotFound(pid));
    }

    let sig: c_int = if force { SIGKILL } else { SIGTERM };
    send_signal(pid, sig)?;

    if !force {
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}

pub fn kill_port(port: u16, force: bool) -> Result<u32, AppError> {
    let ports = scan_ports()?;
    let matches: Vec<u32> = ports
        .into_iter()
        .filter(|p| p.port == port)
        .map(|p| p.pid)
        .collect();

    if matches.is_empty() {
        return Err(AppError::PortNotFound(port));
    }

    let mut killed = 0u32;
    for pid in matches {
        match kill_pid(pid, force) {
            Ok(()) => killed += 1,
            Err(AppError::AccessDenied) => return Err(AppError::AccessDenied),
            Err(_) => {}
        }
    }
    Ok(killed)
}

fn send_signal(pid: u32, sig: c_int) -> Result<(), AppError> {
    let result = unsafe { kill(pid as i32, sig) };
    if result == 0 {
        return Ok(());
    }

    let err = std::io::Error::last_os_error();
    match err.raw_os_error() {
        Some(libc::EPERM) => Err(AppError::AccessDenied),
        Some(libc::ESRCH) => Err(AppError::ProcessNotFound(pid)),
        _ => Err(AppError::Io(err)),
    }
}
