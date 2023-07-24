use crate::components::search::Search;
use crate::locations::Location;
use crate::Metric;

use crate::time::day_name;
use crate::weather::{weather_description, weather_icon};
use crate::{forecast::Forecast, CurrentTime};
use chrono::{DateTime, Datelike, FixedOffset, Month, NaiveDateTime, Timelike, Utc};
use leptos::*;
use leptos_icons::Icon;
use leptos_icons::LuIcon::{LuDroplets, LuThermometer, LuUndo2, LuWind};

pub type Hour = (i64, f32, f32, u32, bool, u32, u32, f32);

#[component]
fn MetricButton(
    cx: Scope,
    metric: Metric,
    selected_metric: (ReadSignal<Metric>, WriteSignal<Metric>),
    icon: Icon,
    children: Children,
) -> impl IntoView {
    let (selected_metric, set_selected_metric) = selected_metric;

    view! { cx,
        <button
            on:click=move |_| set_selected_metric(metric)
            class="flex gap-x-2 items-center py-1 px-2 rounded-full border border-current transition-all hover:opacity-75 hover:scale-105 active:scale-95 backdrop-blur-md"
            class=("opacity-50", move || selected_metric() != metric)
        >
            <Icon width="16" height="16" icon=icon/>
            {children(cx)}
        </button>
    }
}

#[component]
pub fn Today(
    cx: Scope,
    forecast: Resource<Option<Location>, Option<Forecast>>,
    current: MaybeSignal<Option<Hour>>,
    time: (ReadSignal<CurrentTime>, WriteSignal<CurrentTime>),
    metric: (ReadSignal<Metric>, WriteSignal<Metric>),
    location: (ReadSignal<Option<Location>>, WriteSignal<Option<Location>>),
) -> impl IntoView {
    let (time, set_time) = time;
    let (metric, set_metric) = metric;

    let (location, set_location) = location;

    view! { cx,
        <div class="flex w-full h-1/2">
            <div class="flex flex-col gap-y-4 justify-center py-12 m-auto text-center rounded-3xl shrink">
                <Search location=(location, set_location)/>
                <div class="flex gap-x-4 justify-center">
                    {move || {
                        let forecast = forecast.read(cx)??;
                        let offset = FixedOffset::east_opt(forecast.utc_offset_seconds)?;
                        let now: DateTime<FixedOffset> = DateTime::from_utc(
                            NaiveDateTime::from_timestamp_opt(time().into(), 0)?,
                            offset,
                        );
                        let month = Month::try_from(u8::try_from(now.month()).ok()?).ok()?;
                        let time = format!("{:02}:{:02}", now.hour(), now.minute());
                        Some(
                            view! { cx,
                                <span>
                                    {format!("{} on {}, {} {}", time, day_name(now.weekday()), now.day(), month.name())}
                                </span>
                            },
                        )
                    }} {move || match time() {
                        CurrentTime::Now(_) => None,
                        CurrentTime::Later(_) => {
                            Some(
                                view! {
                                    cx, < button class =
                                    "flex gap-x-1 py-1 px-2 my-auto text-xs rounded-full border border-current transition-opacity transition-transform hover:opacity-75 hover:scale-105 active:opacity-50 active:scale-95 backdrop-blur-md"
                                    on : click = move | _ | set_time(CurrentTime::Now(Utc::now()
                                    .timestamp())) >< Icon width = "16" height = "16" icon =
                                    Icon::from(LuUndo2) />< p > now </ p ></ button >
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
                                cx, < div class = "items-center text-center" > < h1 class =
                                "my-4 text-7xl font-bold sm:text-8xl" > { temperature_2m } "°" </ h1 > <
                                p class = "text-lg" > Feels like { apparent_temperature } "°" </ p > <
                                div class = "flex gap-x-2 justify-center items-center my-2" > < Icon
                                class = "shrink-0" width = "16" height = "16" icon =
                                weather_icon(weathercode, is_day) /> < p class = "text-sm" > {
                                weather_description(weathercode, is_day) } </ p > </ div > </ div > < div
                                class = "flex gap-x-4 mx-auto" > < MetricButton metric =
                                Metric::Temperature selected_metric = (metric, set_metric) icon =
                                Icon::from(LuThermometer) > { format!("{}°", temperature_2m) } </
                                MetricButton > < MetricButton metric = Metric::Precipitation
                                selected_metric = (metric, set_metric) icon = Icon::from(LuDroplets) > {
                                format!("{}%", precipitation_probability) } </ MetricButton > <
                                MetricButton metric = Metric::Wind selected_metric = (metric, set_metric)
                                icon = Icon::from(LuWind) >< span > { windspeed } < span class =
                                "pl-1 text-xs" > "km/h" </ span ></ span ></ MetricButton > </ div >
                            }
                                .into_view(cx)
                        })
                }}
            </div>
        </div>
    }
}
