use leptos::prelude::*;
use leptos_icons::Icon;

use crate::forecast::Forecast;
use crate::palette::Palette;
use crate::time::day_name;
use crate::Metric;
use crate::{
    weather::{weather_description, weather_icon, wind_direction_icon},
    CurrentTime,
};
use chrono::{DateTime, Datelike, FixedOffset, Month, NaiveDateTime, Utc};
use itertools::izip;

#[component]
pub fn Daily(
    forecast: LocalResource<Option<Forecast>>,
    time: (ReadSignal<CurrentTime>, WriteSignal<CurrentTime>),
    metric: ReadSignal<Metric>,
) -> impl IntoView {
    let (palette, _) = use_context::<(ReadSignal<Palette<'_>>, WriteSignal<Palette<'_>>)>()
        .expect("palette context");
    let (time, set_time) = time;

    view! {
        <div class=move || {format!("flex flex-col p-4 gap-y-4 mx-auto w-full rounded-3xl grow justify-stretch rounded-3xl backdrop-blur-sm drop-shadow-sm {}", palette().background)}>
            {move || {
                let forecast = forecast.get().as_deref()?.to_owned()?;
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
                            let offset = FixedOffset::east_opt(forecast.utc_offset_seconds)?;
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
                            let current_date = NaiveDateTime::from_timestamp_opt(time().into(), 0)?.date();
                            Some(

                                view! {
                                    <button
                                        on:click=move |_| set_time(
                                            if this_time < now {
                                                CurrentTime::Now(now)
                                            } else {
                                                CurrentTime::Later(later.timestamp())
                                            },
                                        )

                                        class="flex justify-between content-center items-stretch p-4 transition-all hover:scale-105 active:scale-95 grow"
                                        class=("opacity-75", move || naive_datetime.date() != current_date)
                                        class=("font-semibold", move || naive_datetime.date() == current_date)
                                    >

                                        <div class="flex flex-col my-auto text-left">
                                            <p>{day_name(datetime.weekday())}</p>
                                            <p class="text-xs">
                                                {format!("{} {}", datetime.day(), month.name())}
                                            </p>
                                        </div>
                                        <div
                                            class="flex gap-x-4 my-auto text-2xl font-black"
                                            class=("font-bold", move || naive_datetime.date() != current_date)
                                        >

                                            <p class="mx-auto">
                                                {move || match metric() {
                                                    Metric::Temperature => {
                                                        format!("{}°", temperature.round() as i32).into_any()
                                                    }
                                                    Metric::Precipitation => {
                                                        format!("{}%", precipitation_probability).into_any()
                                                    }
                                                    Metric::Wind => {

                                                        view! {
                                                            <span>{windspeed.round() as i32}</span>
                                                            <span class="pl-1 text-base">"km/h"</span>
                                                        }
                                                            .into_any()
                                                    }
                                                }}

                                            </p>
                                            {move || match metric() {
                                                Metric::Temperature => {
                                                    view! {
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
                                                        .into_any()
                                                }
                                                Metric::Precipitation => {

                                                    view! {
                                                        <Icon
                                                            width="24"
                                                            height="24"
                                                            icon={if precipitation_probability > 75 {
                                                                icondata::LuDroplets
                                                            } else {
                                                                icondata::LuDroplet
                                                            }}
                                                            {..}
                                                            class="my-auto"
                                                        />
                                                    }
                                                        .into_any()
                                                }
                                                Metric::Wind => {

                                                    view! {
                                                        <Icon
                                                            width="24"
                                                            height="24"
                                                            icon=wind_direction_icon(winddirection)
                                                            {..}
                                                            class="my-auto"
                                                        />
                                                    }
                                                        .into_any()
                                                }
                                            }}

                                        </div>
                                    </button>
                                },
                            )
                        })
                        .collect_view(),
                )
            }}

        </div>
    }
}
