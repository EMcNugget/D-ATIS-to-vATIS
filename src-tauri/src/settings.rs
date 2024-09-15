use crate::assets::{get_file, response, set_file};
use crate::consts::SETTINGS_PATH;
use crate::structs::{Response, Settings};
use log::info;
use serde_json::{json, Map, Value};
use std::path::{Path, PathBuf};

pub fn check_settings_file() -> Response {
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

    if Path::new(&SETTINGS_PATH).exists() {
        let settings = match get_file::<Value>(&SETTINGS_PATH) {
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
            match set_file(&SETTINGS_PATH, &new_settings) {
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
        std::fs::create_dir_all(PathBuf::from("./config")).unwrap();
        match set_file(&SETTINGS_PATH, &default_settings) {
            Ok(_) => return response("Settings file created successfully", true, None),
            Err(_) => return response("Failed to create settings file", false, None),
        }
    }
}

#[tauri::command]
pub fn write_settings(settings: Settings) -> Response {
    if Path::new(&SETTINGS_PATH).exists() {
        match get_file(&SETTINGS_PATH) {
            Ok(json_value) => {
                if let Ok(existing_settings) = serde_json::from_value::<Settings>(json_value) {
                    if existing_settings == settings {
                        return response("Settings have not changed", true, None);
                    } else {
                        match set_file(&SETTINGS_PATH, &settings) {
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
pub fn read_settings() -> Result<Settings, Response> {
    if !Path::new(&SETTINGS_PATH).exists() {
        return Err(response("Settings file does not exist", false, None));
    }

    match get_file::<Settings>(&SETTINGS_PATH) {
        Ok(json_value) => Ok(json_value),
        Err(err) => Err(response(
            "Failed to read settings",
            false,
            Some(&err.to_string()),
        )),
    }
}
