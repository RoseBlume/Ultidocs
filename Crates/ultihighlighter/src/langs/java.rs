pub const KEYWORDS: &[&str] = &[
    "class","public","private","protected","static","void",
    "int","double","boolean","new","return",
    "if","else","for","while","import","package"
];

pub const SYMBOLS: &[&str] = &[
    "{","}","(",")","[","]",";"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);