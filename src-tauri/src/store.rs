use eyre::{ContextCompat, Result};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

use crate::config::{self, DomainsConfig};

pub fn setup(app_handle: AppHandle, mut config: DomainsConfig) -> Result<()> {
    config.interval_minutes = Some(10); // 10 minutes
    for mut domain in config.clone().domains {
        domain.enable = Some(true);
    }
    set(&app_handle, &config)?;
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

pub fn set(app_handle: &AppHandle, config: &DomainsConfig) -> Result<()> {
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
