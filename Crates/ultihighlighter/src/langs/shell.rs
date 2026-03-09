pub const KEYWORDS: &[&str] = &[
    "if","then","else","elif","fi",
    "for","while","do","done",
    "function","in","case","esac",
    "return","break","continue","exit",
    "export","readonly","local"
];

pub const SYMBOLS: &[&str] = &[
    "|","&","&&","||","!",";","{","}","(",")","[","]","<",">","="
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);