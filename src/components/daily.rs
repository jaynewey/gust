use leptos::*;
use leptos_icons::Icon;

use crate::forecast::Forecast;
use crate::locations::Location;
use crate::palette::Palette;
use crate::time::day_name;
use crate::Metric;
use crate::{
    weather::{weather_description, weather_icon, wind_direction_icon},
    CurrentTime,
};
use chrono::{DateTime, Datelike, FixedOffset, Month, NaiveDateTime, Utc};
use itertools::izip;
use leptos_icons::LuIcon::{LuDroplet, LuDroplets};

#[component]
pub fn Daily(
    cx: Scope,
    forecast: Resource<Option<Location>, Option<Forecast>>,
    time: (ReadSignal<CurrentTime>, WriteSignal<CurrentTime>),
    metric: ReadSignal<Metric>,
) -> impl IntoView {
    let (palette, _) = use_context::<(ReadSignal<Palette<'_>>, WriteSignal<Palette<'_>>)>(cx)
        .expect("palette context");
    let (time, set_time) = time;

    view! { cx,
        <div class="flex flex-col gap-y-4 mx-auto w-full rounded-3xl grow justify-stretch">
            {move || {
                let forecast = forecast.read(cx)??;
                Some(
                    izip!(
                        forecast.daily.time, forecast.daily.temperature_2m_max, forecast.daily
                        .weathercode, forecast.daily.precipitation_probability_max, forecast.daily
                        .winddirection_10m_dominant, forecast.daily.windspeed_10m_max
                    )
                        .filter_map(|
                            (
                                this_time,
                                temperature,
                                weathercode,
                                precipitation_probability,
                                winddirection,
                                windspeed,
                            )|
                        {
                            let offset = FixedOffset::east_opt(forecast.utc_offset_seconds)
                                .unwrap();
                            let naive_datetime = NaiveDateTime::from_timestamp_opt(this_time, 0)?;
                            let datetime: DateTime<FixedOffset> = DateTime::from_utc(
                                naive_datetime,
                                offset,
                            );
                            let month = Month::try_from(u8::try_from(datetime.month()).unwrap_or(0))
                                .ok()?;
                            let now = Utc::now().timestamp();
                            let later = NaiveDateTime::from_timestamp_opt(
                                    this_time + offset.local_minus_utc() as i64,
                                    0,
                                )?
                                .date()
                                .and_time(
                                    NaiveDateTime::from_timestamp_opt(time().into(), 0)?.time(),
                                );
                            Some(

                                view! { cx,
                                    <button
                                        on:click=move |_| set_time(
                                            if this_time < now {
                                                CurrentTime::Now(now)
                                            } else {
                                                CurrentTime::Later(later.timestamp())
                                            },
                                        )

                                        class=move || {
                                            format!(
                                                "flex grow items-stretch content-center justify-between rounded-3xl backdrop-blur-sm p-4 drop-shadow-sm transition-all hover:drop-shadow-md hover:scale-105 active:scale-95 active:drop-shadow-none {}",
                                                palette().background,
                                            )
                                        }
                                    >

                                        <div class="flex flex-col my-auto text-left">
                                            <p>{day_name(datetime.weekday())}</p>
                                            <p class="text-xs">
                                                {format!("{} {}", datetime.day(), month.name())}
                                            </p>
                                        </div>
                                        <div class="flex gap-x-4 my-auto text-2xl font-bold">
                                            <p class="mx-auto">
                                                {move || match metric() {
                                                    Metric::Temperature => {
                                                        format!("{}°", temperature.round() as i32).into_view(cx)
                                                    }
                                                    Metric::Precipitation => {
                                                        format!("{}%", precipitation_probability).into_view(cx)
                                                    }
                                                    Metric::Wind => {

                                                        view! { cx,
                                                            <span>{windspeed.round() as i32}</span>
                                                            <span class="pl-1 text-base">"km/h"</span>
                                                        }
                                                            .into_view(cx)
                                                    }
                                                }}

                                            </p>
                                            {move || match metric() {
                                                Metric::Temperature => {
                                                    view! { cx,
                                                        <div
                                                            class="my-auto"
                                                            title=weather_description(weathercode, true)
                                                        >
                                                            <Icon
                                                                width="24"
                                                                height="24"
                                                                icon=weather_icon(weathercode, true)
                                                            />
                                                        </div>
                                                    }
                                                        .into_view(cx)
                                                }
                                                Metric::Precipitation => {

                                                    view! { cx,
                                                        <Icon
                                                            width="24"
                                                            height="24"
                                                            class="my-auto"
                                                            icon=Icon::from(
                                                                if precipitation_probability > 75 {
                                                                    LuDroplets
                                                                } else {
                                                                    LuDroplet
                                                                },
                                                            )
                                                        />
                                                    }
                                                }
                                                Metric::Wind => {

                                                    view! { cx,
                                                        <Icon
                                                            width="24"
                                                            height="24"
                                                            class="my-auto"
                                                            icon=wind_direction_icon(winddirection)
                                                        />
                                                    }
                                                }
                                            }}

                                        </div>
                                    </button>
                                },
                            )
                        })
                        .collect_view(cx),
                )
            }}

        </div>
    }
}
