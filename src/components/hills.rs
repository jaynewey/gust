use crate::components::{balloon::Balloon, houses::*, trees::*};
use crate::palette::Palette;
use leptos::prelude::*;

#[component]
pub fn Back(palette: ReadSignal<Palette<'static>>) -> impl IntoView {
    view! {
        <Balloon scale=1.6 translate=(75.0, 50.0) palette=palette />
        <Balloon scale=1.7 translate=(300.0, 250.0) palette=palette />
        <Balloon scale=1.7 translate=(775.0, 175.0) palette=palette />
        <Balloon scale=1.5 translate=(1200.0, 100.0) palette=palette />
        <House scale=1.6 translate=(200.0, 350.0) palette=palette />
        <House scale=1.5 translate=(720.0, 410.0) palette=palette />
        <House scale=1.5 translate=(1170.0, 360.0) palette=palette />
        <LargeHouse scale=1.0 translate=(1270.0, 575.0) palette=palette />
        <Tree scale=1.5 translate=(1120.0, 355.0) palette=palette />
        <Tree scale=1.3 translate=(30.0, 485.0) palette=palette />
        <Tree scale=1.3 translate=(500.0, 475.0) palette=palette />
        <Tree scale=1.5 translate=(1000.0, 410.0) palette=palette />
        <g class=move || palette().darkest>
            <circle cx="1344" cy="800" r="160"></circle>
            <circle cx="992" cy="736" r="128"></circle>
            <circle cx="1200" cy="768" r="128"></circle>
        </g>
        <g class=move || palette().darkest>
            <circle cx="2240" cy="800" r="160"></circle>
            <circle cx="1904" cy="672" r="128"></circle>
            <circle cx="2048" cy="704" r="128"></circle>
        </g>
        <g class=move || palette().light>
            <circle cx="832" cy="784" r="128"></circle>
            <circle cx="1024" cy="832" r="128"></circle>
            <circle cx="1184" cy="880" r="128"></circle>
            <circle cx="576" cy="848" r="160"></circle>
        </g>
        <g class=move || palette().dark>
            <circle cx="416" cy="768" r="160"></circle>
            <circle cx="592" cy="768" r="128"></circle>
            <circle cx="768" cy="800" r="128"></circle>
            <circle cx="32" cy="800" r="128"></circle>
            <circle cx="192" cy="816" r="128"></circle>
        </g>
        <g class=move || palette().neutral>
            <circle cx="1728" cy="704" r="128"></circle>
            <circle cx="1920" cy="768" r="128"></circle>
            <circle cx="1344" cy="832" r="128"></circle>
            <circle cx="1552" cy="816" r="160"></circle>
        </g>
    }
}

#[component]
pub fn Middle(palette: ReadSignal<Palette<'static>>) -> impl IntoView {
    view! {
        <House scale=1.8 translate=(320.0, 380.0) palette=palette />
        <House scale=1.8 translate=(720.0, 410.0) palette=palette />
        <LargeHouse scale=1.3 translate=(30.0, 530.0) palette=palette />
        <LargeHouse scale=1.3 translate=(740.0, 570.0) palette=palette />
        <Tree scale=2.4 translate=(740.0, 270.0) palette=palette />
        <g class=move || palette().lightest>
            <circle cx="1536" cy="960" r="160"></circle>
            <circle cx="1728" cy="832" r="128"></circle>
            <circle cx="1856" cy="960" r="128"></circle>
            <circle cx="1200" cy="880" r="128"></circle>
            <circle cx="1392" cy="912" r="128"></circle>
        </g>
        <g class=move || palette().light>
            <circle cx="320" cy="848" r="128"></circle>
            <circle cx="512" cy="896" r="128"></circle>
            <circle cx="704" cy="960" r="128"></circle>
            <circle cx="96" cy="928" r="160"></circle>
        </g>
        <g class=move || palette().darkest>
            <circle cx="800" cy="880" r="160"></circle>
            <circle cx="960" cy="944" r="128"></circle>
            <circle cx="432" cy="944" r="128"></circle>
            <circle cx="608" cy="880" r="128"></circle>
        </g>
        <g class=move || palette().dark>
            <circle cx="1088" cy="992" r="160"></circle>
            <circle cx="1248" cy="960" r="128"></circle>
            <circle cx="1440" cy="1024" r="128"></circle>
            <circle cy="960" r="128"></circle>
            <circle cx="864" cy="1040" r="128"></circle>
        </g>
        <Tree scale=2.2 translate=(340.0, 300.0) palette=palette />
        <g class=move || palette().dark>
            <circle cx="2208" cy="912" r="160"></circle>
            <circle cy="960" r="128"></circle>
            <circle cx="2032" cy="848" r="128"></circle>
        </g>
    }
}

#[component]
pub fn Front(palette: ReadSignal<Palette<'static>>) -> impl IntoView {
    view! {
        <House scale=2.0 translate=(600.0, 410.0) palette=palette />
        <Tree scale=2.2 translate=(650.0, 345.0) palette=palette />
        <Tree scale=2.4 translate=(690.0, 340.0) palette=palette />
        <g class=move || palette().neutral>
            <circle cx="1472" cy="960" r="128"></circle>
            <circle cx="1664" cy="1024" r="128"></circle>
            <circle cx="1856" cy="1072" r="128"></circle>
            <circle cx="1248" cy="1056" r="160"></circle>
        </g>
        <Tree scale=2.4 translate=(350.0, 375.0) palette=palette />
        <Tree scale=2.6 translate=(140.0, 290.0) palette=palette />
        <g class=move || palette().light>
            <circle cx="896" cy="1136" r="160"></circle>
            <circle cx="1136" cy="1088" r="128"></circle>
        </g>
        <g class=move || palette().light>
            <circle cx="2016" cy="1024" r="160"></circle>
            <circle cx="2256" cy="976" r="128"></circle>
            <circle cx="2176" cy="1120" r="128"></circle>
        </g>
        <House scale=2.2 translate=(750.0, 430.0) palette=palette />
        <g class=move || palette().darkest>
            <circle cx="1536" cy="1088" r="128"></circle>
            <circle cx="1728" cy="1152" r="128"></circle>
            <circle cx="1920" cy="1088" r="128"></circle>
            <circle cx="1312" cy="1184" r="160"></circle>
            <circle cx="2048" cy="1184" r="160"></circle>
        </g>
        <g class=move || palette().lightest>
            <circle cx="352" cy="960" r="128"></circle>
            <circle cx="512" cy="1024" r="128"></circle>
            <circle cx="704" cy="1072" r="128"></circle>
            <circle cx="128" cy="1056" r="160"></circle>
        </g>
        <House scale=2.0 translate=(50.0, 475.0) palette=palette />
        <Tree scale=2.6 translate=(120.0, 330.0) palette=palette />
        <g class=move || palette().darkest>
            <circle cx="368" cy="1088" r="160"></circle>
            <circle cx="592" cy="1056" r="128"></circle>
            <circle cx="768" cy="1152" r="128"></circle>
            <circle cy="1088" r="128"></circle>
            <circle cx="192" cy="1152" r="128"></circle>
        </g>
    }
}
