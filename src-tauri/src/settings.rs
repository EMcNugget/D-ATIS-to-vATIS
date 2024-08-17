use crate::structs::{Response, Settings};
use crate::util::{get_resource_json, read_json_file, write_json_file};
use log::{error, info};
use serde_json::{from_value, Map, Value};
use std::path::Path;
use tauri::{AppHandle, Manager};

fn response(res: &str, success: bool, log_msg: Option<&str>) -> Response {
    let log_msg = log_msg.unwrap_or(res);

    if success {
        info!("{}", log_msg);
        return Response {
            alert_type: "success".to_string(),
            message: res.to_string(),
        };
    } else {
        error!("{}", log_msg);
        return Response {
            alert_type: "error".to_string(),
            message: res.to_string(),
        };
    }
}

pub fn check_settings_file(app_handle: &AppHandle) -> Response {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");

    let settings = get_resource_json(app_handle, "default_settings.json").unwrap();

    if Path::new(&file_path).exists() {
        let json_value = read_json_file(file_path.to_str().unwrap()).unwrap();
        let mut json = json_value.as_object().unwrap().clone();

        let mut changed = false;

        let mut new_json: Map<String, Value> = Map::new();
        let default_keys = settings.as_object().unwrap();

        for k in default_keys.keys() {
            if let Some(value) = json.remove(k) {
                new_json.insert(k.to_string(), value);
            } else {
                new_json.insert(k.to_string(), settings[k].clone());
                changed = true;
                info!("Added missing key to settings: {}", k);
            }
        }

        if changed {
            write_json_file(
                file_path.to_str().unwrap(),
                serde_json::to_string_pretty(&new_json).unwrap().as_str(),
            )
            .unwrap();
            return response("Settings file updated successfully", true, None);
        }

        return response("Settings file already exists", true, None);
    }

    let json_string = serde_json::to_string_pretty(&from_value::<Settings>(settings).unwrap());
    std::fs::create_dir_all(app_data_path).unwrap();
    match write_json_file(file_path.to_str().unwrap(), &json_string.unwrap()) {
        Ok(_) => return response("Settings file created successfully", true, None),
        Err(_) => return response("Failed to create settings file", false, None),
    }
}

#[tauri::command]
pub fn write_settings(settings: Settings, app_handle: AppHandle) -> Result<Response, Response> {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");

    if Path::new(&file_path).exists() {
        match read_json_file(&file_path.to_str().unwrap()) {
            Ok(json_value) => {
                if let Ok(existing_settings) = serde_json::from_value::<Settings>(json_value) {
                    if existing_settings == settings {
                        return Ok(response("Settings have not changed", true, None));
                    } else {
                        let json_string = serde_json::to_string_pretty(&settings);
                        write_json_file(file_path.to_str().unwrap(), &json_string.unwrap())
                            .unwrap();
                        return Ok(response("Settings updated successfully", true, None));
                    }
                } else {
                    return Err(response("Failed to parse existing settings", false, None));
                }
            }
            Err(err) => {
                return Err(response(&err.to_string(), false, None));
            }
        }
    } else {
        return Err(response("Settings file does not exist", false, None));
    }
}

#[tauri::command]
pub fn read_settings(app_handle: AppHandle) -> Result<Settings, String> {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");
    if !Path::new(&file_path).exists() {
        return Err("Settings file does not exist".to_string());
    }

    match read_json_file(&file_path.to_str().unwrap()) {
        Ok(json_value) => {
            if let Ok(settings) = serde_json::from_value::<Settings>(json_value) {
                info!("Settings read successfully");
                Ok(settings)
            } else {
                error!("Failed to parse settings");
                Err("Failed to parse settings".to_string())
            }
        }
        Err(err) => {
            error!("{}", err);
            Err(err.to_string())
        }
    }
}
