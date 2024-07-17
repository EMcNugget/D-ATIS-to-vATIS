use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Settings {
    pub facility: String,
    pub file_path: String,
    pub save_facility: bool,
}

#[tauri::command(rename_all = "snake_case")]
pub fn write_settings(settings: Settings) -> Result<(), String> {
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

    let json_string = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    let mut file = File::create(file_path).map_err(|e| e.to_string())?;
    file.write_all(json_string.as_bytes())
        .map_err(|e| e.to_string())?;
    file.flush().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn read_settings() -> Result<Settings, String> {
    let file = File::open("../settings.json").map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| e.to_string())
}