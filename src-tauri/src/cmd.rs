use eyre::Result;
use serde_json::Value;
use std::{str::FromStr, sync::Arc};
use tauri::{command, AppHandle, Manager, State, Url, WebviewWindowBuilder};
use tokio::sync::Mutex;

use crate::{
    config::{self, DomainsConfig},
    duckdns, store,
};
use std::path::PathBuf;

#[tauri::command]
pub fn app_info(app_handle: AppHandle) -> Value {
    let commit = env!("COMMIT_HASH").to_string();
    let version = app_handle.package_info().version.to_string();
    serde_json::json!({"commit": commit, "version": version})
}

#[command]
pub async fn update_domains(config: State<'_, Arc<Mutex<Option<DomainsConfig>>>>) -> Result<()> {
    let config = config.lock().await;
    if let Some(config) = &*config {
        duckdns::update_domains(config.clone()).await?;
    }
    Ok(())
}

#[command]
pub fn show_window(app_handle: AppHandle, label: String) {
    if let Some(window) = app_handle.get_webview_window(&label) {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

#[command]
pub fn open_main_window(app_handle: &AppHandle) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        crate::dock::set_dock_visible(true);
    }

    if let Some(window) = app_handle.get_webview_window("main") {
        window.show().unwrap();
        window.set_focus().unwrap();
    } else {
        let url = tauri::WebviewUrl::App(PathBuf::from_str("/index.html").unwrap());
        tauri::WebviewWindowBuilder::new(app_handle, "main", url)
            .title("RustDuck")
            .inner_size(800.0, 600.0)
            .visible(false)
            .build()
            .unwrap();
    }

    Ok(())
}

#[command]
pub async fn login(app_handle: AppHandle) {
    let url = Url::from_str(config::LOGIN_URL).unwrap();
    let url = tauri::WebviewUrl::External(url);
    WebviewWindowBuilder::new(&app_handle, "duckdns", url)
        .visible(true)
        .title("DuckDNS Login")
        .initialization_script(include_str!("../scripts/duckdns.js"))
        .build()
        .unwrap();
}

#[command]
pub async fn success_auth(app_handle: AppHandle, config: DomainsConfig) -> Result<()> {
    if let Some(window) = app_handle.get_webview_window("duckdns") {
        window.close().unwrap();
    }
    if let Some(window) = app_handle.get_webview_window("main") {
        // store login details
        store::setup(app_handle, config).await?;
        window.show().unwrap();
        window.set_focus().unwrap();
        window.emit("success_auth", serde_json::json!({})).unwrap();
    }
    Ok(())
}

#[command]
pub async fn get_config(app_handle: AppHandle) -> Result<Option<DomainsConfig>> {
    store::get(&app_handle)
}

#[command]
pub async fn set_config(app_handle: AppHandle, config: DomainsConfig) -> Result<()> {
    store::set(&app_handle, &config).await?;
    Ok(())
}

#[command]
pub async fn logout(app_handle: AppHandle) -> Result<()> {
    store::clear(&app_handle)?;
    Ok(())
}
