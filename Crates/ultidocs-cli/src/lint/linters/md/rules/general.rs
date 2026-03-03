use std::path::Path;
use ultilinter::helpers::line_starts;
use ultilinter::{
    LintConfig, LintError, Fix, TextEdit,
    LintReport, Severity, Rule,
};


pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(TrailingWhitespace),
        Box::new(DoubleBlankLines),
        Box::new(LineLengthLimit),
        Box::new(EOFBlankLine),
    ]
}

//
// ============================================================
// MD001 – Trailing whitespace
// ============================================================
//

#[derive(Clone)]
pub struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "MD001" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        let starts = line_starts(source);

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim_end();

            if trimmed.len() != line.len() {
                let start = starts[i] + trimmed.len();
                let end = starts[i] + line.len();

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
                            range: start..end,
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
// MD002 – Collapse multiple blank lines
// ============================================================
//

#[derive(Clone)]
pub struct DoubleBlankLines;

impl Rule for DoubleBlankLines {
    fn id(&self) -> &'static str { "MD002" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        let starts = line_starts(source);

        let mut blank_count = 0;

        for (i, line) in source.lines().enumerate() {
            let is_blank = line.trim().is_empty();

            if is_blank {
                blank_count += 1;

                if blank_count > 1 {
                    let start = starts[i];
                    let end = start + line.len();

                    // Also remove the newline following the blank line
                    let newline_len = if source.as_bytes().get(end) == Some(&b'\r')
                        && source.as_bytes().get(end + 1) == Some(&b'\n')
                    {
                        2
                    } else if source.as_bytes().get(end) == Some(&b'\n') {
                        1
                    } else {
                        0
                    };

                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Multiple consecutive blank lines".into(),
                        suggestion: Some("Remove extra blank line".into()),
                        fix: Some(Fix {
                            rule_id: self.id(),
                            edits: vec![TextEdit {
                                range: start..(end + newline_len),
                                replacement: String::new(),
                            }],
                        }),
                    });
                }
            } else {
                blank_count = 0;
            }
        }
    }
}










//
// ============================================================
// MD009 – Line length limit (default 80 chars)
// ============================================================
//

#[derive(Clone)]
pub struct LineLengthLimit;

impl Rule for LineLengthLimit {
    fn id(&self) -> &'static str { "MD009" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig
    ) {
        if !config.is_enabled(self.id()) { return; }

        let max_length = 80;
        let _starts = line_starts(source);

        for (i, line) in source.lines().enumerate() {
            if line.len() > max_length {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: max_length + 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: format!("Line exceeds {} characters", max_length),
                    suggestion: Some("Split long line".into()),
                    fix: None,
                });
            }
        }
    }
}







//
// ============================================================
// MD013 – No trailing blank line at EOF
// ============================================================
//

#[derive(Clone)]
pub struct EOFBlankLine;

impl Rule for EOFBlankLine {
    fn id(&self) -> &'static str { "MD013" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        if !source.ends_with('\n') {
            let len = source.len();
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: source.lines().count(),
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "File should end with newline".into(),
                suggestion: Some("Add newline at EOF".into()),
                fix: Some(Fix {
                    rule_id: self.id(),
                    edits: vec![TextEdit { range: len..len, replacement: "\n".into() }],
                }),
            });
        }
    }
}


