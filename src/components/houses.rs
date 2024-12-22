use crate::palette::Palette;
use leptos::prelude::*;

#[component]
pub fn House(
    scale: f32,
    translate: (f32, f32),
    palette: ReadSignal<Palette<'static>>,
) -> impl IntoView {
    let (x, y) = translate;
    view! {
        <g transform=format!("scale({}) translate({}, {})", scale, x, y)>
            <g transform="translate(-104.28 58.12)">
                <path
                    d="m104.28-7.3203v-42.333l8.4667-8.4667 8.4667 8.4667v42.333z"
                    class=move || palette().darkest
                ></path>
                <rect
                    x="121.21"
                    y="-49.654"
                    width="21.167"
                    height="42.333"
                    class=move || palette().lightest
                ></rect>
                <path
                    d="m112.75-58.12 8.4667 8.4667h21.167l-8.4667-8.4667z"
                    class=move || palette().light
                ></path>
                <g class=move || palette().neutral>
                    <circle cx="127.56" cy="-43.304" r="2.1167"></circle>
                    <rect x="125.45" y="-43.304" width="4.2333" height="4.2333"></rect>
                </g>
                <g transform="translate(8.4667)">
                    <g class=move || palette().neutral>
                        <circle cx="127.56" cy="-43.304" r="2.1167"></circle>
                        <rect x="125.45" y="-43.304" width="4.2333" height="4.2333"></rect>
                    </g>
                </g>
                <g transform="translate(7.2166e-6 10.583)">
                    <g class=move || palette().neutral>
                        <circle cx="127.56" cy="-43.304" r="2.1167"></circle>
                        <rect x="125.45" y="-43.304" width="4.2333" height="4.2333"></rect>
                    </g>
                </g>
                <g transform="translate(8.4667 10.583)">
                    <g class=move || palette().neutral>
                        <circle cx="127.56" cy="-43.304" r="2.1167"></circle>
                        <rect x="125.45" y="-43.304" width="4.2333" height="4.2333"></rect>
                    </g>
                </g>
            </g>
        </g>
    }
}

#[component]
pub fn LargeHouse(
    scale: f32,
    translate: (f32, f32),
    palette: ReadSignal<Palette<'static>>,
) -> impl IntoView {
    let (x, y) = translate;
    view! {
        <g transform=format!("scale({}) translate({}, {})", scale, x, y)>
            <g transform="translate(43.249 91.414)">
                <rect
                    x="-.91582"
                    y="-74.481"
                    width="59.267"
                    height="67.733"
                    class=move || palette().lightest
                ></rect>
                <path
                    d="m-34.782-6.7472v-67.733l16.933-16.933 16.933 16.933v67.733z"
                    class=move || palette().darkest
                ></path>
                <path
                    d="m-17.849-91.414 16.933 16.933h59.267l-16.933-16.933z"
                    class=move || palette().light
                ></path>
                <g
                    transform="matrix(.26458 0 0 .26458 -102.52 -159.15)"
                    class=move || palette().neutral
                >
                    <circle cx="432" cy="368" r="16"></circle>
                    <rect x="416" y="368" width="32" height="32"></rect>
                </g>
                <g
                    transform="matrix(.26458 0 0 .26458 -85.582 -159.15)"
                    class=move || palette().neutral
                >
                    <circle cx="432" cy="368" r="16"></circle>
                    <rect x="416" y="368" width="32" height="32"></rect>
                </g>
                <g
                    transform="matrix(.26458 0 0 .26458 -68.649 -159.15)"
                    class=move || palette().neutral
                >
                    <circle cx="432" cy="368" r="16"></circle>
                    <rect x="416" y="368" width="32" height="32"></rect>
                </g>
                <g
                    transform="matrix(.26458 0 0 .26458 -68.649 -137.98)"
                    class=move || palette().neutral
                >
                    <circle cx="432" cy="368" r="16"></circle>
                    <rect x="416" y="368" width="32" height="32"></rect>
                </g>
                <g
                    transform="matrix(.26458 0 0 .26458 -85.582 -137.98)"
                    class=move || palette().neutral
                >
                    <circle cx="432" cy="368" r="16"></circle>
                    <rect x="416" y="368" width="32" height="32"></rect>
                </g>
                <g
                    transform="matrix(.26458 0 0 .26458 -102.52 -137.98)"
                    class=move || palette().neutral
                >
                    <circle cx="432" cy="368" r="16"></circle>
                    <rect x="416" y="368" width="32" height="32"></rect>
                </g>
                <rect
                    x="-26.316"
                    y="-70.247"
                    width="16.933"
                    height="63.5"
                    class=move || palette().lightest
                ></rect>
                <path
                    d="m-26.316-6.7472h-16.933v-63.5l8.4667-8.4667 8.4667 8.4667z"
                    class=move || palette().darkest
                ></path>
                <path
                    d="m-34.782-78.714 8.4667 8.4667h16.933l-8.4667-8.4667z"
                    class=move || palette().light
                ></path>
            </g>
        </g>
    }
}
