pub const KEYWORDS: &[&str] = &[
    "using","namespace","class","public","private","protected",
    "static","void","int","string","bool","return","new",
    "if","else","foreach","while"
];

pub const SYMBOLS: &[&str] = &[
    "{","}","(",")","[","]",";","=>"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);