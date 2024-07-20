use std::{fs::File, io::{BufReader, Write}};
use log::error;
use serde_json::Value;

pub fn read_json_file(file_path: &str) -> Result<Value, String> {
  let file = File::open(file_path).map_err(|e| {
      let err_msg = e.to_string();
      error!("Failed to open file {}: {}", file_path, err_msg);
      err_msg
  })?;

  let reader = BufReader::new(file);
  let json_value = serde_json::from_reader(reader).map_err(|e| {
      let err_msg = e.to_string();
      error!("Failed to read file {}: {}", file_path, err_msg);
      err_msg
  })?;
  Ok(json_value)
}

pub fn write_json_file(filename: &str, data: &str) -> Result<(), String> {
  let mut file = File::create(filename).map_err(|e| {
      let err_msg = e.to_string();
      error!("Failed to create file {}: {}", filename, err_msg);
      err_msg
  })?;
  file.write_all(data.as_bytes()).map_err(|e| {
      let err_msg = e.to_string();
      error!("Failed to write to file {}: {}", filename, err_msg);
      err_msg
  })?;
  Ok(())
}