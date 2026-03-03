use std::fs;
use std::path::Path;

use crate::core::{TextEdit, LintReport};

#[derive(Debug)]
pub struct FixPlan {
    pub edits: Vec<TextEdit>,
}

impl FixPlan {
    pub fn from_report(report: &LintReport) -> Self {
        let mut edits = Vec::new();

        for error in &report.errors {
            if let Some(fix) = &error.fix {
                edits.extend(fix.edits.clone());
            }
        }

        edits.sort_by(|a, b| {
            a.range.start
                .cmp(&b.range.start)
                .then(a.range.end.cmp(&b.range.end))
        });

        let mut filtered = Vec::new();
        let mut last_end = 0;

        for edit in edits {
            if edit.range.start >= last_end {
                last_end = edit.range.end;
                filtered.push(edit);
            }
        }

        Self { edits: filtered }
    }

    pub fn apply(&self, source: &str) -> String {
        let mut result = source.to_string();

        for edit in self.edits.iter().rev() {
            result.replace_range(edit.range.clone(), &edit.replacement);
        }

        result
    }
}

pub fn apply_fixes_to_string(report: &LintReport, source: &str) -> String {
    FixPlan::from_report(report).apply(source)
}

pub fn apply_fixes_to_file(path: &Path, report: &LintReport) -> std::io::Result<()> {
    let source = fs::read_to_string(path)?;
    let fixed = apply_fixes_to_string(report, &source);
    fs::write(path, fixed)
}