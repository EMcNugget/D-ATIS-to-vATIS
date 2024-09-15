use crate::structs::Response;
use anyhow::Result;
use log::{error, info};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

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
