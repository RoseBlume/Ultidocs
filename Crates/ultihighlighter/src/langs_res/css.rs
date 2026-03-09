pub const KEYWORDS: &[&str] = &[
    "color","background","display","position","absolute","relative",
    "flex","grid","margin","padding","font","border"
];

pub const SYMBOLS: &[&str] = &[
    "{","}",";",":",".","#"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);

#[cfg(not(debug_assertions))]
pub const LIGHT: &str = include_str!(
    concat!(
        env!("OUT_DIR"),
        "/light/langs/css.css"
    )
);

#[cfg(debug_assertions)]
pub const LIGHT: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/light/langs/css.css"
    )
);

#[cfg(not(debug_assertions))]
pub const DARK: &str = include_str!(
    concat!(
        env!("OUT_DIR"),
        "/dark/langs/css.css"
    )
);

#[cfg(debug_assertions)]
pub const DARK: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/dark/langs/css.css"
    )
);
