#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
mod chrome;
mod commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        // This is required to get tray-relative positions to work
        // .on_system_tray_event(|app, event| {
        //     tauri_plugin_positioner::on_tray_event(app, &event);
        // })
        .invoke_handler(tauri::generate_handler![
            commands::get_cookies
        ])
        .setup(|app|{
              let path = app.path_resolver(); 
              println!("resource_dir:{:?}",path.resource_dir());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
