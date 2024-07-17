// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod settings;
mod structs;

use settings::{read_settings, write_settings};

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // Placeholder
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![write_settings, read_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
