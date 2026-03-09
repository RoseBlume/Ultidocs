pub const KEYWORDS: &[&str] = &[
    "echo","set","if","else","for","in","do","goto",
    "call","exit","rem","pause","shift","title","cd",
    "mkdir","rmdir","copy","move","del","type"
];

pub const SYMBOLS: &[&str] = &[
    "<",">",">>","|","&","&&","||","%","(",")","!"
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);