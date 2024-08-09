use crate::structs::Contraction;
use crate::util::get_resource;
use log::{error, info};
use serde_json::Value;
use tauri::AppHandle;

pub fn write_contractions(
    app_handle: &AppHandle,
    existing: &mut Vec<Value>,
    atis: Value,
    airport_code: &str,
) -> Result<Vec<Value>, String> {
    let json = get_resource(app_handle, "custom_contractions.json");

    if let Ok(Value::Object(map)) = json {
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

        info!("Custom contractions updated for {}", airport_code);
        Ok(existing.to_vec())
    } else {
        let e = format!("Failed to parse custom contractions for {}", airport_code);
        error!("{}", e);
        Err(e)
    }
}
