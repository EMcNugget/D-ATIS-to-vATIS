use crate::structs::{Response, Settings};
use crate::util::{read_json_file, write_json_file};
use log::{error, info};
use std::path::Path;
use tauri::{AppHandle, Manager};

fn response(res: &str, success: bool) -> Result<Response, String> {
    if success {
        info!("{}", res);
        Ok(Response {
            alert_type: "success".to_string(),
            message: res.to_string(),
        })
    } else {
        error!("{}", res);
        return Ok(Response {
            alert_type: "error".to_string(),
            message: res.to_string(),
        });
    }
}

pub fn create_settings_file(app_handle: &AppHandle) -> Result<Response, String> {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");

    if Path::new(&file_path).exists() {
        return response("Settings file already exists", true);
    }

    let settings = Settings {
        facility: "".to_string(),
        file_path: "".to_string(),
        custom_path: false,
        save_facility: false,
        profile: "".to_string(),
        theme: "system".to_string(),
    };

    let json_string = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(app_data_path).map_err(|e| e.to_string())?;
    write_json_file(file_path.to_str().unwrap(), &json_string)?;

    response("Settings file created successfully", true)
}

#[tauri::command]
pub fn write_settings(settings: Settings, app_handle: AppHandle) -> Result<Response, String> {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");

    if Path::new(&file_path).exists() {
        match read_json_file(&file_path.to_str().unwrap()) {
            Ok(json_value) => {
                if let Ok(existing_settings) = serde_json::from_value::<Settings>(json_value) {
                    if existing_settings == settings {
                        return response("Settings have not changed", true);
                    } else {
                        let json_string = serde_json::to_string_pretty(&settings)
                            .map_err(|e| response(e.to_string().as_str(), false));
                        write_json_file(file_path.to_str().unwrap(), &json_string.unwrap())?;
                        return response("Settings updated successfully", true);
                    }
                } else {
                    return response("Failed to parse existing settings", false);
                }
            }
            Err(err) => {
                return response(&err, false);
            }
        }
    } else {
        return response("Settings file does not exist, restart the app", false);
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
            Err(err)
        }
    }
}

pub fn check_settings(app_handle: &AppHandle) -> Result<(), String> {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");

    let mut json = read_json_file(file_path.to_str().unwrap()).unwrap();

    let settings = Settings {
        facility: "".to_string(),
        file_path: "".to_string(),
        custom_path: false,
        save_facility: false,
        profile: "".to_string(),
        theme: "system".to_string(),
    };

    let keys = vec![
        "facility",
        "file_path",
        "custom_path",
        "save_facility",
        "profile",
        "theme",
    ];

    for key in keys {
        if json[key].is_null() {
            json[key] = serde_json::to_value(&settings).unwrap()[key].clone();
        } else {
            continue;
        }
    }

    write_json_file(
        file_path.to_str().unwrap(),
        serde_json::to_string_pretty(&json).unwrap().as_str(),
    )
    .unwrap();

    Ok(())
}
