use crate::structs::{Response, Settings};
use crate::util::{read_json_file, response, write_json_file};
use log::info;
use serde_json::{from_value, json, Map};
use std::path::Path;
use tauri::{AppHandle, Manager};

pub fn check_settings_file(app_handle: &AppHandle) -> Response {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");
    let default_settings = json!({
        "facility": "",
        "file_path": "",
        "custom_path": false,
        "save_facility": false,
        "open_vatis_on_fetch": false,
        "check_updates": false,
        "suppress_notification": false,
        "check_updates_freq": false,
        "fetch_for_profile": false,
        "update_time": 60,
        "profile": "",
        "theme": "system"
    });

    if Path::new(&file_path).exists() {
        let settings = match read_json_file(file_path.to_str().unwrap()) {
            Ok(json) => json,
            Err(err) => {
                return response(
                    "Failed to read settings file",
                    false,
                    Some(&err.to_string()),
                );
            }
        };

        let json = settings.as_object().unwrap().clone();
        let mut new_settings = Map::new();

        let mut changed = false;

        let default_keys = default_settings.as_object().unwrap();

        for k in default_keys.keys() {
            if json.contains_key(k) {
                new_settings.insert(k.to_string(), json[k].clone());
            } else {
                new_settings.insert(k.to_string(), default_keys[k].clone());
                changed = true;
                info!("Added missing key: {}", k);
            }
        }

        if changed {
            match write_json_file(
                file_path.to_str().unwrap(),
                serde_json::to_string_pretty(&new_settings)
                    .unwrap()
                    .as_str(),
            ) {
                Ok(_) => return response("Settings file updated successfully", true, None),
                Err(err) => response(
                    "Failed to update settings file",
                    false,
                    Some(&err.to_string()),
                ),
            };
        }

        return response("Settings file exists and is up to date", true, None);
    } else {
        let json_string =
            serde_json::to_string_pretty(&from_value::<Settings>(default_settings).unwrap());
        std::fs::create_dir_all(app_data_path).unwrap();
        match write_json_file(file_path.to_str().unwrap(), &json_string.unwrap()) {
            Ok(_) => return response("Settings file created successfully", true, None),
            Err(_) => return response("Failed to create settings file", false, None),
        }
    }
}

#[tauri::command]
pub fn write_settings(settings: Settings, app_handle: AppHandle) -> Response {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");

    if Path::new(&file_path).exists() {
        match read_json_file(&file_path.to_str().unwrap()) {
            Ok(json_value) => {
                if let Ok(existing_settings) = serde_json::from_value::<Settings>(json_value) {
                    if existing_settings == settings {
                        return response("Settings have not changed", true, None);
                    } else {
                        let json_string = serde_json::to_string_pretty(&settings);
                        match write_json_file(file_path.to_str().unwrap(), &json_string.unwrap()) {
                            Ok(_) => return response("Settings updated successfully", true, None),
                            Err(err) => {
                                return response(
                                    "Failed to write settings",
                                    false,
                                    Some(&err.to_string()),
                                );
                            }
                        }
                    }
                } else {
                    return response("Failed to parse existing settings", false, None);
                }
            }
            Err(err) => {
                return response(
                    "Failed to read existing settings",
                    false,
                    Some(&err.to_string()),
                );
            }
        }
    } else {
        return response("Settings file does not exist", false, None);
    }
}

#[tauri::command]
pub fn read_settings(app_handle: AppHandle) -> Result<Settings, Response> {
    let app_data_path = app_handle.path().app_data_dir().unwrap();
    let file_path = app_data_path.join("settings.json");
    if !Path::new(&file_path).exists() {
        return Err(response("Settings file does not exist", false, None));
    }

    match read_json_file(&file_path.to_str().unwrap()) {
        Ok(json_value) => Ok(serde_json::from_value::<Settings>(json_value).unwrap()),
        Err(err) => Err(response(
            "Failed to read settings",
            false,
            Some(&err.to_string()),
        )),
    }
}
