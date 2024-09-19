use crate::util::assets::get_file;
use crate::consts::CONTRACTION_PATH;
use crate::structs::Contraction;
use anyhow::Result;
use log::{error, info};
use serde_json::Value;

pub fn get_intro_contraction(airport_code: &str, atis_type: &str) -> Vec<Contraction> {
    let json = get_file::<Value>(&CONTRACTION_PATH).unwrap()["airports"].clone();

    let atis_type_spoken = match atis_type {
        "dep" => "DEPARTURE",
        "arr" => "ARRIVAL",
        _ => "",
    };

    let string = format!("{} {} INFO", &airport_code[1..], atis_type);
    let spoken = format!(
        "{} AIRPORT {} INFORMATION",
        json[airport_code].to_string(),
        atis_type_spoken
    );

    vec![Contraction { string, spoken }]
}

pub fn write_contractions(
    existing: &mut Vec<Value>,
    atis: Value,
    airport_code: &str,
    atis_type: &str,
) -> Result<Vec<Value>> {
    let json = get_file::<Value>(&CONTRACTION_PATH).unwrap()["notam_contractions"].clone();

    if let Value::Object(map) = json {
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

        let intro_contraction = get_intro_contraction(airport_code, atis_type);

        if !existing.contains(&serde_json::to_value(intro_contraction.first().unwrap()).unwrap()) {
            existing.push(serde_json::to_value(intro_contraction.first().unwrap()).unwrap());
        }

        info!("Custom contractions updated for {}", airport_code);
        Ok(existing.to_vec())
    } else {
        let e = format!("Failed to parse custom contractions for {}", airport_code);
        error!("{}", e);
        Err(anyhow::Error::msg(e))
    }
}

