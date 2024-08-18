use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Settings {
    pub facility: String,
    pub file_path: String,
    pub custom_path: bool,
    pub save_facility: bool,
    pub open_vatis_on_fetch: bool,
    pub check_updates: bool,
    pub check_updates_freq: bool,
    pub update_time: u64,
    pub profile: String,
    pub theme: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalGenerator {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ATIS {
    pub id: Uuid,
    pub name: String,
    pub airportConditions: String,
    pub notams: String,
    pub template: String,
    pub externalGenerator: ExternalGenerator,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VATIS {
    pub atis_type: String,
    pub atis: ATIS,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub alert_type: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FindComposite {
    pub profile_index: usize,
    pub composite_index: usize,
}

#[derive(Serialize)]
pub struct Alert {
    pub alert_type: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contraction {
    pub string: String,
    pub spoken: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Preset {
    pub name: String,
    pub index: usize,
}