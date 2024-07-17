use crate::settings::read_settings;
use serde_json::{self, Value};
use std::fs::File;
use std::io::Read;

fn read_json_file() -> Result<Value, Box<dyn std::error::Error>> {
    let settings = read_settings()?;

    let mut file = File::open(settings.file_path.to_string())?;
    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let data: Value = serde_json::from_str(&json_data)?;
    return Ok(data);
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
