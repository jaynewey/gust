#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! rouille = "3.6.2"
//! serde_json = "1.0"
//! chrono = "0.4.26"
//! rand = "0.8.5"
//! ```

use chrono::{NaiveDateTime, Utc};
use rand::seq::SliceRandom;
use rand::Rng;
use rouille::{router, Response};
use serde_json;
use std::fs;

const HOST: &str = "127.0.0.1";
const PORT: u32 = 8081;
const SEARCH_FILE: &str = "scripts/search.json";

fn forecast(latitude: f32, longitude: f32) -> serde_json::Value {
    let day_start = Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .timestamp();
    let mut rng = rand::thread_rng();
    let weathers = [
        0, 1, 2, 3, 45, 48, 51, 53, 55, 56, 57, 61, 63, 65, 66, 67, 71, 73, 75, 77, 80, 81, 82, 85,
        86, 95, 96, 99,
    ];
    serde_json::json!(
        {
            "latitude": latitude,
            "longitude": longitude,
            "generationtime_ms": 1.453995704650879,
            "utc_offset_seconds": 0,
            "timezone": "GMT",
            "timezone_abbreviation": "GMT",
            "elevation": 54.0,
            "hourly_units": {
              "time": "unixtime",
              "temperature_2m": "°C",
              "apparent_temperature": "°C",
              "precipitation_probability": "%",
              "windspeed_10m": "km/h",
              "winddirection_10m": "°",
              "weathercode": "wmo code",
              "is_day": ""
            },
            "hourly": {
                "time": (0..168).map(|i| day_start + (i * 3600)).collect::<Vec<_>>(),
                "temperature_2m": (0..168).map(|_| rng.gen_range(150..350) as f32 / 10.0).collect::<Vec<_>>(),
                "apparent_temperature": (0..168).map(|_| rng.gen_range(150..350) as f32 / 10.0 ).collect::<Vec<_>>(),
                "precipitation_probability": (0..168).map(|_| rng.gen_range(0..100)).collect::<Vec<_>>(),
                "weathercode": (0..168).map(|_| weathers.choose(&mut rng)).collect::<Vec<_>>(),
                "is_day": (0..168).map(|i| {
                    let current = day_start + (i * 3600);
                    // assume sunrise is always 6am
                    let sunrise = NaiveDateTime::from_timestamp_opt(current, 0).unwrap().date().and_hms_opt(6, 0, 0).unwrap().timestamp();
                    // assume sunset is always 8pm
                    let sunset = NaiveDateTime::from_timestamp_opt(current, 0).unwrap().date().and_hms_opt(20, 0, 0).unwrap().timestamp();
                    if current >= sunrise && current <= sunset {
                        1
                    } else { 0 }
                }).collect::<Vec<_>>(),
                "windspeed_10m": (0..168).map(|_| rng.gen_range(10..200) as f32 / 10.0).collect::<Vec<_>>(),
                "winddirection_10m": (0..168).map(|_| rng.gen_range(0..360)).collect::<Vec<_>>(),
            },
            "daily_units": {
              "time": "unixtime",
              "weathercode": "wmo code",
              "temperature_2m_max": "°C",
              "precipitation_probability_max": "%",
              "windspeed_10m_max": "km/h",
              "winddirection_10m_dominant": "°",
            },
            "daily": {
              "time": (0..7).map(|i| day_start + (i * 3600 * 24)).collect::<Vec<_>>(),
              "weathercode": (0..7).map(|_| weathers.choose(&mut rng)).collect::<Vec<_>>(),
              "temperature_2m_max": (0..7).map(|_| rng.gen_range(150..350) as f32 / 10.0).collect::<Vec<_>>(),
              "precipitation_probability_max": (0..7).map(|_| rng.gen_range(0..100)).collect::<Vec<_>>(),
              "windspeed_10m_max": (0..7).map(|_| rng.gen_range(10..200) as f32 / 10.0).collect::<Vec<_>>(),
              "winddirection_10m_dominant": (0..7).map(|_| rng.gen_range(0..360)).collect::<Vec<_>>(),
            }
        }
    )
}

fn main() {
    println!("Serving at {} on port {}", HOST, PORT);

    let json = fs::read_to_string(SEARCH_FILE).unwrap();
    let search: serde_json::Value = serde_json::from_str(&json).unwrap();

    rouille::start_server(format!("{}:{}", HOST, PORT), move |request| {
        router!(request,
            (GET) (/forecast) => {
                let latitude: Option<f32> = request.get_param("latitude").map(|l| l.parse().ok()).flatten();
                let longitude: Option<f32> = request.get_param("longitude").map(|l| l.parse().ok()).flatten();
                match (latitude, longitude) {
                    (Some(latitude), Some(longitude)) => {
                        Response::json(&(forecast(latitude, longitude))).with_additional_header("Access-Control-Allow-Origin", "*")
                    }
                    _ => Response::empty_400()
                }
            },
            (GET) (/search) => {
                Response::json(&search).with_additional_header("Access-Control-Allow-Origin", "*")
            },
            _ => Response::empty_404()
        )
    });
}
