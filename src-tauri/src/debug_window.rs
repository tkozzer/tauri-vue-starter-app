use std::env;
use tauri::Manager;
use tauri::WebviewWindowBuilder;

pub fn create_debug_window(app: &tauri::App) -> tauri::Result<()> {
    if env::var("TAURI_DEBUG").unwrap_or_else(|_| "0".to_string()) == "1" {
        // Get the main window
        let main_window = app
            .get_webview_window("main")
            .expect("Failed to get main window");

        // Get the position and size of the main window
        let main_position = main_window.outer_position()?;
        let main_size = main_window.outer_size()?;

        // Get the monitor where the main window is located
        let monitor = main_window.current_monitor()?.expect("No monitor found");

        // Get the monitor's usable position and size
        let monitor_position = monitor.position();
        let monitor_size = monitor.size();

        // Set the debug window width to half of the previous value
        let debug_window_width = 400.0;

        // Calculate the position for the debug window (to the right of the main window)
        let debug_x = (main_position.x + main_size.width as i32)
            .min(monitor_position.x + monitor_size.width as i32 - debug_window_width as i32);
        let debug_y = main_position.y;

        let debug_window =
            WebviewWindowBuilder::new(app, "debug", tauri::WebviewUrl::App("debug.html".into()))
                .title("Debug Window")
                .inner_size(debug_window_width, 600.0)
                .build()?;

        // Set the position of the debug window after it's created
        debug_window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
            x: debug_x,
            y: debug_y,
        }))?;

        // Ensure the debug window is on the same monitor as the main window
        debug_window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
            x: debug_x,
            y: debug_y,
        }))?;
    }

    Ok(())
}
