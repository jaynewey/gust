use crate::palette::Palette;
use leptos::prelude::*;

#[component]
pub fn Tree(
    scale: f32,
    translate: (f32, f32),
    palette: ReadSignal<Palette<'static>>,
) -> impl IntoView {
    let (x, y) = translate;
    view! {
        <g transform=format!("scale({}) translate({}, {})", scale, x, y)>
            <g transform="translate(78.083 -5.9194)">
                <rect
                    x="-72.262"
                    y="29.203"
                    width="1.0583"
                    height="12.7"
                    class=move || palette().darkest
                ></rect>
                <path
                    d="m-71.733 5.9194s-6.35 14.817-6.35 19.05a6.35 6.35 0 0 0 6.35 6.35v-6.35z"
                    class=move || palette().dark
                ></path>
                <path
                    d="m-71.733 5.9194s6.35 14.817 6.35 19.05a6.35 6.35 0 0 1-6.35 6.35v-6.35z"
                    class=move || palette().light
                ></path>
            </g>
        </g>
    }
}
