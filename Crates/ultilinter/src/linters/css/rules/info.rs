use std::path::Path;
use crate::helpers::line_starts;
use crate::{
    LintConfig, LintError, Fix, TextEdit,
    LintReport, Severity, Rule,
};


pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(TrailingWhitespace),
        Box::new(UppercaseHexColor),
        Box::new(InlineBlockStyle),
        Box::new(ZeroWithUnit),
        Box::new(MultipleBlankLines),
        Box::new(LeadingZeroDecimal),
        Box::new(UniversalSelector),
    ]
}

//
// ============================================================
// CSS001 – Trailing whitespace
// ============================================================
//

#[derive(Clone)]
pub struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "CSS001" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig
    ) {
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
                    suggestion: Some("Remove trailing spaces".into()),
                    fix: Some(Fix {
                        rule_id: self.id(),
                        edits: vec![TextEdit {
                            range: starts[i] + trimmed.len()..starts[i] + line.len(),
                            replacement: String::new(),
                        }],
                    }),
                });
            }
        }
    }
}

//
// ============================================================
// CSS004 – Uppercase hex color
// ============================================================
//

#[derive(Clone)]
pub struct UppercaseHexColor;

impl Rule for UppercaseHexColor {
    fn id(&self) -> &'static str { "CSS004" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig
    ) {
        if !config.is_enabled(self.id()) { return; }

        for (idx, _) in source.match_indices('#') {
            let slice = &source[idx..];
            if slice.len() >= 7 {
                let hex = &slice[1..7];
                if hex.chars().any(|c| c.is_ascii_uppercase()) {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: 0,
                        column: 0,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Hex color should be lowercase".into(),
                        suggestion: Some("Use lowercase hex".into()),
                        fix: Some(Fix {
                            rule_id: self.id(),
                            edits: vec![TextEdit {
                                range: idx + 1..idx + 7,
                                replacement: hex.to_lowercase(),
                            }],
                        }),
                    });
                }
            }
        }
    }
}

//
// ============================================================
// CSS006 – Inline block style
// ============================================================
//

#[derive(Clone)]
pub struct InlineBlockStyle;

impl Rule for InlineBlockStyle {
    fn id(&self) -> &'static str { "CSS006" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        let mut offset = 0;

        for (line_index, line) in source.lines().enumerate() {
            let len = line.len();

            if let (Some(open), Some(close)) = (line.find('{'), line.rfind('}')) {
                if open < close {
                    let indent: String =
                        line.chars().take_while(|c| c.is_whitespace()).collect();
                    let selector = line[..open].trim_end();
                    let body = line[open + 1..close].trim();

                    if !body.is_empty() {
                        let formatted = format!(
                            "{indent}{selector} {{\n{indent}    {body}\n{indent}}}"
                        );

                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: line_index + 1,
                            column: 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Avoid single-line block declarations".into(),
                            suggestion: Some("Use multi-line block".into()),
                            fix: Some(Fix {
                                rule_id: self.id(),
                                edits: vec![TextEdit {
                                    range: offset..offset + len,
                                    replacement: formatted,
                                }],
                            }),
                        });
                    }
                }
            }

            offset += len + 1;
        }
    }
}

//
// ============================================================
// CSS007 – Zero with unit
// ============================================================
//

#[derive(Clone)]
pub struct ZeroWithUnit;

impl Rule for ZeroWithUnit {
    fn id(&self) -> &'static str { "CSS007" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig
    ) {
        if !config.is_enabled(self.id()) { return; }

        for (idx, _) in source.match_indices("0px") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Zero should not have unit".into(),
                suggestion: Some("Use '0' instead of '0px'".into()),
                fix: Some(Fix {
                    rule_id: self.id(),
                    edits: vec![TextEdit {
                        range: idx..idx + 3,
                        replacement: "0".into(),
                    }],
                }),
            });
        }
    }
}

//
// ============================================================
// CSS008 – Multiple blank lines
// ============================================================
//

#[derive(Clone)]
pub struct MultipleBlankLines;

impl Rule for MultipleBlankLines {
    fn id(&self) -> &'static str { "CSS008" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig
    ) {
        if !config.is_enabled(self.id()) { return; }

        let mut previous_blank = false;

        for (i, line) in source.lines().enumerate() {
            let blank = line.trim().is_empty();

            if blank && previous_blank {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Multiple consecutive blank lines".into(),
                    suggestion: Some("Reduce to a single blank line".into()),
                    fix: None,
                });
            }

            previous_blank = blank;
        }
    }
}

//
// ============================================================
// CSS009 – Leading zero in decimal
// ============================================================
//

#[derive(Clone)]
pub struct LeadingZeroDecimal;

impl Rule for LeadingZeroDecimal {
    fn id(&self) -> &'static str { "CSS009" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig
    ) {
        if !config.is_enabled(self.id()) { return; }

        for (_idx, _) in source.match_indices("0.") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Avoid leading zero in decimal".into(),
                suggestion: Some("Use '.5' instead of '0.5'".into()),
                fix: None,
            });
        }
    }
}

//
// ============================================================
// CSS010 – Universal selector usage
// ============================================================
//

#[derive(Clone)]
pub struct UniversalSelector;

impl Rule for UniversalSelector {
    fn id(&self) -> &'static str { "CSS010" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig
    ) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.trim_start().starts_with('*') {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Avoid universal selector".into(),
                    suggestion: Some("Use more specific selector".into()),
                    fix: None,
                });
            }
        }
    }
}