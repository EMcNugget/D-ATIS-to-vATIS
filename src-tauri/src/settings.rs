use crate::structs::Settings;
use crate::util::{read_json_file, write_json_file};
use log::{error, info};
use std::path::Path;
use tauri::{AppHandle, Manager};

fn response(res: &str, success: bool) -> Result<String, String> {
    if success {
        info!("{}", res);
        Ok(res.to_string())
    } else {
        error!("{}", res);
        Err(res.to_string())
    }
}

#[tauri::command]
pub fn write_settings(settings: Settings, app_handle: AppHandle) -> Result<String, String> {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");

    if Path::new(&file_path).exists() {
        match read_json_file(&file_path.to_str().unwrap()) {
            Ok(json_value) => {
                if let Ok(existing_settings) = serde_json::from_value::<Settings>(json_value) {
                    if existing_settings == settings {
                        return response("Settings have not changed.", true);
                    } else {
                        let json_string =
                            serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
                        write_json_file(file_path.to_str().unwrap(), &json_string)?;
                        return response("Settings updated successfully.", true);
                    }
                } else {
                    return response("Failed to parse existing settings.", false);
                }
            }
            Err(err) => {
                return response(&err, false);
            }
        }
    } else {
        match std::fs::create_dir_all(&app_data_path) {
            Ok(_) => info!("Directory created successfully."),
            Err(e) => {
                error!("{}", e);
                return response(
                    format!("Failed to create directory: {}", app_data_path.display()).as_str(),
                    false,
                );
            }
        }

        let json_string = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
        write_json_file(file_path.to_str().unwrap(), &json_string)?;
        return response("Settings written successfully.", true);
    }
}

#[tauri::command]
pub fn read_settings(app_handle: AppHandle) -> Result<Settings, String> {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");
    if !Path::new(&file_path).exists() {
        return Err("Settings file does not exist.".to_string());
    }

    match read_json_file(&file_path.to_str().unwrap()) {
        Ok(json_value) => {
            if let Ok(settings) = serde_json::from_value::<Settings>(json_value) {
                info!("Settings read successfully.");
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
}
