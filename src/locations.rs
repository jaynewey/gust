use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

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

impl Hash for Location {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.name.hash(state);
        self.country.hash(state);
        self.country_code.hash(state);
        self.admin1.hash(state);
        self.admin2.hash(state);
        self.admin3.hash(state);
        self.admin4.hash(state);
    }
}

impl Eq for Location {}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Locations {
    pub results: Vec<Location>,
}
