// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod settings;
mod structs;

use app::write_profile;
use serde_json::Value;
use settings::{read_settings, write_settings};
use tauri::{Event, Listener};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.listen("atis", |event: Event| {
                let settings = read_settings().unwrap();
                let payload: Value = serde_json::from_str(&event.payload()).unwrap();
                let atis_array = payload["atis"].as_array().unwrap();
                let facility = payload["facility"].as_str().unwrap();

                for (index, atis_entry) in atis_array.iter().enumerate() {
                    let atis = &atis_entry["atis"];
                    let atis_type = atis_entry["atis_type"].as_str().unwrap_or("unknown");

                    let result = write_profile(
                        atis,
                        &settings.profile,
                        facility,
                        format!("{}\\AppConfig.json", &settings.profile).as_str(),
                        Some(atis_type),
                    );

                    match result {
                        Ok(_) => println!("Successfully wrote profile for ATIS Index: {}", index),
                        Err(e) => {
                            eprintln!("Error writing profile for ATIS Index: {}: {}", index, e)
                        }
                    }
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![write_settings, read_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
