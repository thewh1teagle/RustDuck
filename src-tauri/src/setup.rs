use std::{error::Error, sync::Arc};
use tauri::{App, Manager};
use tokio::sync::Mutex;

use crate::{cmd, duckdns, store, tray::create_tray};

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let app_handle = app.app_handle();
    let app_handle1 = app_handle.clone();
    create_tray(app_handle)?;

    app_handle.listen("single-instance", move |_event| {
        cmd::open_main_window(&app_handle1).unwrap();
    });
    let config = store::get(app_handle)?;
    let app_clone = app_handle.app_handle();
    let config = Arc::new(Mutex::new(config));
    app_clone.manage(config.clone());
    tauri::async_runtime::spawn(async {
        duckdns::updater_task(config).await;
    });

    Ok(())
}
