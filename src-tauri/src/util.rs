use serde_json::Value;
use std::{
    fs::File,
    io::{BufReader, Write},
};
use tauri::{path::BaseDirectory, AppHandle, Manager};

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
