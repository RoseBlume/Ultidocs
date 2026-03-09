pub const KEYWORDS: &[&str] = &[
    "fn","let","mut","pub","impl","struct","enum","match","use",
    "crate","mod","return","trait","where","const","static","async","await"
];

pub const SYMBOLS: &[&str] = &[
    "{","}","(",")","[","]",";","::","->"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);