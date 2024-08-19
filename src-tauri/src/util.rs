use log::error;
use serde_json::Value;
use std::{
    fs::File,
    io::{BufReader, Write},
};
use sysinfo::System;
use tauri::{path::BaseDirectory, AppHandle, Manager};

use crate::settings::read_settings;

pub fn read_json_file(file_path: &str) -> Result<Value, anyhow::Error> {
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);
    let json_value = serde_json::from_reader(reader).map_err(anyhow::Error::from);
    json_value
}

pub fn write_json_file(filename: &str, data: &str) -> Result<(), anyhow::Error> {
    let file = File::create(filename);
    file?.write_all(data.as_bytes())?;
    Ok(())
}

pub fn get_resource_json(app_handle: &AppHandle, file_name: &str) -> Result<Value, anyhow::Error> {
    let resource = app_handle
        .path()
        .resolve(format!("assets/{}", file_name), BaseDirectory::Resource);

    Ok(read_json_file(&resource?.to_str().unwrap())?)
}

pub fn get_resource(app_handle: &AppHandle, file_name: &str) -> Result<File, anyhow::Error> {
    let resource = app_handle
        .path()
        .resolve(format!("assets/{}", file_name), BaseDirectory::Resource);

    let file = File::open(&resource?);

    Ok(file?)
}

#[tauri::command]
pub fn is_vatis_running() -> bool {
    let s = System::new_all();
    let is_running = s.processes().values().any(|p| p.name() == "vATIS.exe");
    is_running
}

pub fn get_vatis_path(app_handle: &AppHandle) -> String {
    let settings = read_settings(app_handle.clone()).unwrap();
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
