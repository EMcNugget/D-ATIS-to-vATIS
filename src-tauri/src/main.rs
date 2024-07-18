// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod settings;
mod structs;

use settings::{read_settings, write_settings};
use app::write_profile;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![write_settings, read_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
