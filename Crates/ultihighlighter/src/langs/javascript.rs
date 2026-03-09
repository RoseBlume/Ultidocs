pub const KEYWORDS: &[&str] = &[
    "function","let","const","var","return","if","else","class",
    "import","export","new","this","async","await","try","catch"
];

pub const SYMBOLS: &[&str] = &[
    "{","}","(",")","[","]",";","=>"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);