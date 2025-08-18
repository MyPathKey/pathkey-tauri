#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Result};

fn main() -> Result<()> {
    tauri::Builder::default()
        // .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Set the main window title at runtime (optional).
            if let Some(window) = app.get_webview_window("main") {
                window.set_title("PathKey Console").ok();
            }
            Ok(())
        })
        .run(tauri::generate_context!())?;

    Ok(())
}