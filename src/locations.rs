use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub country: String,
    pub country_code: String,
    pub admin1: Option<String>,
    pub admin2: Option<String>,
    pub admin3: Option<String>,
    pub admin4: Option<String>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Locations {
    pub results: Vec<Location>,
}
