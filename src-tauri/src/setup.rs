use clap::Parser;
use std::{error::Error, sync::Arc};
use tauri::{App, Manager};
use tokio::sync::Mutex;

use crate::{cmd, duckdns, store, tray::create_tray, Args};

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    log::debug!("args: {:?}", args);
    if !args.minimized {
        cmd::open_main_window(app.app_handle()).unwrap();
    }

    let app_handle = app.app_handle();
    let app_handle1 = app_handle.clone();
    create_tray(app_handle)?;
    let app_handle = app.handle();
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
