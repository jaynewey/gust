use crate::palette::Palette;
use leptos::*;

use crate::components::today::Hour;

#[component]
pub fn Balloon(
    cx: Scope,
    scale: f32,
    translate: (f32, f32),
    palette: ReadSignal<Palette<'static>>,
) -> impl IntoView {
    let (x, y) = translate;
    let current = use_context::<MaybeSignal<Option<Hour>>>(cx).expect("current forecast context");

    view! { cx,
        <g
            transform=move || {
                let (wind_x, wind_y) = if let Some((_, _, _, _, _, _, winddirection, windspeed))
                    = current() {
                    let winddirection = (winddirection as f32).to_radians();
                    let x = windspeed * winddirection.sin();
                    let y = -windspeed * winddirection.cos();
                    (x, y)
                } else {
                    (0.0, 0.0)
                };
                format!("scale({}) translate({}, {})", scale, x + wind_x, y + wind_y)
            }
            class=(
                "opacity-0",
                move || {
                    if let Some((_, _, _, weathercode, _, _, _, _)) = current() {
                        [95, 96, 99].contains(&weathercode)
                    } else {
                        false
                    }
                },
            )
        >
            <g transform="translate(-74.935 -.2274)">
                <g transform="matrix(.26458 0 0 .26458 7.202 -33.639)" class=move || palette().dark>
                    <path d="m288 239.05a0.94488 0.94488 0 0 0-0.66797 0.27734 0.94488 0.94488 0 0 0 0 1.3359l32 32a0.94488 0.94488 0 0 0 1.3359 0 0.94488 0.94488 0 0 0 0-1.3359l-32-32a0.94488 0.94488 0 0 0-0.66797-0.27734z"></path>
                    <path d="m352 239.05a0.94488 0.94488 0 0 0-0.66797 0.27734l-32 32a0.94488 0.94488 0 0 0 0 1.3359 0.94488 0.94488 0 0 0 1.3359 0l32-32a0.94488 0.94488 0 0 0 0-1.3359 0.94488 0.94488 0 0 0-0.66797-0.27734z"></path>
                    <path d="m320 239.05a0.94488 0.94488 0 0 0-0.94531 0.94531v32a0.94488 0.94488 0 0 0 0.94531 0.94531 0.94488 0.94488 0 0 0 0.94531-0.94531v-32a0.94488 0.94488 0 0 0-0.94531-0.94531z"></path>
                </g>
                <circle cx="91.869" cy="17.161" r="16.933" class=move || palette().neutral></circle>
                <ellipse
                    cx="91.869"
                    cy="17.161"
                    rx="8.4667"
                    ry="16.933"
                    class=move || palette().lightest
                ></ellipse>
                <path
                    d="m87.635 38.327a4.2333 4.2333 0 0 0 4.2333 4.2333 4.2333 4.2333 0 0 0 4.2333-4.2333z"
                    class=move || palette().neutral
                ></path>
            </g>
        </g>
    }
}
