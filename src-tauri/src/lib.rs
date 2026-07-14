mod commands;
mod platform;
mod updater;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use portkiller_core::{diff_ports, PortInfo};
use serde::Serialize;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};
use tauri_plugin_autostart::MacosLauncher;

static MONITOR_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize)]
struct MonitoringStatus {
    active: bool,
    last_updated_ms: Option<u64>,
}

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
                        let _ = scan_and_emit(app);
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
                        toggle_main_window(app);
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
                let _ = platform::apply_glass(&window);
            }

            spawn_monitor(app_handle);
            Ok(())
        })
        .on_window_event(|window, event| {
            platform::configure_window_events(window, event);
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn toggle_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

fn spawn_monitor(app: tauri::AppHandle) {
    if MONITOR_RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }

    tauri::async_runtime::spawn(async move {
        let mut last_ports: Vec<PortInfo> = Vec::new();
        let mut last_updated_ms: Option<u64> = None;

        loop {
            let settings = portkiller_core::Store::new()
                .and_then(|s| s.load())
                .unwrap_or_default();

            if settings.monitoring_enabled {
                if let Ok(ports) = portkiller_core::scan_ports() {
                    let _changes = diff_ports(&last_ports, &ports);
                    last_ports = ports.clone();
                    last_updated_ms = Some(now_ms());
                    let _ = app.emit("ports-updated", &ports);
                }
            }

            let _ = emit_monitoring_status(
                &app,
                MonitoringStatus {
                    active: settings.monitoring_enabled,
                    last_updated_ms,
                },
            );

            let sleep_secs = if settings.monitoring_enabled {
                settings.refresh_interval_secs.max(1)
            } else {
                1
            };
            tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
        }
    });
}

fn scan_and_emit(app: &tauri::AppHandle) -> Result<(), String> {
    let ports = portkiller_core::scan_ports().map_err(|e| e.to_string())?;
    app.emit("ports-updated", &ports)
        .map_err(|e| e.to_string())?;
    emit_monitoring_status(
        app,
        MonitoringStatus {
            active: true,
            last_updated_ms: Some(now_ms()),
        },
    )
}

fn emit_monitoring_status(app: &tauri::AppHandle, status: MonitoringStatus) -> Result<(), String> {
    app.emit("monitoring-status", status)
        .map_err(|e| e.to_string())
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
