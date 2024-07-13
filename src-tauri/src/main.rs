// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

#[derive(Serialize, Deserialize)]
struct Settings {
    pub facility: String,
    pub filepath: String,
}

#[tauri::command]
fn write_settings(settings: Settings) -> Result<(), String> {
    let file = File::create("../settings").map_err(|e| e.to_string())?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &settings).map_err(|e| e.to_string())?;
    writer.flush().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn read_settings() -> Result<Settings, String> {
    let file = File::open("../settings").map_err(|e| e.to_string())?;
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![write_settings])
        .invoke_handler(tauri::generate_handler![read_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
