// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Settings {
    pub facility: String,
    pub file_path: String,
}

#[tauri::command(rename_all = "snake_case")]
fn write_settings(settings: Settings) -> Result<(), String> {
    let file_path = "../settings.json";

    if Path::new(file_path).exists() {
        let file = File::open(file_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        if let Ok(existing_settings) = serde_json::from_reader::<_, Settings>(reader) {
            if existing_settings == settings {
                return Ok(());
            }
        }
    }
    let file = File::create(file_path).map_err(|e| e.to_string())?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &settings).map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn read_settings() -> Result<Settings, String> {
    let file = File::open("../settings.json").map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![write_settings, read_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
