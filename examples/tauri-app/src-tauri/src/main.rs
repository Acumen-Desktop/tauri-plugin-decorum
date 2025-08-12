// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_decorum::WebviewWindowExt;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_decorum::init())
        .setup(|app| {
            println!("Setting up application...");
            
            // Get the main window
            let main_window = app.get_webview_window("main").unwrap();
            println!("Got main window: {}", main_window.label());
            
            // Create a custom titlebar for the main window
            main_window.create_overlay_titlebar().unwrap();
            println!("Created overlay titlebar");

            // On macOS, set traffic light positions
            #[cfg(target_os = "macos")]
            {
                println!("Setting traffic light positions...");
                main_window.set_traffic_lights_inset(20.0, 15.0).unwrap();
                println!("Traffic light positions set");
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
