use crate::{structs::Contraction, util::get_resource};
use serde_json::Value;
use tauri::AppHandle;

fn get_contractions(app_handle: &AppHandle) -> Result<Vec<Contraction>, String> {
    let json = get_resource(app_handle, "custom_contractions.json");

    if let Ok(Value::Object(map)) = json {
        let contractions: Vec<Contraction> = map
            .into_iter()
            .map(|(key, value)| Contraction {
                contraction: key,
                description: value.as_str().unwrap_or("").to_string(),
            })
            .collect();

        Ok(contractions)
    } else {
        Err("Failed to parse custom contractions".to_string())
    }
}
