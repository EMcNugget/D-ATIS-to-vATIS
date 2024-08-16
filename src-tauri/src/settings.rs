use crate::structs::{Response, Settings};
use crate::util::{get_resource_json, get_vatis_path, read_json_file, write_json_file};
use log::{error, info};
use serde_json::{from_value, Map, Value};
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

pub fn check_settings_file(app_handle: &AppHandle) -> Result<Response, String> {
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
            return response("Settings file validated", true);
        }

        return response("Settings file already exists", true);
    }

    let json_string = serde_json::to_string_pretty(
        &from_value::<Settings>(settings)
            .map_err(|e| e.to_string())
            .unwrap(),
    )
    .map_err(|e| e.to_string())?;
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

#[tauri::command]
pub fn get_profiles(app_handle: AppHandle) -> Result<Vec<String>, String> {
    let json =
        read_json_file(&format!("{}\\AppConfig.json", &get_vatis_path(&app_handle))).unwrap();

    let mut profiles = Vec::new();

    profiles.push("No Profile".to_string());

    for profile in json["profiles"].as_array().unwrap() {
        profiles.push(profile["name"].as_str().unwrap().to_string());
    }

    match profiles.len() {
        0 => Err("No profiles found".to_string()),
        _ => Ok(profiles),
    }
}

#[tauri::command]

pub fn get_airports_in_profile(
    app_handle: AppHandle,
    profile: usize,
) -> Result<Vec<String>, String> {
    let json =
        read_json_file(&format!("{}\\AppConfig.json", &get_vatis_path(&app_handle))).unwrap();

    let mut airports = Vec::new();

    while let Some(composite) = json["profiles"].get(profile) {
        if !airports.contains(&composite["identifier"].as_str().unwrap().to_string()) {
            airports.push(composite["identifier"].to_string());
        }
    }

    match airports.len() {
        0 => Err("No airports found".to_string()),
        _ => Ok(airports),
    }
}
