mod commands;
mod updater;

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_positioner::{Position, WindowExt};

static REFRESH_RUNNING: AtomicBool = AtomicBool::new(false);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            commands::list_ports,
            commands::kill_port_cmd,
            commands::kill_pid_cmd,
            commands::get_settings,
            commands::save_settings,
            commands::toggle_favorite,
            commands::path_status_cmd,
            commands::path_add,
            commands::path_remove,
            updater::check_for_updates,
            updater::install_update,
        ])
        .setup(|app| {
            let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let refresh_item = MenuItem::with_id(app, "refresh", "Refresh", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &refresh_item, &quit_item])?;

            let icon = app.default_window_icon().cloned().expect("missing icon");
            let app_handle = app.handle().clone();

            TrayIconBuilder::new()
                .icon(icon)
                .tooltip("PortKiller")
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "show" => show_main_window(app),
                    "refresh" => {
                        let _ = emit_ports(app);
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);

                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        toggle_main_window(&app);
                    }
                })
                .build(app)?;

            if let Some(window) = app.get_webview_window("main") {
                let settings = portkiller_core::Store::new()
                    .and_then(|s| s.load())
                    .unwrap_or_default();
                if settings.start_minimized {
                    let _ = window.hide();
                }
            }

            spawn_refresh_task(app_handle);
            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    let _ = window.hide();
                }
                WindowEvent::Focused(false) => {
                    let _ = window.hide();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.move_window(Position::TrayCenter);
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn toggle_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.move_window(Position::TrayCenter);
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

fn spawn_refresh_task(app: tauri::AppHandle) {
    if REFRESH_RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }

    tauri::async_runtime::spawn(async move {
        loop {
            let interval = portkiller_core::Store::new()
                .and_then(|s| s.load())
                .map(|s| s.refresh_interval_secs)
                .unwrap_or(5)
                .max(1);

            tokio::time::sleep(Duration::from_secs(interval)).await;
            let _ = emit_ports(&app);
        }
    });
}

fn emit_ports(app: &tauri::AppHandle) -> Result<(), String> {
    let ports = portkiller_core::scan_ports().map_err(|e| e.to_string())?;
    app.emit("ports-updated", ports).map_err(|e| e.to_string())
}
