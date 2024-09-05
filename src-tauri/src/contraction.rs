use crate::{
    structs::{Contraction, Response},
    util::{get_resource_json, response, write_resource_json},
};
use log::{error, info};
use serde_json::Value;
use tauri::AppHandle;

pub fn get_intro_contraction(
    app_handle: &AppHandle,
    airport_code: &str,
    atis_type: &str,
) -> Vec<Contraction> {
    let json =
        get_resource_json(app_handle, "custom_contractions.json").unwrap()["airports"].clone();

    let atis_type_spoken = match atis_type {
        "dep" => "DEPARTURE",
        "arr" => "ARRIVAL",
        _ => "",
    };

    let string = format!("{} {} INFO", &airport_code[1..], atis_type);
    let spoken = format!(
        "{} AIRPORT {} INFORMATION",
        json[airport_code].to_string(),
        atis_type_spoken
    );

    vec![Contraction { string, spoken }]
}

pub fn write_contractions(
    app_handle: &AppHandle,
    existing: &mut Vec<Value>,
    atis: Value,
    airport_code: &str,
    atis_type: &str,
) -> Result<Vec<Value>, anyhow::Error> {
    let json = get_resource_json(app_handle, "custom_contractions.json").unwrap()
        ["notam_contractions"]
        .clone();

    if let Value::Object(map) = json {
        let new_contractions: Vec<Contraction> = map
            .into_iter()
            .filter_map(|(key, value)| {
                if atis["notams"]
                    .as_str()
                    .unwrap()
                    .contains(format!(" {} ", &key).as_str())
                    || atis["airportConditions"]
                        .as_str()
                        .unwrap()
                        .contains(format!(" {} ", &key).as_str())
                {
                    if existing
                        .iter()
                        .any(|c| c["string"].as_str().unwrap() == key)
                    {
                        return None;
                    } else {
                        Some(Contraction {
                            string: key,
                            spoken: value.as_str().unwrap().to_string(),
                        })
                    }
                } else {
                    None
                }
            })
            .collect();

        existing.extend(
            new_contractions
                .iter()
                .map(|c| serde_json::to_value(c).expect("Failed to serialize custom contractions")),
        );

        let intro_contraction = get_intro_contraction(app_handle, airport_code, atis_type);

        if !existing.contains(&serde_json::to_value(intro_contraction.first().unwrap()).unwrap()) {
            existing.push(serde_json::to_value(intro_contraction.first().unwrap()).unwrap());
        }

        info!("Custom contractions updated for {}", airport_code);
        Ok(existing.to_vec())
    } else {
        let e = format!("Failed to parse custom contractions for {}", airport_code);
        error!("{}", e);
        Err(anyhow::Error::msg(e.to_string()))
    }
}

#[tauri::command]
pub fn create_new_contraction(contraction: Contraction, app_handle: AppHandle) -> Response {
    let mut contractions = get_resource_json(&app_handle, "custom_contractions.json").unwrap()
        ["notam_contractions"]
        .clone();

    contractions[contraction.string.clone()] = Value::String(contraction.spoken.clone());

    match write_resource_json(&app_handle, "custom_contractions.json", &contractions) {
        Ok(_) => response("Custom contraction edit successfully", true, None),
        Err(err) => response(
            "Failed to add custom contraction",
            false,
            Some(&err.to_string()),
        ),
    }
}
