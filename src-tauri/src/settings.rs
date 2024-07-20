use crate::structs::Settings;
use std::{
    fs::File,
    io::{BufReader, Write},
    path::Path,
};
use tauri::{AppHandle, Manager};

#[tauri::command(rename_all = "snake_case")]
pub fn write_settings(settings: Settings, app_handle: AppHandle) -> Result<(), String> {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");
    println!("{:?}", file_path);
    std::fs::create_dir_all(&app_data_path).map_err(|e| e.to_string())?;

    if Path::new(&file_path).exists() {
        let file = File::open(&file_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        if let Ok(existing_settings) = serde_json::from_reader::<_, Settings>(reader) {
            if existing_settings == settings {
                return Ok(());
            }
        } else {
            return Err("Failed to read existing settings.".to_string());
        }
    }
    let json_string = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
    file.write_all(json_string.as_bytes())
        .map_err(|e| e.to_string())?;
    file.flush().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn read_settings(app_handle: AppHandle) -> Result<Settings, String> {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");
    if !Path::new(&file_path).exists() {
        return Err("Settings file does not exist.".to_string());
    }

    let file = File::open(&file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| e.to_string())
}
