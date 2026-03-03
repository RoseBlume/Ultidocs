use std::path::Path;
use crate::helpers::line_starts;
use crate::{
    LintConfig,
    LintError,
    Fix,
    TextEdit,
    LintReport,
    Severity,
    Rule,
    Linter
};

pub fn linter() -> Linter {
    Linter::new()
        .add_rule(Box::new(TrailingWhitespace))
        .add_rule(Box::new(MissingDoctype))
        .add_rule(Box::new(LowercaseTags))
        .add_rule(Box::new(EmptyAltAttribute))
        .add_rule(Box::new(MissingClosingTag))
        .add_rule(Box::new(MultipleSpacesBetweenAttributes))
}



//
// ============================================================
// HTML001 – Trailing whitespace
//

#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "HTML001" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }
        let starts = line_starts(source);
        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim_end();
            if trimmed.len() != line.len() {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: trimmed.len() + 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Trailing whitespace".into(),
                    suggestion: Some("Remove trailing whitespace".into()),
                    fix: Some(Fix {
                        rule_id: self.id(),
                        edits: vec![TextEdit { range: starts[i]+trimmed.len()..starts[i]+line.len(), replacement: String::new() }]
                    }),
                });
            }
        }
    }
}

//
// HTML002 – Missing <!DOCTYPE html>
//

#[derive(Clone)]
struct MissingDoctype;

impl Rule for MissingDoctype {
    fn id(&self) -> &'static str { "HTML002" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }
        if !source.to_lowercase().starts_with("<!doctype html>") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Missing <!DOCTYPE html>".into(),
                suggestion: Some("Add <!DOCTYPE html> at the top".into()),
                fix: Some(Fix { rule_id: self.id(), edits: vec![TextEdit { range: 0..0, replacement: "<!DOCTYPE html>\n".into() }] }),
            });
        }
    }
}

//
// HTML003 – Lowercase tags
//

#[derive(Clone)]
struct LowercaseTags;

impl Rule for LowercaseTags {
    fn id(&self) -> &'static str { "HTML003" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, _config: &LintConfig) {
        for (idx, _) in source.match_indices(|c| c == '<') {
            let slice = &source[idx+1..];
            if let Some(end) = slice.find(|c: char| c.is_whitespace() || c == '>') {
                let name = &slice[..end];
                if name.chars().any(|c| c.is_uppercase()) {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: 0,
                        column: 0,
                        severity: Severity::Info,
                        rule_id: self.id(),
                        message: "HTML tag should be lowercase".into(),
                        suggestion: Some("Convert tag name to lowercase".into()),
                        fix: Some(Fix { rule_id: self.id(), edits: vec![TextEdit { range: idx+1..idx+1+end, replacement: name.to_lowercase() }] }),
                    });
                }
            }
        }
    }
}

//
// HTML004 – <img> with empty alt
//

#[derive(Clone)]
struct EmptyAltAttribute;

impl Rule for EmptyAltAttribute {
    fn id(&self) -> &'static str { "HTML004" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, _config: &LintConfig) {
        for (i, line) in source.lines().enumerate() {
            if line.contains("<img") && line.contains("alt=\"\"") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "<img> has empty alt attribute".into(),
                    suggestion: Some("Provide meaningful alt text".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// HTML005 – Missing closing tag (simplistic)
//

#[derive(Clone)]
struct MissingClosingTag;

impl Rule for MissingClosingTag {
    fn id(&self) -> &'static str { "HTML005" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        _config: &LintConfig,
    ) {
        let mut stack: Vec<String> = vec![];

        for (_, line) in source.lines().enumerate() {
            let mut offset = 0;
            while let Some(start) = line[offset..].find('<') {
                let tag_start = offset + start;

                // find '>' relative to the whole line
                if let Some(rel_end) = line[tag_start..].find('>') {
                    let tag_end = tag_start + rel_end;

                    let tag = &line[tag_start + 1..tag_end];
                    let tag_trim = tag.trim();

                    if tag_trim.starts_with('/') {
                        stack.pop();
                    } else if !tag_trim.ends_with('/') {
                        stack.push(tag_trim.to_string());
                    }

                    offset = tag_end + 1; // move past this tag
                } else {
                    break; // no closing '>' found
                }
            }
        }

        if !stack.is_empty() {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Potential missing closing tag".into(),
                suggestion: Some("Check your HTML tags".into()),
                fix: None,
            });
        }
    }
}

//
// HTML006 – Multiple spaces between attributes
//

#[derive(Clone)]
struct MultipleSpacesBetweenAttributes;

impl Rule for MultipleSpacesBetweenAttributes {
    fn id(&self) -> &'static str { "HTML006" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, _config: &LintConfig) {
        for (i, line) in source.lines().enumerate() {
            if line.contains("  ") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Multiple spaces between attributes".into(),
                    suggestion: Some("Reduce to single space".into()),
                    fix: None,
                });
            }
        }
    }
}