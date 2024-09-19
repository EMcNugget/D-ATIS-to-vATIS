use crate::structs::Response;
use anyhow::Result;
use log::{error, info};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use crate::consts::{CONTRACTION_PATH, FACILITY_CONFIG_PATH, RUNWAYS_PATH};

pub fn set_file<T: Serialize>(path: &str, data: &T) -> Result<()> {
    let mut file = File::create(PathBuf::from(path))?;
    file.write_all(serde_json::to_string_pretty(data)?.as_bytes())?;
    Ok(())
}

pub fn get_file<T: DeserializeOwned>(path: &str) -> Result<T> {
    let file = File::open(PathBuf::from(path))?;
    let data = serde_json::from_reader(BufReader::new(file))?;
    Ok(data)
}


pub fn response(res: &str, success: bool, log_msg: Option<&str>) -> Response {
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



#[tauri::command]
pub fn set_contractions(contractions: Value) -> Response {
    match set_file(&CONTRACTION_PATH, &contractions) {
        Ok(_) => response("Custom contractions updated successfully", true, None),
        Err(e) => response(
            "Failed to update custom contractions",
            false,
            Some(&e.to_string()),
        ),
    }
}

#[tauri::command]
pub fn get_facility_config(facility: &str) -> Value {
    let facility_config = get_file::<Value>(&FACILITY_CONFIG_PATH).unwrap();
    return facility_config[facility].clone();
}


#[tauri::command]
pub fn get_contractions() -> Value {
    get_file(&CONTRACTION_PATH).unwrap()
}

#[tauri::command]
pub fn get_runways() -> Value {
    get_file::<Value>(&RUNWAYS_PATH).unwrap()
}
