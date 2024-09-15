use crate::assets::{get_file, set_file};
use crate::consts::FACILITY_CONFIG_PATH;
use crate::contraction::write_contractions;
use crate::settings::read_settings;
use crate::structs::{FindComposite, Response};
use log::{error, info};
use serde::Serialize;
use serde_json::{self, Value};
use sysinfo::System;
use tauri::{AppHandle, Manager};

fn get_atis_type(atis_type: &str) -> &'static str {
    return match atis_type {
        "dep" => "Departure",
        "arr" => "Arrival",
        "combined" => "Combined",
        _ => "Combined",
    };
}

fn find_composite(
    data: &Value,
    profile: &str,
    facility: &str,
    atis_type: Option<&str>,
) -> Option<FindComposite> {
    let atis_type_str = get_atis_type(atis_type.unwrap_or("unknown"));

    let profiles = data.get("profiles").and_then(|p| p.as_array()).unwrap();

    if profile == "No Profile" {
        for (profile_index, profile) in profiles.iter().enumerate() {
            if let Some(composites) = profile.get("composites").and_then(|c| c.as_array()) {
                for (composite_index, composite) in composites.iter().enumerate() {
                    if let Some(identifier) = composite.get("identifier").and_then(|id| id.as_str())
                    {
                        if identifier == facility {
                            if let Some(atis) = composite.get("atisType").and_then(|a| a.as_str()) {
                                if atis == atis_type_str {
                                    return Some(FindComposite {
                                        profile_index,
                                        composite_index,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        for (profile_index, prof) in profiles.iter().enumerate() {
            if let Some(name) = prof.get("name").and_then(|n| n.as_str()) {
                if name == profile {
                    if let Some(composites) = prof.get("composites").and_then(|c| c.as_array()) {
                        for (composite_index, composite) in composites.iter().enumerate() {
                            if let Some(identifier) =
                                composite.get("identifier").and_then(|id| id.as_str())
                            {
                                if identifier == facility {
                                    if let Some(atis) =
                                        composite.get("atisType").and_then(|a| a.as_str())
                                    {
                                        if atis == atis_type_str {
                                            return Some(FindComposite {
                                                profile_index,
                                                composite_index,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                    continue;
                }
            }
        }
    }
    None
}

pub fn write_profile(
    atis_preset: &Value,
    profile: &str,
    facility: &str,
    file_path: &str,
    atis_type: Option<&str>,
) -> Result<(), String> {
    let file_path = format!("{}\\AppConfig.json", file_path);

    let mut data = get_file(&file_path).unwrap();
    let indexes: FindComposite = find_composite(&data, profile, facility, atis_type).unwrap();

    let atis_position =
        &mut data["profiles"][indexes.profile_index]["composites"][indexes.composite_index];

    let presets = &mut atis_position["presets"];

    if let Some(presets_array) = presets.as_array_mut() {
        presets_array.retain(|preset| {
            if let Some(name) = preset["name"].as_str() {
                !name.contains("REAL WORLD")
            } else {
                true
            }
        });
    }

    presets.as_array_mut().unwrap().push(atis_preset.clone());

    let mut existing = atis_position["contractions"]
        .as_array_mut()
        .unwrap_or(&mut Vec::new())
        .to_vec();

    atis_position["contractions"] = write_contractions(
        &mut existing,
        atis_preset.clone(),
        facility,
        &atis_type.unwrap(),
    )
    .unwrap()
    .into();

    set_file(&file_path, &data).unwrap();
    Ok(())
}

#[derive(Serialize)]
pub struct Codes {
    pub key: String,
    pub codes: Vec<String>,
}
#[derive(Serialize)]

pub struct WriteAtis {
    pub alert: Response,
    pub codes: Codes,
}

#[tauri::command]
pub fn write_atis(
    facility: String,
    atis: Value,
    app_handle: AppHandle,
) -> Result<WriteAtis, String> {
    let settings = read_settings().unwrap();
    let atis_array = atis.as_array().unwrap();

    let mut res = WriteAtis {
        alert: Response {
            alert_type: "success".to_string(),
            message: "".to_string(),
        },
        codes: Codes {
            key: facility.clone(),
            codes: Vec::new(),
        },
    };
    let mut codes: Vec<String> = Vec::new();

    let file_path = get_vatis_path(&app_handle);

    for atis_entry in atis_array {
        let result = write_profile(
            &atis_entry["atis"],
            &settings.profile,
            &facility,
            &file_path,
            Some(atis_entry["atis_type"].as_str().unwrap_or("unknown")),
        );

        match result {
            Ok(_) => {
                let data = &format!("Successfully wrote ATIS for {}", &facility);
                codes.push(format!(
                    "{}: {}",
                    get_atis_type(atis_entry["atis_type"].as_str().unwrap_or("unknown")),
                    &atis_entry["atis_code"].as_str().unwrap_or_default()
                ));
                info!("{}", data);
                if res.alert.message != *data {
                    res.alert.message.push_str(data);
                }
            }
            Err(e) => {
                let data = &format!("Error writing ATIS: {}", e);
                error!("{}", data);
                if res.alert.message != *data {
                    res.alert.message.push_str(data);
                    res.alert.alert_type = "error".to_string();
                }
            }
        }
    }

    res.codes.codes = codes;

    Ok(res)
}

#[tauri::command]
pub fn is_vatis_running() -> bool {
    let s = System::new_all();
    let is_running = s.processes().values().any(|p| p.name() == "vATIS.exe");
    is_running
}

pub fn get_vatis_path(app_handle: &AppHandle) -> String {
    let settings = read_settings().unwrap();
    let mut app_data_path = app_handle.path().app_local_data_dir().unwrap();
    app_data_path.pop();
    if settings.custom_path {
        return settings.file_path.clone();
    }
    format!("{}\\vATIS-4.0", app_data_path.to_str().unwrap())
}

#[tauri::command]
pub fn open_vatis(app_handle: AppHandle, custom_path: Option<&str>) -> Result<(), String> {
    let s = System::new_all();
    let is_running = s.processes().values().any(|p| p.name() == "vATIS.exe");

    let mut app_data_path = app_handle.path().app_local_data_dir().unwrap();
    app_data_path.pop();
    let file_path = format!(
        "{}\\vATIS-4.0\\Application\\vATIS.exe",
        app_data_path.to_str().unwrap()
    );

    if !is_running {
        let path = custom_path.unwrap_or(&file_path);
        std::process::Command::new(path).spawn().map_err(|e| {
            error!("Failed to open vATIS: {}", e);
            e.to_string()
        })?;
    }

    Ok(())
}

#[tauri::command]
pub fn get_profiles(app_handle: AppHandle) -> Result<Vec<String>, String> {
    let file_path = format!("{}\\AppConfig.json", get_vatis_path(&app_handle));
    let data = get_file::<Value>(&file_path).unwrap();
    let mut profiles = Vec::new();
    for profile in data["profiles"].as_array().unwrap() {
        profiles.push(profile["name"].as_str().unwrap().to_string());
    }
    Ok(profiles)
}

#[tauri::command]
pub fn get_airports_in_profile(app_handle: AppHandle, profile: &str) -> Vec<String> {
    let file_path = format!("{}\\AppConfig.json", get_vatis_path(&app_handle));
    let data = get_file::<Value>(&file_path).unwrap();
    let mut airports = Vec::new();

    for p in data["profiles"].as_array().unwrap() {
        if p["name"].as_str().unwrap() == profile {
            for composite in p["composites"].as_array().unwrap() {
                let identifier = composite["identifier"].as_str().unwrap().to_string();
                if !airports.contains(&identifier) {
                    airports.push(identifier);
                }
            }
        }
    }

    return airports;
}

#[tauri::command]
pub fn get_facility_config(facility: &str) -> Value {
    let facility_config = get_file::<Value>(&FACILITY_CONFIG_PATH).unwrap();
    return facility_config[facility].clone();
}
