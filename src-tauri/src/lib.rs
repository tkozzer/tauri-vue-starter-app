// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::env;

mod debug_window;
mod menu;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            // Create and set the menu
            let menu = menu::create_menu(&app_handle).expect("Failed to create menu");
            app.set_menu(menu).expect("Failed to set app menu");

            // Create the debug window
            debug_window::create_debug_window(app).expect("Failed to create debug window");

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
