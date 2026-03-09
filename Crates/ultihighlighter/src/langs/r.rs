pub const KEYWORDS: &[&str] = &[
    "function","if","else","for","while",
    "TRUE","FALSE","NULL","library","return"
];

pub const SYMBOLS: &[&str] = &[
    "{","}","(",")","[","]","<-"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);