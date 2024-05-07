// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_plugin_autostart::MacosLauncher;

mod cmd;
mod config;
mod duckdns;
mod runtime;
mod setup;
mod store;
mod tray;
use clap::Parser;

#[cfg(target_os = "macos")]
mod dock;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Start app minimized
    #[arg(short, long)]
    minimized: bool,
}

pub fn main() {
    env_logger::init();
    log::debug!("init");
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            log::debug!("single instance event");
            cmd::open_main_window(app).unwrap();
        }))
        .setup(setup::setup)
        .invoke_handler(tauri::generate_handler![
            cmd::login,
            cmd::success_auth,
            cmd::set_config,
            cmd::get_config,
            cmd::logout,
            cmd::update_domains,
            cmd::show_window,
            cmd::app_info
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(runtime::on_run_event);
}
