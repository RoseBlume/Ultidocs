pub const KEYWORDS: &[&str] = &[
    "SELECT","FROM","WHERE","INSERT","INTO","UPDATE",
    "DELETE","JOIN","LEFT","RIGHT","INNER","OUTER",
    "CREATE","TABLE","DROP","ALTER","AND","OR","NOT"
];

pub const SYMBOLS: &[&str] = &[
    ",",";","(",")","*","="
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);

#[cfg(not(debug_assertions))]
pub const LIGHT: &str = include_str!(
    concat!(
        env!("OUT_DIR"),
        "/light/langs/sql.css"
    )
);

#[cfg(debug_assertions)]
pub const LIGHT: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/light/langs/sql.css"
    )
);

#[cfg(not(debug_assertions))]
pub const DARK: &str = include_str!(
    concat!(
        env!("OUT_DIR"),
        "/dark/langs/sql.css"
    )
);

#[cfg(debug_assertions)]
pub const DARK: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/dark/langs/sql.css"
    )
);
