use crate::{components::hills::*, palette::Palette, locations::Location};
use leptos::prelude::*;

#[component]
pub fn Scene(location: ReadSignal<Option<Location>>) -> impl IntoView {
    let (palette, _) = use_context::<(ReadSignal<Palette<'_>>, WriteSignal<Palette<'_>>)>()
        .expect("palette context");

    let (do_translate, set_do_translate) = signal(false);
    Effect::new(move |_| {
        let _ = location.get();
        set_do_translate.update(|state| *state = !*state);
    });

    view! {
        <div class=move || {
            format!(
                "fixed inset-0 -z-10 transition-all duration-1000 overflow-y-auto overflow-x-hidden {}",
                palette().sky,
            )
        }>
            <svg
                class="absolute inset-0 [&_*]:transition-all [&_*]:duration-1000 [&_g]:transition-transform [&_g]:duration-1000"
                overflow="visible"
                viewBox="0 0 1920 1080"
                height="100%"
            >
                <g class="!duration-1000 ease-in-out" class=("translate-x-10", do_translate)>
                    <Back palette=palette />
                </g>
                <Middle palette=palette />
                <g class="!duration-1000 ease-in-out" class=("-translate-x-10", do_translate)>
                    <Front palette=palette />
                </g>
            </svg>
        </div>
    }
}
