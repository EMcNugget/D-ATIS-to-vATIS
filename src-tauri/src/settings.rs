use crate::structs::Settings;
use std::{
    env,
    fs::File,
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

fn get_app_data_path() -> PathBuf {
    env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("d-atis-to-v-atis")
}

#[tauri::command(rename_all = "snake_case")]
pub fn write_settings(settings: Settings) -> Result<(), String> {
    let app_data_path = get_app_data_path();
    let file_path = app_data_path.join("settings.json");

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
    } else {
        return Err("Settings file does not exist.".to_string());
    }
    std::fs::create_dir_all(&app_data_path).map_err(|e| e.to_string())?;

    let json_string = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
    file.write_all(json_string.as_bytes())
        .map_err(|e| e.to_string())?;
    file.flush().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn read_settings() -> Result<Settings, String> {
    let app_data_path = get_app_data_path();
    let file_path = app_data_path.join("settings.json");
    if !Path::new(&file_path).exists() {
        return Err("Settings file does not exist.".to_string());
    }

    let file = File::open(&file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| e.to_string())
}
