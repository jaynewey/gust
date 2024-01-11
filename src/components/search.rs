use std::collections::HashSet;

use gloo_net::http::Request;
use gloo_timers::callback::Timeout;
use itertools::Itertools;
use leptos_icons::Icon;
use leptos_icons::LuIcon::{LuSearch, LuStar, LuStarOff};

use leptos::*;

use crate::locations::{Location, Locations};
use crate::palette::Palette;
use crate::FLAG_ICONS_ENDPOINT;
use crate::{GEOCODING_ENDPOINT, LOCAL_SERVER};

const DEBOUNCE_TIMEOUT: u32 = 500;

#[component]
pub fn Search(
    cx: Scope,
    location: (ReadSignal<Option<Location>>, WriteSignal<Option<Location>>),
    starred: (
        ReadSignal<HashSet<Location>>,
        WriteSignal<HashSet<Location>>,
    ),
) -> impl IntoView {
    let (palette, _) = use_context::<(ReadSignal<Palette<'_>>, WriteSignal<Palette<'_>>)>(cx)
        .expect("palette context");

    let (location, set_location) = location;
    let (starred, set_starred) = starred;
    let (search, set_search) = create_signal(cx, "".to_string());

    let (is_focussed, set_is_focussed) = create_signal(cx, false);

    let locations = create_action(cx, |search: &String| {
        let search = search.to_owned();
        async move {
            if search.len() > 1 {
                Request::get(
                    format!(
                        "{}/search?name={}",
                        GEOCODING_ENDPOINT.unwrap_or(LOCAL_SERVER),
                        search
                    )
                    .as_str(),
                )
                .send()
                .await
                .unwrap()
                .json::<Locations>()
                .await
                .unwrap()
            } else {
                Locations {
                    results: Vec::new(),
                }
            }
        }
    });

    create_effect(cx, move |timeout: Option<Timeout>| {
        if let Some(timeout) = timeout {
            timeout.cancel();
        }

        let search = search();
        Timeout::new(DEBOUNCE_TIMEOUT, move || locations.dispatch(search))
    });
    let value = move || {
        if is_focussed() {
            search()
        } else if let Some(location) = location() {
            format!("{}, {}", location.name, location.country)
        } else {
            search()
        }
    };
    let location_is_starred = move || {
        location()
            .map(|location| starred().contains(&location))
            .unwrap_or(false)
    };

    view! { cx,
        <div class="flex relative gap-x-4 content-center mx-auto group">
            <Icon class="my-auto shrink-0" width="24" height="24" icon=Icon::from(LuSearch)/>
            <div class="flex shrink">
                <input
                    on:input=move |ev| set_search(event_target_value(&ev))
                    prop:value=value
                    on:focus=move |_| set_is_focussed(true)
                    on:blur=move |_| set_is_focussed(false)
                    class="flex p-2 w-full text-3xl text-center outline-none lg:text-4xl bg-inherit"
                    placeholder="Search location"
                />
            </div>
            <ul
                class=move || {
                    format!(
                        "absolute flex flex-col justify-between z-20 top-full px-4 invisible group-focus-within:visible opacity-0 group-focus-within:opacity-100 transition backdrop-blur-md rounded-3xl mt-4 w-full max-h-[75vh] drop-shadow-sm {}",
                        palette().background
                    )
                }
                class=("hidden", move || { value().len() <= 1 })
            >
                {move || {
                    Some(
                        locations
                            .value()
                            .get()?
                            .results
                            .iter()
                            .map(|location| {
                                let location_clone = location.clone();
                                Some(
                                    view! { cx,
                                        <button
                                            on:click=move |_| {
                                                set_location(Some(location_clone.clone()));
                                                set_search("".to_string())
                                            }
                                            class="flex overflow-x-auto overflow-y-hidden gap-x-4 content-center py-4 transition-transform cursor-pointer hover:scale-105 active:scale-95"
                                        >
                                            <img
                                                src=format!("{}/{}.svg", FLAG_ICONS_ENDPOINT, location.country_code.to_lowercase())
                                                width=24
                                                class="my-auto"
                                            />
                                            <span class="whitespace-nowrap">
                                                {
                                                    let location = location.clone();
                                                    vec![
                                                        Some(location.name), location.admin4, location.admin3, location.admin2, location
                                                        .admin1, Some(location.country)
                                                    ]
                                                        .into_iter()
                                                        .flatten()
                                                        .dedup()
                                                        .join(", ")
                                                }
                                            </span>
                                        </button>
                                    },
                                )
                            })
                            .collect_view(cx),
                    )
                }}
            </ul>
            <button
                on:click=move |_| {
                    if let Some(location) = location() {
                        let mut starred = starred().clone();
                        if starred.contains(&location) {
                            starred.remove(&location);
                        } else {
                            starred.insert(location);
                        }
                        set_starred(starred);
                    }
                }
                class="my-auto hover:opacity-75 hover:scale-105 active:scale-95 shrink-0 group/star"
                class=("opacity-50", !location_is_starred())
            >
                <span class=("group-hover/star:hidden", location_is_starred())>
                    <Icon width="24" height="24" icon=Icon::from(LuStar)/>
                </span>
                <span class="hidden" class=("group-hover/star:block", location_is_starred())>
                    <Icon width="24" height="24" icon=Icon::from(LuStarOff)/>
                </span>
            </button>
        </div>
    }
}
