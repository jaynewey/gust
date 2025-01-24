#![allow(non_snake_case)]

use std::collections::HashSet;

use chrono::Utc;
use itertools::izip;
use leptos::prelude::*;
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

use leptos_icons::Icon;

pub type Hour = (i64, f32, f32, u32, bool, u32, u32, f32);

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

async fn get_forecast(location: Option<Location>) -> Option<Forecast> {
    let location = location?;
    Some(Request::get(format!("{}/forecast?latitude={}&longitude={}&hourly=temperature_2m,apparent_temperature,precipitation_probability,windspeed_10m,winddirection_10m,weathercode,is_day&daily=weathercode,temperature_2m_max,precipitation_probability_max,windspeed_10m_max,winddirection_10m_dominant&timeformat=unixtime&timezone=auto", FORECAST_ENDPOINT.unwrap_or(LOCAL_SERVER), location.latitude, location.longitude).as_str())
            .send()
            .await
            .unwrap()
            .json::<Forecast>()
            .await
            .unwrap())
}

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();

    if let Some(window) = web_sys::window() {
        let _ = window
            .navigator()
            .service_worker()
            .register("./service_worker.js");
    }

    mount_to_body(|| {
        let (palette, set_palette) = signal(palette::CLEAR);
        provide_context((palette, set_palette));

        let (metric, set_metric) =
            signal(LocalStorage::get("metric").unwrap_or(Metric::Temperature));
        Effect::new(move |_| LocalStorage::set("metric", metric()));

        let (location, set_location) = signal(LocalStorage::get("location").ok());
        Effect::new(move |_| LocalStorage::set("location", location()));

        let (starred, set_starred): (
            ReadSignal<HashSet<Location>>,
            WriteSignal<HashSet<Location>>,
        ) = signal(LocalStorage::get("starred").unwrap_or(HashSet::new()));
        Effect::new(move |_| LocalStorage::set("starred", starred()));

        let forecast = LocalResource::new(move || {
            let location: Option<Location> = location();
            get_forecast(location)
        });

        let (time, set_time) = signal(CurrentTime::Now(Utc::now().timestamp()));

        let current = Signal::derive(move || match forecast.get().as_deref() {
            Some(Some(forecast)) => {
                let forecast = forecast.clone();
                let mut forecasts = izip!(
                    forecast.hourly.time,
                    forecast.hourly.temperature_2m,
                    forecast.hourly.apparent_temperature,
                    forecast.hourly.weathercode,
                    forecast.hourly.is_day,
                    forecast.hourly.precipitation_probability,
                    forecast.hourly.winddirection_10m,
                    forecast.hourly.windspeed_10m,
                );

                let current_time = <CurrentTime as Into<i64>>::into(time());
                let current_position =
                    forecasts
                        .clone()
                        .rposition(|(forecast_time, _, _, _, _, _, _, _)| {
                            current_time >= forecast_time
                        })?;

                let (prev, current) = if let Some(prev_position) = current_position.checked_sub(1) {
                    (
                        forecasts.nth(prev_position) as Option<Hour>,
                        forecasts.next() as Option<Hour>,
                    )
                } else {
                    (None, forecasts.nth(current_position) as Option<Hour>)
                };
                let next = forecasts.next() as Option<Hour>;

                let (_, _, _, weathercode, is_day, _, _, _) = current?;
                let prev_is_day = prev
                    .map(|(_, _, _, _, is_day, _, _, _)| is_day)
                    .unwrap_or(false);
                let next_is_day = next
                    .map(|(_, _, _, _, is_day, _, _, _)| is_day)
                    .unwrap_or(false);
                set_palette(if is_day {
                    match weathercode {
                        0 | 1 => {
                            if !next_is_day || !prev_is_day {
                                palette::DUSK_DAWN_SUNNY
                            } else {
                                palette::CLEAR
                            }
                        }
                        2 | 3 => {
                            if !next_is_day || !prev_is_day {
                                palette::DUSK_DAWN
                            } else {
                                palette::CLOUDY
                            }
                        }
                        45 | 48 => palette::FOGGY,
                        56 | 57 | 66 | 67 | 71 | 73 | 75 | 77 | 85 | 86 => palette::SNOW,
                        95 | 96 | 99 => palette::THUNDER,
                        61 | 63 | 65 | 80 | 81 | 82 => palette::RAIN,
                        51 | 53 | 55 => palette::DRIZZLE,
                        _ => palette::CLEAR,
                    }
                } else {
                    match weathercode {
                        0 | 1 => palette::NIGHT_CLEAR,
                        2 | 3 => palette::NIGHT_RAIN,
                        45 | 48 => palette::NIGHT_FOGGY,
                        56 | 57 | 66 | 67 | 71 | 73 | 75 | 77 | 85 | 86 => palette::NIGHT_SNOW,
                        95 | 96 | 99 => palette::THUNDER,
                        61 | 63 | 65 | 80 | 81 | 82 => palette::NIGHT_RAIN,
                        51 | 53 | 55 => palette::NIGHT_RAIN,
                        _ => palette::NIGHT_CLEAR,
                    }
                });

                current
            }
            _ => {
                set_palette(palette::CLEAR);
                None
            }
        });
        provide_context(current);

        view! {
            <Scene location=location />
            <div class=move || {
                format!(
                    "w-screen h-screen font-sans md:grid md:grid-cols-2 lg:grid-cols-4 {}",
                    palette().text,
                )
            }>
                <div class="flex flex-col col-span-1 p-4 lg:col-span-3">
                    <Today
                        forecast=forecast
                        time=(time, set_time)
                        metric=(metric, set_metric)
                        location=(location, set_location)
                        starred=(starred, set_starred)
                        current=current
                    />
                    <Hourly forecast=forecast time=(time, set_time) metric=metric current=current />
                </div>
                <div class="flex flex-col col-span-1 p-4 md:h-screen">
                    <Daily forecast=forecast time=(time, set_time) metric=metric />
                    <div class="flex overflow-x-auto flex-col gap-y-4 pb-2 mt-4 text-sm text-center text-white whitespace-nowrap opacity-50 transition-opacity md:flex-row md:gap-x-4 md:text-xs">
                        <a
                            target="_blank"
                            class="flex gap-x-2 mx-auto hover:opacity-75"
                            href="https://open-meteo.com/"
                        >
                            <Icon
                                width="16"
                                height="16"
                                icon={icondata::LuExternalLink}
                                {..}
                                class="my-auto"
                            />
                            <span class="my-auto">Weather data by Open-Meteo.com</span>
                        </a>
                        <a
                            target="_blank"
                            class="flex gap-x-2 mx-auto hover:opacity-75"
                            href="https://github.com/jaynewey/gust/"
                        >
                            <Icon
                                width="16"
                                height="16"
                                icon={icondata::LuGitFork}
                                {..}
                                class="my-auto"
                            />
                            <span class="my-auto">Source Code</span>
                        </a>
                    </div>
                </div>
            </div>
        }
    })
}
