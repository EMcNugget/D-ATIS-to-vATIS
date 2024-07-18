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

// I know this is nested if then hell but until I get more confident in rust its going to remain this way
fn find_composite(
    data: &Value,
    profile: &str,
    facility: &str,
    atis_type: Option<&str>,
) -> Option<FindComposite> {
    fn find_atis_type(atis_type: Option<&str>) -> &str {
        match atis_type {
            Some("dep") => "Departure",
            Some("arr") => "Arrival",
            Some("combined") => "Combined",
            _ => "Combined",
        }
    }

    let atis_type_str = find_atis_type(atis_type);

    if let Some(profiles) = data.get("profiles").and_then(|p| p.as_array()) {
        for (profile_index, prof) in profiles.iter().enumerate() {
            if let Some(name) = prof.get("name").and_then(|n| n.as_str()) {
                if name == profile {
                    // Check composites for atis_type
                    if let Some(composites) = prof.get("composites").and_then(|c| c.as_array()) {
                        for (composite_index, composite) in composites.iter().enumerate() {
                            if let Some(identifier) =
                                composite.get("identifier").and_then(|id| id.as_str())
                            {
                                if identifier == facility {
                                    if let Some(atis) =
                                        composite.get("atisType").and_then(|a| a.as_str())
                                    {
                                        if atis == atis_type_str {
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
                            if let Some(atis) = composite.get("atisType").and_then(|a| a.as_str()) {
                                if atis == atis_type_str {
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
        }
    }
    None
}

pub fn write_profile(
    atis_preset: &Value,
    profile: &str,
    facility: &str,
    file_path: &str,
    atis_type: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut data: Value =
        serde_json::from_str(&read_json_file(file_path).unwrap().to_string()).unwrap();
    let indexes: FindComposite = find_composite(&data, profile, facility, atis_type).unwrap();

    let presets = data["profiles"][indexes.profile_index]["composites"][indexes.composite_index]
        ["presets"]
        .as_array_mut()
        .unwrap();

    presets.push(serde_json::json!(atis_preset));

    write_json_file("Output.json", &data.to_string())?;
    Ok(())
}
