pub const KEYWORDS: &[&str] = &[
    "function","class","public","private","protected",
    "echo","return","if","else","foreach","while",
    "namespace","use","new"
];

pub const SYMBOLS: &[&str] = &[
    "{","}","(",")","[","]",";","=>","$"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);