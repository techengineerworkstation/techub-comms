// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Techub Comms.", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // KDE Plasma Wayland compatibility
            #[cfg(target_os = "linux")]
            {
                // Set WAYLAND_DISPLAY for proper Wayland support
                if std::env::var("WAYLAND_DISPLAY").is_ok() {
                    // Running on Wayland
                    std::env::set_var("GDK_BACKEND", "wayland");
                    std::env::set_var("QT_QPA_PLATFORM", "wayland");
                }

                // Set app ID for KDE Plasma taskbar grouping
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_title("Techub Comms");
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Techub Comms Desktop");
}
