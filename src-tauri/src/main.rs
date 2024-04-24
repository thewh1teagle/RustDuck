// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
use tray::EXIT_FLAG;

mod cmd;
mod config;
mod duckdns;
mod setup;
mod store;
mod tray;

#[cfg(target_os = "macos")]
mod dock;

use clap::Parser;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Start app minimized
    #[arg(short, long)]
    minimized: bool,
}

pub fn main() {
    env_logger::init();
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
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
        .run(|app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                if !EXIT_FLAG.load(std::sync::atomic::Ordering::Relaxed) {
                    api.prevent_exit();
                    #[cfg(target_os = "macos")]
                    {
                        dock::set_dock_visible(false);
                    }
                    for (_label, window) in app_handle.webview_windows() {
                        window.close().unwrap();
                    }
                }
            }
        });
}
