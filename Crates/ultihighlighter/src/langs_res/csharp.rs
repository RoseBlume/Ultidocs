pub const KEYWORDS: &[&str] = &[
    "using","namespace","class","public","private","protected",
    "static","void","int","string","bool","return","new",
    "if","else","foreach","while"
];

pub const SYMBOLS: &[&str] = &[
    "{","}","(",")","[","]",";","=>"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);

#[cfg(not(debug_assertions))]
pub const LIGHT: &str = include_str!(
    concat!(
        env!("OUT_DIR"),
        "/light/langs/c#.css"
    )
);

#[cfg(debug_assertions)]
pub const LIGHT: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/light/langs/c#.css"
    )
);

#[cfg(not(debug_assertions))]
pub const DARK: &str = include_str!(
    concat!(
        env!("OUT_DIR"),
        "/dark/langs/c#.css"
    )
);

#[cfg(debug_assertions)]
pub const DARK: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/dark/langs/c#.css"
    )
);
