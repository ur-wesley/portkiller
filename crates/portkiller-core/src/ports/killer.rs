use std::thread;
use std::time::Duration;

use sysinfo::{Pid, System};

use crate::models::AppError;
use crate::ports::scanner::scan_ports;

pub fn kill_pid(pid: u32, force: bool) -> Result<(), AppError> {
    if pid == 0 {
        return Err(AppError::ProcessNotFound(pid));
    }

    if !force && try_graceful_close(pid) {
        return Ok(());
    }

    force_kill(pid)
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

fn try_graceful_close(pid: u32) -> bool {
    if !post_close_to_main_window(pid) {
        return false;
    }
    thread::sleep(Duration::from_millis(500));
    !process_exists(pid)
}

fn post_close_to_main_window(pid: u32) -> bool {
    use std::sync::atomic::{AtomicBool, Ordering};

    static FOUND: AtomicBool = AtomicBool::new(false);

    unsafe extern "system" fn enum_proc(hwnd: windows_sys::Win32::Foundation::HWND, lparam: isize) -> i32 {
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            GetWindowThreadProcessId, IsWindowVisible, PostMessageW, WM_CLOSE,
        };

        let target_pid = lparam as u32;
        let mut window_pid = 0u32;
        GetWindowThreadProcessId(hwnd, &mut window_pid);
        if window_pid == target_pid && IsWindowVisible(hwnd) != 0 {
            PostMessageW(hwnd, WM_CLOSE, 0, 0);
            FOUND.store(true, Ordering::SeqCst);
        }
        1
    }

    FOUND.store(false, Ordering::SeqCst);
    unsafe {
        use windows_sys::Win32::UI::WindowsAndMessaging::EnumWindows;
        EnumWindows(Some(enum_proc), pid as isize);
    }
    FOUND.load(Ordering::SeqCst)
}

fn force_kill(pid: u32) -> Result<(), AppError> {
    let mut system = System::new();
    system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    let process = system
        .process(Pid::from_u32(pid))
        .ok_or(AppError::ProcessNotFound(pid))?;

    if !process.kill() {
        return Err(AppError::AccessDenied);
    }
    Ok(())
}

fn process_exists(pid: u32) -> bool {
    let mut system = System::new();
    system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    system.process(Pid::from_u32(pid)).is_some()
}
