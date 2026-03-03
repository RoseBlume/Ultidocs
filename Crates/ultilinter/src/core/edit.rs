use std::ops::Range;

#[derive(Debug, Clone)]
pub struct TextEdit {
    pub range: Range<usize>,
    pub replacement: String,
}

#[derive(Debug, Clone)]
pub struct Fix {
    pub rule_id: &'static str,
    pub edits: Vec<TextEdit>,
}