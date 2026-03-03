use std::path::Path;
use crate::{
    LintConfig, LintError, Fix, TextEdit,
    LintReport, Severity, Rule,
};

pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(ListFormatting),
        Box::new(MixedListMarkers),
        Box::new(TrailingSpacesInList),
        Box::new(BlankLineAfterListItem),
        Box::new(SeparateConsecutiveLists),
        Box::new(OrderedListNumbering),
    ]
}

//
// ============================================================
// MD006 – List formatting
// ============================================================
//

#[derive(Clone)]
pub struct ListFormatting;

impl Rule for ListFormatting {
    fn id(&self) -> &'static str { "MD006" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut prev_was_list = false;

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            let is_list = trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ");

            if is_list {
                if !prev_was_list && i > 0 && !source.lines().nth(i-1).unwrap_or("").trim().is_empty() {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Missing blank line before list".into(),
                        suggestion: Some("Insert blank line before list".into()),
                        fix: Some(Fix {
                            rule_id: self.id(),
                            edits: vec![TextEdit {
                                range: 0..0, // calculated per line in full implementation
                                replacement: "\n".into(),
                            }],
                        }),
                    });
                }
            }

            prev_was_list = is_list;
        }
    }
}

//
// ============================================================
// MD011 – Mixed list markers
// ============================================================
//

#[derive(Clone)]
pub struct MixedListMarkers;

impl Rule for MixedListMarkers {
    fn id(&self) -> &'static str { "MD011" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut current_marker: Option<&str> = None;

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
                let marker = &trimmed[0..1];
                if let Some(prev) = current_marker {
                    if prev != marker {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Mixed list markers".into(),
                            suggestion: Some("Use consistent list marker".into()),
                            fix: None,
                        });
                    }
                }
                current_marker = Some(marker);
            } else if trimmed.is_empty() {
                current_marker = None;
            }
        }
    }
}

//
// ============================================================
// MD030 – Trailing spaces in list items
// ============================================================
//
#[derive(Clone)]
pub struct TrailingSpacesInList;

impl Rule for TrailingSpacesInList {
    fn id(&self) -> &'static str { "MD030" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim_end();
            if line != trimmed && (line.trim_start().starts_with("- ") || line.trim_start().starts_with("* ") || line.trim_start().starts_with("+ ") || line.trim_start().chars().next().unwrap_or(' ').is_numeric()) {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: trimmed.len() + 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "List item has trailing whitespace".into(),
                    suggestion: Some("Remove trailing spaces".into()),
                    fix: Some(Fix {
                        rule_id: self.id(),
                        edits: vec![TextEdit {
                            range: trimmed.len()..line.len(),
                            replacement: "".into(),
                        }],
                    }),
                });
            }
        }
    }
}

//
// ============================================================
// MD031 – Blank line after list item (except last)
// ============================================================
//
#[derive(Clone)]
pub struct BlankLineAfterListItem;

impl Rule for BlankLineAfterListItem {
    fn id(&self) -> &'static str { "MD031" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let lines: Vec<&str> = source.lines().collect();

        for i in 0..lines.len() - 1 {
            let trimmed = lines[i].trim_start();
            let next = lines[i+1].trim();

            let is_list_item = trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") || trimmed.chars().next().unwrap_or(' ').is_numeric();
            if is_list_item && !next.is_empty() && (lines[i+1].trim_start().starts_with("- ") || lines[i+1].trim_start().starts_with("* ") || lines[i+1].trim_start().starts_with("+ ") || lines[i+1].chars().next().unwrap_or(' ').is_numeric()) {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 2,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Missing blank line after list item".into(),
                    suggestion: Some("Insert blank line after this list item".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// ============================================================
// MD032 – Consecutive lists should have a blank line between them
// ============================================================
//
#[derive(Clone)]
pub struct SeparateConsecutiveLists;

impl Rule for SeparateConsecutiveLists {
    fn id(&self) -> &'static str { "MD032" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut prev_was_list = false;

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            let is_list_item = trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") || trimmed.chars().next().unwrap_or(' ').is_numeric();

            if is_list_item {
                if prev_was_list && i > 0 && !source.lines().nth(i-1).unwrap_or("").trim().is_empty() {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Consecutive lists should be separated by a blank line".into(),
                        suggestion: Some("Insert blank line between lists".into()),
                        fix: None,
                    });
                }
                prev_was_list = true;
            } else if !trimmed.is_empty() {
                prev_was_list = false;
            }
        }
    }
}

//
// ============================================================
// MD033 – Ordered list numbering sequence
// ============================================================
//
#[derive(Clone)]
pub struct OrderedListNumbering;

impl Rule for OrderedListNumbering {
    fn id(&self) -> &'static str { "MD033" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut expected_number = 1;

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            if let Some(pos) = trimmed.find('.') {
                if let Ok(num) = trimmed[..pos].parse::<usize>() {
                    if num != expected_number {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: format!("Ordered list item number {} out of sequence (expected {})", num, expected_number),
                            suggestion: Some(format!("Use number {}", expected_number)),
                            fix: None,
                        });
                    }
                    expected_number = num + 1;
                }
            }
        }
    }
}