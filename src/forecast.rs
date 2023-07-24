use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Hourly {
    pub time: Vec<i64>,
    pub temperature_2m: Vec<f32>,
    pub apparent_temperature: Vec<f32>,
    #[serde(deserialize_with = "null_to_default")]
    pub precipitation_probability: Vec<u32>,
    pub windspeed_10m: Vec<f32>,
    pub winddirection_10m: Vec<u32>,
    pub weathercode: Vec<u32>,
    #[serde(deserialize_with = "bool_from_int")]
    pub is_day: Vec<bool>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Daily {
    pub time: Vec<i64>,
    pub temperature_2m_max: Vec<f32>,
    #[serde(deserialize_with = "null_to_default")]
    pub precipitation_probability_max: Vec<u32>,
    pub windspeed_10m_max: Vec<f32>,
    pub winddirection_10m_dominant: Vec<u32>,
    pub weathercode: Vec<u32>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Forecast {
    pub latitude: f32,
    pub longitude: f32,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub elevation: f32,
    pub hourly: Hourly,
    pub daily: Daily,
}
fn bool_from_int<'de, D>(deserializer: D) -> Result<Vec<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let int_vec: Vec<u8> = Deserialize::deserialize(deserializer)?;
    let bool_vec: Vec<bool> = int_vec.into_iter().map(|x| x != 0).collect();
    Ok(bool_vec)
}
fn null_to_default<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt_vec: Vec<Option<T>> = Deserialize::deserialize(deserializer)?;
    let default_vec = opt_vec.into_iter().map(Option::unwrap_or_default).collect();
    Ok(default_vec)
}
