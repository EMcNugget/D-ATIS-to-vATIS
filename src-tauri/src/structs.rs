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
#[allow(non_snake_case)]
struct ATIS {
    id: Uuid,
    name: String,
    airportConditions: String,
    notams: String,
    template: String,
    externalGenerator: ExternalGenerator,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VATIS {
    atis_type: String,
    atis: ATIS,
}
