use eyre::{ContextCompat, Result};
use std::{path::PathBuf, sync::Arc};
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;

use crate::config::{self, DomainsConfig};
pub async fn setup(app_handle: AppHandle, mut config: DomainsConfig) -> Result<()> {
    config.interval_minutes = Some(10); // 10 minutes
    for domain in &mut config.domains {
        domain.enable = Some(false);
    }
    log::debug!("storing {:?}", config);
    set(&app_handle, &config).await?;
    Ok(())
}

pub fn store_path(app_handle: AppHandle) -> Result<PathBuf> {
    Ok(app_handle
        .path()
        .config_dir()?
        .join(config::CONFIG_FILENAME))
}

pub fn get(app_handle: &AppHandle) -> Result<Option<DomainsConfig>> {
    let path = store_path(app_handle.clone())?;
    if path.exists() {
        let content = std::fs::read_to_string(path)?;
        let value: DomainsConfig = serde_json::from_str(&content)?;
        return Ok(Some(value));
    }
    Ok(None)
}

pub async fn set(app_handle: &AppHandle, config: &DomainsConfig) -> Result<()> {
    // Update global state
    let config_state: State<'_, Arc<Mutex<Option<DomainsConfig>>>> = app_handle.state();
    *config_state.lock().await = Some(config.clone());
    let path = store_path(app_handle.clone())?;
    std::fs::create_dir_all(path.parent().context("no parent")?)?;
    let serialized = serde_json::to_string_pretty(config)?;
    log::debug!("writing to {}", path.display());
    std::fs::write(&path, serialized)?;
    Ok(())
}

pub fn clear(app_handle: &AppHandle) -> Result<()> {
    let path = store_path(app_handle.clone())?;
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}
