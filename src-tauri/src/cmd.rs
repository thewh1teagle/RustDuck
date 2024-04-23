use eyre::Result;
use std::str::FromStr;
use tauri::{command, AppHandle, Manager, Url, WebviewWindowBuilder};

use crate::{
    config::{self, DomainsConfig},
    store,
};

#[command]
pub async fn login(app_handle: AppHandle) {
    let url = Url::from_str(config::LOGIN_URL).unwrap();
    let url = tauri::WebviewUrl::External(url);
    WebviewWindowBuilder::new(&app_handle, "duckdns", url)
        .visible(true)
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
        store::setup(app_handle, config)?;
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
    store::set(&app_handle, &config)?;
    Ok(())
}

#[command]
pub async fn logout(app_handle: AppHandle) -> Result<()> {
    store::clear(&app_handle)?;
    Ok(())
}
