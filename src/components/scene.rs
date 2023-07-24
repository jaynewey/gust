use crate::{components::hills::*, palette::Palette};
use leptos::*;

#[component]
pub fn Scene(cx: Scope) -> impl IntoView {
    let (palette, _) = use_context::<(ReadSignal<Palette<'_>>, WriteSignal<Palette<'_>>)>(cx)
        .expect("palette context");

    view! { cx,
        <div class=move || {
            format!(
                "fixed inset-0 -z-10 transition-all duration-300 overflow-y-auto overflow-x-hidden {}",
                palette().sky
            )
        }>
            <svg
                class="absolute inset-0 [&_*]:transition-all [&_*]:duration-300 [&_g]:transition-transform"
                overflow="visible"
                viewBox="0 0 1920 1080"
                height="100%"
            >
                <Back palette=palette/>
                <Middle palette=palette/>
                <Front palette=palette/>
            </svg>
        </div>
    }
}
