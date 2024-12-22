use std::collections::HashSet;

use crate::components::search::Search;
use crate::locations::Location;
use crate::Metric;

use crate::time::day_name;
use crate::weather::{weather_description, weather_icon};
use crate::FLAG_ICONS_ENDPOINT;
use crate::{forecast::Forecast, CurrentTime};
use chrono::{DateTime, Datelike, FixedOffset, Month, NaiveDateTime, Timelike, Utc};
use leptos::prelude::*;
use leptos_icons::Icon;

pub type Hour = (i64, f32, f32, u32, bool, u32, u32, f32);

#[component]
fn LocationButton(
    location: Location,
    selected_location: (ReadSignal<Option<Location>>, WriteSignal<Option<Location>>),
    starred: (
        ReadSignal<HashSet<Location>>,
        WriteSignal<HashSet<Location>>,
    ),
    children: Children,
) -> impl IntoView {
    let (selected_location, set_selected_location) = selected_location;
    let (starred, set_starred) = starred;
    let location_ = location.clone();
    let location__ = location_.clone();

    view! {
        <button
            on:click=move |_| set_selected_location(Some(location.clone()))
            class="flex gap-x-2 items-center py-1 px-2 rounded-full border border-current transition-all hover:opacity-75 hover:scale-105 active:scale-95 backdrop-blur-md"
            class=("opacity-50", move || selected_location() != Some(location_.clone()))
        >
            {children()}
            <button
                on:click=move |event| {
                    event.stop_propagation();
                    let mut starred = starred().clone();
                    starred.remove(&location__.clone());
                    set_starred(starred);
                }

                class="opacity-50 hover:opacity-100"
            >
                <Icon width="16" height="16" icon=icondata::LuX />
            </button>
        </button>
    }
}

#[component]
fn MetricButton(
    metric: Metric,
    selected_metric: (ReadSignal<Metric>, WriteSignal<Metric>),
    icon: icondata::Icon,
    children: Children,
) -> impl IntoView {
    let (selected_metric, set_selected_metric) = selected_metric;

    view! {
        <button
            on:click=move |_| set_selected_metric(metric)
            class="flex gap-x-2 items-center py-1 px-2 rounded-full border border-current transition-all hover:opacity-75 hover:scale-105 active:scale-95 backdrop-blur-md"
            class=("opacity-50", move || selected_metric() != metric)
        >
            <Icon width="16" height="16" icon=icon />
            {children()}
        </button>
    }
}

#[component]
pub fn Today(
    forecast: LocalResource<Option<Forecast>>,
    current: Signal<Option<Hour>>,
    time: (ReadSignal<CurrentTime>, WriteSignal<CurrentTime>),
    metric: (ReadSignal<Metric>, WriteSignal<Metric>),
    location: (ReadSignal<Option<Location>>, WriteSignal<Option<Location>>),
    starred: (
        ReadSignal<HashSet<Location>>,
        WriteSignal<HashSet<Location>>,
    ),
) -> impl IntoView {
    let (time, set_time) = time;
    let (metric, set_metric) = metric;

    let (location, set_location) = location;
    let (starred, set_starred) = starred;

    view! {
        <div class="flex max-w-full h-1/2">
            <div class="flex flex-col gap-y-4 justify-center py-12 m-auto max-w-full text-center rounded-3xl shrink">
                <Search location=(location, set_location) starred=(starred, set_starred) />
                <div class="flex gap-x-4 justify-center">
                    {move || {
                        let forecast = forecast.get().as_deref()?.to_owned()?;
                        let offset = FixedOffset::east_opt(forecast.utc_offset_seconds)?;
                        let now: DateTime<FixedOffset> = DateTime::from_utc(
                            NaiveDateTime::from_timestamp_opt(time().into(), 0)?,
                            offset,
                        );
                        let month = Month::try_from(u8::try_from(now.month()).ok()?).ok()?;
                        let time = format!("{:02}:{:02}", now.hour(), now.minute());
                        Some(
                            view! {
                                <span>
                                    {format!(
                                        "{} on {}, {} {}",
                                        time,
                                        day_name(now.weekday()),
                                        now.day(),
                                        month.name(),
                                    )}

                                </span>
                            },
                        )
                    }}
                    {move || match time() {
                        CurrentTime::Now(_) => None,
                        CurrentTime::Later(_) => {
                            Some(
                                view! {
                                    <button
                                        class="flex gap-x-1 py-1 px-2 my-auto text-xs rounded-full border border-current transition-opacity transition-transform hover:opacity-75 hover:scale-105 active:opacity-50 active:scale-95 backdrop-blur-md"
                                        on:click=move |_| set_time(
                                            CurrentTime::Now(Utc::now().timestamp()),
                                        )
                                    >

                                        <Icon width="16" height="16" icon=icondata::LuUndo2 />
                                        <p>now</p>
                                    </button>
                                },
                            )
                        }
                    }}

                </div>
                {move || {
                    current()
                        .map(|
                            (
                                _,
                                temperature_2m,
                                apparent_temperature,
                                weathercode,
                                is_day,
                                precipitation_probability,
                                _,
                                windspeed,
                            )|
                        {

                            view! {
                                <div class="items-center text-center">
                                    <h1 class="my-4 text-7xl font-bold sm:text-8xl">
                                        {temperature_2m.round() as i32} "°"
                                    </h1>
                                    <p class="text-lg">
                                        Feels like {apparent_temperature.round() as i32} "°"
                                    </p>
                                    <div class="flex gap-x-2 justify-center items-center my-2">
                                        <Icon
                                            width="16"
                                            height="16"
                                            icon=weather_icon(weathercode, is_day)
                                            {..}
                                            class="shrink-0"
                                        />
                                        <p class="text-sm">
                                            {weather_description(weathercode, is_day)}
                                        </p>
                                    </div>
                                </div>
                                <div class="flex gap-x-4 mx-auto">
                                    <MetricButton
                                        metric=Metric::Temperature
                                        selected_metric=(metric, set_metric)
                                        icon=icondata::LuThermometer
                                    >
                                        {format!("{}°", temperature_2m.round() as i32)}
                                    </MetricButton>
                                    <MetricButton
                                        metric=Metric::Precipitation
                                        selected_metric=(metric, set_metric)
                                        icon=icondata::LuDroplets
                                    >
                                        {format!("{}%", precipitation_probability)}
                                    </MetricButton>
                                    <MetricButton
                                        metric=Metric::Wind
                                        selected_metric=(metric, set_metric)
                                        icon=icondata::LuWind
                                    >
                                        <span>
                                            {windspeed.round() as i32}
                                            <span class="pl-1 text-xs">"km/h"</span>
                                        </span>
                                    </MetricButton>
                                </div>
                                <div class="flex overflow-x-auto gap-x-4 p-2 mx-auto max-w-full text-sm whitespace-nowrap">
                                    {move || {
                                        starred()
                                            .into_iter()
                                            .map(|starred_location| {
                                                view! {
                                                    <LocationButton
                                                        location=starred_location.clone()
                                                        selected_location=(location, set_location)
                                                        starred=(starred, set_starred)
                                                    >
                                                        <div class="w-4">
                                                            <img
                                                                src=format!(
                                                                    "{}/{}.svg",
                                                                    FLAG_ICONS_ENDPOINT,
                                                                    starred_location.country_code.to_lowercase(),
                                                                )

                                                                width=16
                                                                class="my-auto"
                                                            />
                                                        </div>
                                                        <span>{starred_location.name.clone()}</span>
                                                    </LocationButton>
                                                }
                                                    .into_view()
                                            })
                                            .collect_view()
                                    }}

                                </div>
                            }
                                .into_view()
                        })
                }}

            </div>
        </div>
    }
}
