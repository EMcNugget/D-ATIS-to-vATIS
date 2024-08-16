use crate::contraction::write_contractions;
use crate::settings::read_settings;
use crate::structs::{Alert, FindComposite};
use crate::util::{read_json_file, write_json_file};
use log::{error, info};
use serde_json::{self, Value};
use tauri::{AppHandle, Manager};

// Nested hell, needs a refactor
fn find_composite(
    data: &Value,
    profile: &str,
    facility: &str,
    atis_type: Option<&str>,
) -> Option<FindComposite> {
    fn find_atis_type(atis_type: Option<&str>) -> &str {
        match atis_type {
            Some("dep") => "Departure",
            Some("arr") => "Arrival",
            Some("combined") => "Combined",
            _ => "Combined",
        }
    }

    let atis_type_str = find_atis_type(atis_type);

    if let Some(profiles) = data.get("profiles").and_then(|p| p.as_array()) {
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
    }
    None
}

pub fn write_profile(
    app_handle: &AppHandle,
    atis_preset: &Value,
    profile: &str,
    facility: &str,
    file_path: &str,
    atis_type: Option<&str>,
) -> Result<(), String> {
    let mut data: Value =
        serde_json::from_str(&read_json_file(file_path).unwrap().to_string()).unwrap();
    let indexes: FindComposite = match find_composite(&data, profile, facility, atis_type) {
        Some(result) => result,
        None => {
            let err = format!("Could not find profile {} for{}", profile, facility);
            error!("{}", err);
            return Err(err);
        }
    };

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

    atis_position["contractions"] =
        write_contractions(app_handle, &mut existing, atis_preset.clone(), facility)
            .unwrap()
            .into();

    write_json_file(file_path, &data.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn write_atis(facility: String, atis: Value, app_handle: AppHandle) -> Result<Alert, String> {
    let settings = read_settings(app_handle.clone()).unwrap();
    let atis_array = atis.as_array().unwrap();

    let mut message = Alert {
        alert_type: "success".to_string(),
        message: String::new(),
    };

    let file_path: String;

    if !settings.custom_path {
        let mut app_data_path = app_handle.path().app_local_data_dir().unwrap();
        app_data_path.pop();
        file_path = format!(
            "{}\\vATIS-4.0\\AppConfig.json",
            app_data_path.to_str().unwrap()
        );
    } else {
        file_path = format!("{}\\AppConfig.json", &settings.file_path);
    }

    for atis_entry in atis_array {
        let result = write_profile(
            &app_handle,
            &atis_entry["atis"],
            &settings.profile,
            &facility,
            &file_path,
            Some(atis_entry["atis_type"].as_str().unwrap_or("unknown")),
        );

        match result {
            Ok(_) => {
                let data = &format!("Successfully wrote ATIS for {}", &facility);
                info!("{}", data);
                if message.message == *data {
                } else {
                    message.message.push_str(data);
                }
            }
            Err(e) => {
                let data = &format!("Error writing ATIS: {}", e);
                error!("{}", data);
                if message.message == *data {
                } else {
                    message.message.push_str(data);
                }
                message.alert_type = "error".to_string();
            }
        }
    }

    Ok(message)
}
