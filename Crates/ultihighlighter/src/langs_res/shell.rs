pub const KEYWORDS: &[&str] = &[
    "if","then","else","elif","fi",
    "for","while","do","done",
    "function","in","case","esac",
    "return","break","continue","exit",
    "export","readonly","local"
];

pub const SYMBOLS: &[&str] = &[
    "|","&","&&","||","!",";","{","}","(",")","[","]","<",">","="
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);

#[cfg(not(debug_assertions))]
pub const LIGHT: &str = include_str!(
    concat!(
        env!("OUT_DIR"),
        "/light/langs/shell.css"
    )
);

#[cfg(debug_assertions)]
pub const LIGHT: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/light/langs/shell.css"
    )
);

#[cfg(not(debug_assertions))]
pub const DARK: &str = include_str!(
    concat!(
        env!("OUT_DIR"),
        "/dark/langs/shell.css"
    )
);

#[cfg(debug_assertions)]
pub const DARK: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/dark/langs/shell.css"
    )
);
