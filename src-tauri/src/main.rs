// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;
mod config;
mod store;

pub fn main() {
    env_logger::init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            cmd::login,
            cmd::success_auth,
            cmd::set_config,
            cmd::get_config,
            cmd::logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
