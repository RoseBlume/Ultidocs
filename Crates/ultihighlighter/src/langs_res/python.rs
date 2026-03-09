pub const KEYWORDS: &[&str] = &[
    "def","class","import","from","return","if","elif","else",
    "for","while","try","except","True","False","None","with","as"
];

pub const SYMBOLS: &[&str] = &[
    ":", "(", ")", "[", "]"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);

#[cfg(not(debug_assertions))]
pub const LIGHT: &str = include_str!(
    concat!(
        env!("OUT_DIR"),
        "/light/langs/python.css"
    )
);

#[cfg(debug_assertions)]
pub const LIGHT: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/light/langs/python.css"
    )
);

#[cfg(not(debug_assertions))]
pub const DARK: &str = include_str!(
    concat!(
        env!("OUT_DIR"),
        "/dark/langs/python.css"
    )
);

#[cfg(debug_assertions)]
pub const DARK: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/dark/langs/python.css"
    )
);
