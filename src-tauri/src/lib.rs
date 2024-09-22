// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::menu::{AboutMetadata, Menu, PredefinedMenuItem, Submenu};
use tauri::WebviewWindowBuilder;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            // Create the about menu item
            let about_metadata = AboutMetadata::default();
            let about_item = PredefinedMenuItem::about(app_handle, None, Some(about_metadata))
                .expect("Failed to create About menu item");

            // Create a quit menu item
            let quit_item = PredefinedMenuItem::quit(app_handle, None)
                .expect("Failed to create Quit menu item");

            // Create a submenu with the about and quit items
            let submenu =
                Submenu::new(app_handle, "App", true).expect("Failed to create App submenu");

            submenu
                .append(&about_item)
                .expect("Failed to append About item");

            submenu
                .append(
                    &PredefinedMenuItem::separator(app_handle).expect("Failed to create separator"),
                )
                .expect("Failed to append separator");

            submenu
                .append(&quit_item)
                .expect("Failed to append Quit item");

            // Create the main menu and add the submenu
            let menu = Menu::new(app_handle).expect("Failed to create menu");

            menu.append(&submenu).expect("Failed to append submenu");

            // Set the menu for the application
            app.set_menu(menu).expect("Failed to set app menu");

            // Create the debug window
            WebviewWindowBuilder::new(app, "debug", tauri::WebviewUrl::App("debug.html".into()))
                .title("Debug Window")
                .build()
                .expect("Failed to create debug window");

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
