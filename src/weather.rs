use leptos_icons::{Icon, LuIcon};

pub fn weather_icon(weathercode: u32, is_day: bool) -> Icon {
    match weathercode {
        0 => {
            if is_day {
                Icon::from(LuIcon::LuSun)
            } else {
                Icon::from(LuIcon::LuMoon)
            }
        }
        1 => {
            if is_day {
                Icon::from(LuIcon::LuCloudSun)
            } else {
                Icon::from(LuIcon::LuCloudMoon)
            }
        }
        2 => Icon::from(LuIcon::LuCloud),
        3 => Icon::from(LuIcon::LuCloudy),
        45 | 48 => Icon::from(LuIcon::LuCloudFog),
        51 | 53 | 55 => Icon::from(LuIcon::LuCloudDrizzle),
        61 | 63 | 80 | 81 => Icon::from(LuIcon::LuCloudRain),
        65 | 82 => Icon::from(LuIcon::LuCloudRainWind),
        56 | 57 | 66 | 67 => Icon::from(LuIcon::LuCloudHail),
        71 | 73 | 75 | 77 | 85 | 86 => Icon::from(LuIcon::LuCloudSnow),
        95 | 96 | 99 => Icon::from(LuIcon::LuCloudLightning),
        _ => Icon::from(LuIcon::LuThermometer),
    }
}

pub fn weather_description(weathercode: u32, is_day: bool) -> &'static str {
    match weathercode {
        0 => {
            if is_day {
                "Sunny skies"
            } else {
                "Clear skies"
            }
        }
        1 => {
            if is_day {
                "Mostly sunny skies"
            } else {
                "Mostly clear skies"
            }
        }
        2 => "Partly cloudy",
        3 => "Overcast",
        45 => "Foggy",
        48 => "Foggy, depositing rime",
        51 => "Light drizzle",
        53 => "Moderate drizzle",
        55 => "Dense drizzle",
        56 => "Light freezing drizzle",
        57 => "Heavy freezing drizzle",
        61 => "Light rain",
        63 => "Moderate rain",
        65 => "Heavy rain",
        66 => "Light freezing rain",
        67 => "Heavy freezing rain",
        71 => "Light snow fall",
        73 => "Moderate snow fall",
        75 => "Heavy snow fall",
        77 => "Snow grains",
        80 => "Light showers",
        81 => "Moderate showers",
        82 => "Violent showers",
        85 => "Light snow showers",
        86 => "Heavy snow showers",
        95 => "Thunderstorms",
        96 => "Thunderstorms with slight hail",
        99 => "Thunderstorms with heavy hail",
        _ => "",
    }
}

pub fn wind_direction_icon(wind_direction: u32) -> Icon {
    let wind_direction = &(wind_direction as f64);
    if (0.0..22.5).contains(wind_direction) {
        Icon::from(LuIcon::LuArrowUp)
    } else if (22.5..67.5).contains(wind_direction) {
        Icon::from(LuIcon::LuArrowUpRight)
    } else if (67.5..112.5).contains(wind_direction) {
        Icon::from(LuIcon::LuArrowRight)
    } else if (112.5..157.5).contains(wind_direction) {
        Icon::from(LuIcon::LuArrowDownRight)
    } else if (157.5..202.5).contains(wind_direction) {
        Icon::from(LuIcon::LuArrowDown)
    } else if (202.5..247.5).contains(wind_direction) {
        Icon::from(LuIcon::LuArrowDownLeft)
    } else if (202.5..292.5).contains(wind_direction) {
        Icon::from(LuIcon::LuArrowLeft)
    } else if (292.5..347.5).contains(wind_direction) {
        Icon::from(LuIcon::LuArrowUpLeft)
    } else {
        Icon::from(LuIcon::LuArrowUp)
    }
}
