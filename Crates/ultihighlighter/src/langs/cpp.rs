pub const KEYWORDS: &[&str] = &[
    "int","char","float","double","void","return",
    "if","else","for","while","class","struct",
    "public","private","protected","namespace",
    "using","new","delete","auto","template"
];

pub const SYMBOLS: &[&str] = &[
    "{","}","(",")","[","]",";","::","*","&"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);