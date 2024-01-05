use crate::Metric;
use leptos::*;
use leptos_icons::Icon;
use leptos_icons::LuIcon::{LuDroplet, LuDroplets};

use crate::forecast::Forecast;
use crate::locations::Location;
use crate::palette::Palette;
use crate::{
    weather::{weather_icon, wind_direction_icon},
    CurrentTime,
};
use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime, Timelike, Utc};
use itertools::izip;

#[component]
pub fn Hourly(
    cx: Scope,
    forecast: Resource<Option<Location>, Option<Forecast>>,
    time: (ReadSignal<CurrentTime>, WriteSignal<CurrentTime>),
    metric: ReadSignal<Metric>,
) -> impl IntoView {
    let (palette, _) = use_context::<(ReadSignal<Palette<'_>>, WriteSignal<Palette<'_>>)>(cx)
        .expect("palette context");
    let (time, set_time) = time;

    view! { cx,
        <div class=move || {
            format!(
                "flex max-w-full shrink mt-auto mx-auto drop-shadow-sm backdrop-blur-sm rounded-3xl overflow-x-auto gap-8 px-6 py-4 empty:hidden {}",
                palette().background
            )
        }>
            {move || {
                let forecast = forecast.read(cx)??;
                let offset = FixedOffset::east_opt(forecast.utc_offset_seconds)?;
                Some(
                    izip!(
                        forecast.hourly.time, forecast.hourly.temperature_2m, forecast.hourly
                        .weathercode, forecast.hourly.is_day, forecast.hourly
                        .precipitation_probability, forecast.hourly.winddirection_10m, forecast
                        .hourly.windspeed_10m
                    )
                        .filter_map(|
                            (
                                other_time,
                                temperature,
                                weathercode,
                                is_day,
                                precipitation_probability,
                                winddirection,
                                windspeed,
                            )|
                        {
                            let selected: DateTime<FixedOffset> = DateTime::from_utc(
                                NaiveDateTime::from_timestamp_opt(time().into(), 0)?,
                                offset,
                            );
                            let now = (Utc::now().with_timezone(&offset) - Duration::hours(1))
                                .timestamp() + forecast.utc_offset_seconds as i64;
                            let start = std::cmp::max(
                                selected.date_naive().and_hms_opt(0, 0, 0)?.timestamp(),
                                now,
                            );
                            let end = selected.date_naive().and_hms_opt(23, 59, 59)?;
                            if (start..end.timestamp())
                                .contains(&(other_time + forecast.utc_offset_seconds as i64))
                            {
                                Some(
                                    view! { cx,
                                        <button
                                            on:click=move |_| set_time(CurrentTime::Later(other_time))
                                            class="flex flex-col gap-y-2 justify-center w-16 text-center transition-transform hover:scale-105 active:scale-95"
                                        >
                                            <p class="mx-auto text-xs">
                                                {if let Some(naive_datetime) = NaiveDateTime::from_timestamp_opt(other_time, 0) {
                                                    let datetime: DateTime<FixedOffset> = DateTime::from_utc(naive_datetime, offset);
                                                    format!("{:02}:{:02}", datetime.hour(), datetime.minute())
                                                } else {
                                                    String::from("?")
                                                }}
                                            </p>
                                            {move || match metric() {
                                                Metric::Temperature => {
                                                    view! { cx, <Icon width="24" height="24" class="mx-auto" icon=weather_icon(weathercode, is_day)/> }
                                                }
                                                Metric::Precipitation => {
                                                    view! { cx,
                                                        <Icon
                                                            width="24"
                                                            height="24"
                                                            class="mx-auto"
                                                            icon=Icon::from(if precipitation_probability > 75 { LuDroplets } else { LuDroplet })
                                                        />
                                                    }
                                                }
                                                Metric::Wind => {
                                                    view! { cx, <Icon width="24" height="24" class="mx-auto" icon=wind_direction_icon(winddirection)/> }
                                                }
                                            }}
                                            <p class="mx-auto">
                                                {move || match metric() {
                                                    Metric::Temperature => format!("{}°", temperature.round() as i32).into_view(cx),
                                                    Metric::Precipitation => format!("{}%", precipitation_probability).into_view(cx),
                                                    Metric::Wind => {
                                                        view! { cx,
                                                            <span>{windspeed.round() as i32}</span>
                                                            <span class="pl-1 text-xs">"km/h"</span>
                                                        }
                                                            .into_view(cx)
                                                    }
                                                }}
                                            </p>
                                        </button>
                                    },
                                )
                            } else {
                                None
                            }
                        })
                        .collect_view(cx),
                )
            }}
        </div>
    }.into_view(cx)
}
