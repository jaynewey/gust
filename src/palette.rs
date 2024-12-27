#[derive(Clone)]
pub struct Palette<'a> {
    pub sky: &'a str,
    pub background: &'a str,
    pub lightest: &'a str,
    pub light: &'a str,
    pub neutral: &'a str,
    pub dark: &'a str,
    pub darkest: &'a str,
    pub text: &'a str,
}

pub const DUSK_DAWN_SUNNY: Palette = Palette {
    sky: "bg-amber-100",
    background: "bg-amber-100/50",
    lightest: "fill-amber-200",
    light: "fill-amber-300",
    neutral: "fill-amber-500",
    dark: "fill-amber-600",
    darkest: "fill-amber-800",
    text: "text-amber-800",
};

pub const SNOW: Palette = Palette {
    sky: "bg-sky-50",
    background: "bg-sky-50/50",
    lightest: "fill-sky-100",
    light: "fill-sky-200",
    neutral: "fill-cyan-200",
    dark: "fill-sky-300",
    darkest: "fill-sky-400",
    text: "text-sky-600",
};

pub const THUNDER: Palette = Palette {
    sky: "bg-slate-700",
    background: "bg-slate-700/50",
    lightest: "fill-indigo-200",
    light: "fill-indigo-300",
    neutral: "fill-slate-400",
    dark: "fill-slate-500",
    darkest: "fill-slate-600",
    text: "text-indigo-100",
};

pub const RAIN: Palette = Palette {
    sky: "bg-slate-200",
    background: "bg-slate-200/50",
    lightest: "fill-slate-300",
    light: "fill-slate-400",
    neutral: "fill-slate-500",
    dark: "fill-slate-600",
    darkest: "fill-slate-700",
    text: "text-slate-700",
};

pub const CLEAR: Palette = Palette {
    sky: "bg-lime-50",
    background: "bg-lime-50/50",
    lightest: "fill-lime-100",
    light: "fill-yellow-200",
    neutral: "fill-lime-300",
    dark: "fill-lime-600",
    darkest: "fill-green-700",
    text: "text-green-700",
};

pub const DRIZZLE: Palette = Palette {
    sky: "bg-teal-100",
    background: "bg-teal-100/50",
    lightest: "fill-teal-200",
    light: "fill-teal-300",
    neutral: "fill-teal-600",
    dark: "fill-emerald-700",
    darkest: "fill-teal-800",
    text: "text-teal-800",
};

pub const DUSK_DAWN: Palette = Palette {
    sky: "bg-rose-50",
    background: "bg-rose-50/50",
    lightest: "fill-teal-100",
    light: "fill-red-200",
    neutral: "fill-teal-600",
    dark: "fill-red-500",
    darkest: "fill-emerald-700",
    text: "text-emerald-700",
};

pub const NIGHT_CLEAR: Palette = Palette {
    sky: "bg-indigo-900",
    background: "bg-indigo-900/50",
    lightest: "fill-violet-300",
    light: "fill-violet-400",
    neutral: "fill-indigo-500",
    dark: "fill-indigo-600",
    darkest: "fill-indigo-700",
    text: "text-violet-200",
};

pub const NIGHT_SNOW: Palette = Palette {
    sky: "bg-blue-900",
    background: "bg-blue-900/50",
    lightest: "fill-blue-300",
    light: "fill-blue-400",
    neutral: "fill-blue-500",
    dark: "fill-blue-600",
    darkest: "fill-blue-700",
    text: "text-blue-200",
};

pub const NIGHT_RAIN: Palette = Palette {
    sky: "bg-slate-900",
    background: "bg-slate-900/50",
    lightest: "fill-slate-400",
    light: "fill-slate-500",
    neutral: "fill-slate-600",
    dark: "fill-slate-700",
    darkest: "fill-slate-800",
    text: "text-slate-300",
};

pub const CLOUDY: Palette = Palette {
    sky: "bg-green-50",
    background: "bg-green-50/50",
    lightest: "fill-emerald-200",
    light: "fill-green-400",
    neutral: "fill-green-500",
    dark: "fill-green-700",
    darkest: "fill-emerald-800",
    text: "text-emerald-800",
};

pub const FOGGY: Palette = Palette {
    sky: "bg-stone-100",
    background: "bg-stone-100/50",
    lightest: "fill-zinc-300",
    light: "fill-stone-200",
    neutral: "fill-zinc-400",
    dark: "fill-stone-400",
    darkest: "fill-zinc-500",
    text: "text-zinc-600",
};

pub const NIGHT_FOGGY: Palette = Palette {
    sky: "bg-stone-600",
    background: "bg-stone-800/50",
    lightest: "fill-stone-400",
    light: "fill-stone-500",
    neutral: "fill-neutral-500",
    dark: "fill-stone-700",
    darkest: "fill-neutral-800",
    text: "text-neutral-300",
};
