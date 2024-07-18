use serde_json::{self, Value};
use std::fs::File;
use std::io::{BufReader, Write};

fn read_json_file(file_path: &str) -> Result<Value, String> {
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| e.to_string())
}

fn write_json_file(filename: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
pub struct FindComposite {
    pub profile_index: usize,
    pub composite_index: usize,
}

pub fn find_composite(data: &Value, profile: &str, facility: &str) -> Option<FindComposite> {
    if let Some(profiles) = data.get("profiles").and_then(|p| p.as_array()) {
        for (profile_index, prof) in profiles.iter().enumerate() {
            if let Some(name) = prof.get("name").and_then(|n| n.as_str()) {
                if name == profile {
                    return Some(FindComposite {
                        profile_index,
                        composite_index: usize::MAX,
                    });
                }
            }
        }

        for (profile_index, profile) in profiles.iter().enumerate() {
            if let Some(composites) = profile.get("composites").and_then(|c| c.as_array()) {
                for (composite_index, composite) in composites.iter().enumerate() {
                    if let Some(identifier) = composite.get("identifier").and_then(|id| id.as_str())
                    {
                        if identifier == facility {
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
    None
}

pub fn write_profile(
    atis_preset: &Value,
    profile: &str,
    facility: &str,
    file_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let mut data: Value =
        serde_json::from_str(&read_json_file(file_path).unwrap().to_string()).unwrap();
    let indexes: FindComposite = find_composite(&data, profile, facility).unwrap();

    let presets = data["profiles"][indexes.profile_index]["composites"][indexes.composite_index]
        ["presets"]
        .as_array_mut()
        .unwrap();

    presets.push(serde_json::json!(atis_preset));

    write_json_file("Output.json", &data.to_string())?;
    Ok(())
}
