// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod settings;
mod structs;

use app::write_profile;
use settings::{read_settings, write_settings};
use tauri::Event;
use tauri::Listener;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.listen("atis", |event: Event| {
                let settings = read_settings().unwrap();
                let payload = serde_json::json!(event.payload());
                let atis = &payload["atis"]["atis"];
                let facility = &payload["facility"].to_string();
                let atis_type = &payload["atis"]["atis_type"].to_string();

                let _ = write_profile(
                    &atis,
                    &settings.profile,
                    &facility,
                    &settings.file_path,
                    Some(atis_type),
                );
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![write_settings, read_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
