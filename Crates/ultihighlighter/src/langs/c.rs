pub const KEYWORDS: &[&str] = &[
    "int","char","float","double","void","return",
    "if","else","for","while","struct","typedef",
    "include"
];

pub const SYMBOLS: &[&str] = &[
    "{","}","(",")","[","]",";","*","&","#"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);