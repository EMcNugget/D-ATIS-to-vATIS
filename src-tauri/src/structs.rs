use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Settings {
    pub facility: String,
    pub file_path: String,
    pub save_facility: bool,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalGenerator {
    enabled: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VATIS {
    id: Uuid,
    name: String,
    airport_conditions: String,
    notams: String,
    template: String,
    external_generator: ExternalGenerator,
}