pub const KEYWORDS: &[&str] = &[
    "mov","add","sub","jmp","cmp","push","pop",
    "call","ret","section","global"
];

pub const SYMBOLS: &[&str] = &[
    ":",",","[","]"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);