use std::path::PathBuf;
use super::{Severity, Fix};

#[derive(Debug, Clone)]
pub struct LintError {
    pub file: Option<PathBuf>,
    pub line: usize,
    pub column: usize,
    pub severity: Severity,
    pub rule_id: &'static str,
    pub message: String,
    pub suggestion: Option<String>,
    pub fix: Option<Fix>,
}