use crate::util::{read_json_file, write_json_file};
use crate::structs::Settings;
use log::{error, info};
use std::path::Path;
use tauri::{AppHandle, Manager};

#[tauri::command(rename_all = "snake_case")]
pub fn write_settings(settings: Settings, app_handle: AppHandle) -> Result<(), String> {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");
    std::fs::create_dir_all(&app_data_path).map_err(|e| {
        let err_msg = e.to_string();
        error!(
            "Failed to create directory {}: {}",
            app_data_path.display(),
            err_msg
        );
        err_msg
    })?;

    if Path::new(&file_path).exists() {
        match read_json_file(&file_path.to_str().unwrap()) {
            Ok(json_value) => {
                if let Ok(existing_settings) = serde_json::from_value::<Settings>(json_value) {
                    if existing_settings == settings {
                        return Ok(());
                    }
                } else {
                    error!("{}", "Failed to parse existing settings.");
                }
            }
            Err(err) => {
                error!("{}", err);
            }
        }
    }

    let json_string = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    write_json_file(file_path.to_str().unwrap(), &json_string)?;
    info!("Settings written to {}", file_path.display());
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn read_settings(app_handle: AppHandle) -> Result<Settings, String> {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");
    if !Path::new(&file_path).exists() {
        return Err("Settings file does not exist.".to_string());
    }

    match read_json_file(&file_path.to_str().unwrap()) {
        Ok(json_value) => {
            if let Ok(settings) = serde_json::from_value::<Settings>(json_value) {
                Ok(settings)
            } else {
                error!("Failed to parse settings.");
                Err("Failed to parse settings.".to_string())
            }
        }
        Err(err) => {
            error!("{}", err);
            Err(err)
        }
    }
    info!("Settings read from {}", file_path.display());
}
