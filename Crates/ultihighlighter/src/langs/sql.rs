pub const KEYWORDS: &[&str] = &[
    "SELECT","FROM","WHERE","INSERT","INTO","UPDATE",
    "DELETE","JOIN","LEFT","RIGHT","INNER","OUTER",
    "CREATE","TABLE","DROP","ALTER","AND","OR","NOT"
];

pub const SYMBOLS: &[&str] = &[
    ",",";","(",")","*","="
];

pub const LANG: (&[&str], &[&str]) = (KEYWORDS, SYMBOLS);