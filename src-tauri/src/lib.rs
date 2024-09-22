// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::env;
use tauri::menu::{AboutMetadata, Menu, PredefinedMenuItem, Submenu};
use tauri::Manager;
use tauri::WebviewWindowBuilder; // Add this import at the top of the file

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

            // Create the debug window only if the TAURI_DEBUG environment variable is set to "1"
            if env::var("TAURI_DEBUG").unwrap_or_else(|_| "0".to_string()) == "1" {
                // Get the main window
                let main_window = app
                    .get_webview_window("main")
                    .expect("Failed to get main window");

                // Get the position and size of the main window
                let main_position = main_window
                    .outer_position()
                    .expect("Failed to get main window position");
                let main_size = main_window
                    .outer_size()
                    .expect("Failed to get main window size");

                // Get the monitor where the main window is located
                let monitor = main_window
                    .current_monitor()
                    .expect("Failed to get current monitor")
                    .expect("No monitor found");

                // Get the monitor's usable position and size
                let monitor_position = monitor.position();
                let monitor_size = monitor.size();

                // Set the debug window width to half of the previous value
                let debug_window_width = 400.0; // Half of the previous 800.0

                // Calculate the position for the debug window (to the right of the main window)
                let debug_x = (main_position.x + main_size.width as i32).min(
                    monitor_position.x + monitor_size.width as i32 - debug_window_width as i32,
                ); // Ensure it's within the monitor
                let debug_y = main_position.y; // Same vertical position as main window

                let debug_window = WebviewWindowBuilder::new(
                    app,
                    "debug",
                    tauri::WebviewUrl::App("debug.html".into()),
                )
                .title("Debug Window")
                .inner_size(debug_window_width, 600.0)
                .build()
                .expect("Failed to create debug window");

                // Set the position of the debug window after it's created
                debug_window
                    .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                        x: debug_x,
                        y: debug_y,
                    }))
                    .expect("Failed to set debug window position");

                // Ensure the debug window is on the same monitor as the main window
                debug_window
                    .set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                        x: debug_x,
                        y: debug_y,
                    }))
                    .expect("Failed to set debug window position");
            }

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
