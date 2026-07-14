mod glass;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "macos")]
mod macos;

pub use glass::apply_glass;

#[cfg(target_os = "windows")]
pub use windows::configure_window_events;
#[cfg(target_os = "macos")]
pub use macos::configure_window_events;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn configure_window_events(window: &tauri::Window, event: &tauri::WindowEvent) {
    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        let _ = window.hide();
    }
}
