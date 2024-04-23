use std::sync::atomic::AtomicBool;

use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::{ClickType, TrayIconBuilder},
    AppHandle,
};

use crate::cmd;

pub static EXIT_FLAG: AtomicBool = AtomicBool::new(false);

pub fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit RustDuck", true, None::<&str>)?;
    let app_clone = app.clone();
    let app_clone1 = app.clone();
    let menu = Menu::with_items(&app.clone(), &[&quit_i])?;
    let _ = TrayIconBuilder::with_id("tray")
        .tooltip("RustDuck")
        .icon(app.clone().default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(move |app: &AppHandle, event: MenuEvent| {
            if event.id.as_ref() == "quit" {
                EXIT_FLAG.store(true, std::sync::atomic::Ordering::Relaxed);
                app.exit(0);
            }
        })
        .on_tray_icon_event(move |_tray, event| {
            let app_clone = app_clone.clone();
            if event.click_type == ClickType::Left {
                // Focus window
                cmd::open_main_window(&app_clone).unwrap();
            }
        })
        .build(&app_clone1);

    Ok(())
}
