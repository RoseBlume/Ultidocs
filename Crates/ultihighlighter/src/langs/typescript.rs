pub const KEYWORDS: &[&str] = &[
    "function","let","const","var","return","if","else","class",
    "import","export","new","this","async","await",
    "interface","type","implements","public","private","readonly"
];

pub const SYMBOLS: &[&str] = &[
    "{","}","(",")","[","]",";","=>",":"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);