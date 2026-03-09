pub const KEYWORDS: &[&str] = &[
    "echo","set","if","else","for","in","do","goto",
    "call","exit","rem","pause","shift","title","cd",
    "mkdir","rmdir","copy","move","del","type"
];

pub const SYMBOLS: &[&str] = &[
    "<",">",">>","|","&","&&","||","%","(",")","!"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);

#[cfg(not(debug_assertions))]
pub const LIGHT: &str = include_str!(
    concat!(
        env!("OUT_DIR"),
        "/light/langs/batch.css"
    )
);

#[cfg(debug_assertions)]
pub const LIGHT: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/light/langs/batch.css"
    )
);

#[cfg(not(debug_assertions))]
pub const DARK: &str = include_str!(
    concat!(
        env!("OUT_DIR"),
        "/dark/langs/batch.css"
    )
);

#[cfg(debug_assertions)]
pub const DARK: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/dark/langs/batch.css"
    )
);
