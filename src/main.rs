#![allow(non_snake_case)]
#![feature(exclusive_range_pattern)]

use std::collections::HashSet;

use chrono::Utc;
use itertools::izip;
use leptos::*;
pub mod components;
use crate::components::daily::*;
use crate::components::hourly::*;
use crate::components::scene::*;
use crate::components::today::*;
pub mod forecast;
pub mod locations;

pub mod palette;
pub mod time;
pub mod weather;

use crate::forecast::Forecast;
use crate::locations::Location;
use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub enum CurrentTime {
    Now(i64),
    Later(i64),
}
impl From<CurrentTime> for i64 {
    fn from(time: CurrentTime) -> Self {
        let (CurrentTime::Now(timestamp) | CurrentTime::Later(timestamp)) = time;
        timestamp
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Metric {
    Temperature,
    Precipitation,
    Wind,
}

const LOCAL_SERVER: &str = "http://127.0.0.1:8081";
const FORECAST_ENDPOINT: Option<&'static str> = option_env!("FORECAST_ENDPOINT");
const GEOCODING_ENDPOINT: Option<&'static str> = option_env!("GEOCODING_ENDPOINT");
pub const FLAG_ICONS_ENDPOINT: &str = "https://hatscripts.github.io/circle-flags/flags";

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();

    if let Some(window) = web_sys::window() {
        let _ = window
            .navigator()
            .service_worker()
            .register("./service_worker.js");
    }

    mount_to_body(|cx| {
        let (palette, set_palette) = create_signal(cx, palette::CLEAR);
        provide_context(cx, (palette, set_palette));

        let (metric, set_metric) = create_signal(
            cx,
            LocalStorage::get("metric").unwrap_or(Metric::Temperature),
        );
        create_effect(cx, move |_| LocalStorage::set("metric", metric()));

        let (location, set_location) = create_signal(cx, LocalStorage::get("location").ok());
        create_effect(cx, move |_| LocalStorage::set("location", location()));

        let (starred, set_starred): (
            ReadSignal<HashSet<Location>>,
            WriteSignal<HashSet<Location>>,
        ) = create_signal(cx, LocalStorage::get("starred").unwrap_or(HashSet::new()));
        create_effect(cx, move |_| LocalStorage::set("starred", starred()));

        let forecast = create_resource(cx, location, |location: Option<Location>| async move {
            let location = location?;
            Some(Request::get(format!("{}/forecast?latitude={}&longitude={}&hourly=temperature_2m,apparent_temperature,precipitation_probability,windspeed_10m,winddirection_10m,weathercode,is_day&daily=weathercode,temperature_2m_max,precipitation_probability_max,windspeed_10m_max,winddirection_10m_dominant&timeformat=unixtime&timezone=auto", FORECAST_ENDPOINT.unwrap_or(LOCAL_SERVER), location.latitude, location.longitude).as_str())
                    .send()
                    .await
                    .unwrap()
                    .json::<Forecast>()
                    .await
                    .unwrap())
        });

        let (time, set_time) = create_signal(cx, CurrentTime::Now(Utc::now().timestamp()));

        let current = MaybeSignal::derive(cx, move || match forecast.read(cx)? {
            Some(forecast) => {
                let current = izip!(
                    forecast.hourly.time,
                    forecast.hourly.temperature_2m,
                    forecast.hourly.apparent_temperature,
                    forecast.hourly.weathercode,
                    forecast.hourly.is_day,
                    forecast.hourly.precipitation_probability,
                    forecast.hourly.winddirection_10m,
                    forecast.hourly.windspeed_10m,
                )
                .filter(|&(other_time, _, _, _, _, _, _, _)| {
                    <CurrentTime as Into<i64>>::into(time()) >= other_time
                })
                .last() as Option<Hour>; // shuts rust-analyzer up

                let (_, _, _, weathercode, is_day, _, _, _) = current?;
                set_palette(if is_day {
                    match weathercode {
                        0 | 1 => palette::CLEAR,
                        2 | 3 | 45 | 48 => palette::CLOUDY,
                        56 | 57 | 66 | 67 | 71 | 73 | 75 | 77 | 85 | 86 => palette::SNOW,
                        95 | 96 | 99 => palette::THUNDER,
                        61 | 63 | 65 | 80 | 81 | 82 => palette::RAIN,
                        51 | 53 | 55 => palette::DRIZZLE,
                        _ => palette::CLEAR,
                    }
                } else {
                    match weathercode {
                        0 | 1 => palette::NIGHT_CLEAR,
                        2 | 3 | 45 | 48 => palette::NIGHT_RAIN,
                        56 | 57 | 66 | 67 | 71 | 73 | 75 | 77 | 85 | 86 => palette::NIGHT_SNOW,
                        95 | 96 | 99 => palette::THUNDER,
                        61 | 63 | 65 | 80 | 81 | 82 => palette::NIGHT_RAIN,
                        51 | 53 | 55 => palette::NIGHT_RAIN,
                        _ => palette::NIGHT_CLEAR,
                    }
                });

                current
            }
            None => {
                set_palette(palette::CLEAR);
                None
            }
        });
        provide_context(cx, current);

        view! { cx,
            <Scene />
                <div class=move || {
                    format!(
                        "w-screen h-screen font-sans md:grid md:grid-cols-2 lg:grid-cols-4 {}", palette()
                        .text
                    )
                }>
                    <div class="flex flex-col col-span-1 p-4 lg:col-span-3">
                        <Today forecast=forecast time=(time, set_time) metric=(metric, set_metric) location=(location, set_location) starred=(starred, set_starred) current=current/>
                        <Hourly forecast=forecast time=(time, set_time) metric=metric/>
                    </div>
                    <div class="flex flex-col col-span-1 p-4 md:h-screen">
                        <Daily forecast=forecast time=(time, set_time) metric=metric/>
                        <a target="_blank" class="mt-4 text-sm text-center text-white opacity-50 transition-opacity hover:opacity-75" href="https://open-meteo.com/">Weather data by Open-Meteo.com</a>
                    </div>
                </div>
        }
    })
}
