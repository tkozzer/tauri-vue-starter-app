use tauri::menu::{AboutMetadata, Menu, PredefinedMenuItem, Submenu};
use tauri::Runtime;

pub fn create_menu<R: Runtime>(app_handle: &tauri::AppHandle<R>) -> tauri::Result<Menu<R>> {
    // Create the about menu item
    let about_metadata = AboutMetadata::default();
    let about_item = PredefinedMenuItem::about(app_handle, None, Some(about_metadata))?;

    // Create a quit menu item
    let quit_item = PredefinedMenuItem::quit(app_handle, None)?;

    // Create a submenu with the about and quit items
    let submenu = Submenu::new(app_handle, "App", true)?;

    submenu.append(&about_item)?;
    submenu.append(&PredefinedMenuItem::separator(app_handle)?)?;
    submenu.append(&quit_item)?;

    // Create the main menu and add the submenu
    let menu = Menu::new(app_handle)?;
    menu.append(&submenu)?;

    Ok(menu)
}
