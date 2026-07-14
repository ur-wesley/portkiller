use tauri::{Window, WindowEvent};

pub fn configure_window_events(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            let close_to_tray = portkiller_core::Store::new()
                .and_then(|s| s.load())
                .map(|s| s.close_to_tray)
                .unwrap_or(true);

            if close_to_tray {
                api.prevent_close();
                let _ = window.hide();
            }
        }
        _ => {}
    }
}
