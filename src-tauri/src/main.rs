// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
use tray::EXIT_FLAG;

mod cmd;
mod config;
mod dock;
mod duckdns;
mod setup;
mod store;
mod tray;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

pub fn main() {
    env_logger::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            cmd::open_main_window(app).unwrap();
            app.emit("single-instance", serde_json::json!({})).unwrap();
        }))
        .setup(setup::setup)
        .invoke_handler(tauri::generate_handler![
            cmd::login,
            cmd::success_auth,
            cmd::set_config,
            cmd::get_config,
            cmd::logout,
            cmd::update_domains
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                if !EXIT_FLAG.load(std::sync::atomic::Ordering::Relaxed) {
                    api.prevent_exit();
                    dock::set_dock_visible(false);
                    for (_label, window) in app_handle.webview_windows() {
                        window.close().unwrap();
                    }
                }
            }
            _ => {}
        });
}
