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
        .add_rule(Box::new(TabIndentation))
        .add_rule(Box::new(MissingSpaceAfterComma))
        .add_rule(Box::new(SpaceBeforeBrace))
        .add_rule(Box::new(UppercaseBoolean))
        .add_rule(Box::new(UnwrapUsage))
        .add_rule(Box::new(TodoMacro))
        .add_rule(Box::new(DebugMacro))
        .add_rule(Box::new(DoubleSemicolon))
        .add_rule(Box::new(WildcardImport))
        .add_rule(Box::new(EmptyBlock))
        .add_rule(Box::new(TrailingNewline))
}


//
// RUST001 – Trailing whitespace
//

#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "RUST001" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig) {

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
                    suggestion: Some("Remove trailing whitespace".into()),
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
// RUST002 – Tabs instead of spaces
//

#[derive(Clone)]
struct TabIndentation;

impl Rule for TabIndentation {
    fn id(&self) -> &'static str { "RUST002" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig) {

        if !config.is_enabled(self.id()) { return; }

        for (idx, _) in source.match_indices('\t') {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Tab indentation found".into(),
                suggestion: Some("Replace with 4 spaces".into()),
                fix: Some(Fix {
                    rule_id: self.id(),
                    edits: vec![TextEdit {
                        range: idx..idx + 1,
                        replacement: "    ".into(),
                    }],
                }),
            });
        }
    }
}

//
// RUST003 – Missing space after comma
//

#[derive(Clone)]
struct MissingSpaceAfterComma;

impl Rule for MissingSpaceAfterComma {
    fn id(&self) -> &'static str { "RUST003" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig) {

        if !config.is_enabled(self.id()) { return; }

        for (idx, _) in source.match_indices(',') {
            if source.get(idx + 1..idx + 2) != Some(" ")
                && source.get(idx + 1..idx + 2) != Some("\n") {

                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: 0,
                    column: 0,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Missing space after comma".into(),
                    suggestion: Some("Add space after comma".into()),
                    fix: Some(Fix {
                        rule_id: self.id(),
                        edits: vec![TextEdit {
                            range: idx + 1..idx + 1,
                            replacement: " ".into(),
                        }],
                    }),
                });
            }
        }
    }
}

//
// RUST004 – Missing space before {
//

#[derive(Clone)]
struct SpaceBeforeBrace;

impl Rule for SpaceBeforeBrace {
    fn id(&self) -> &'static str { "RUST004" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig) {

        if !config.is_enabled(self.id()) { return; }

        for (idx, _) in source.match_indices('{') {
            if idx > 0 && &source[idx - 1..idx] != " " {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: 0,
                    column: 0,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Missing space before '{'".into(),
                    suggestion: Some("Add space before '{'".into()),
                    fix: Some(Fix {
                        rule_id: self.id(),
                        edits: vec![TextEdit {
                            range: idx..idx,
                            replacement: " ".into(),
                        }],
                    }),
                });
            }
        }
    }
}

//
// RUST005 – Uppercase boolean
//

#[derive(Clone)]
struct UppercaseBoolean;

impl Rule for UppercaseBoolean {
    fn id(&self) -> &'static str { "RUST005" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig) {

        if !config.is_enabled(self.id()) { return; }

        for (idx, _) in source.match_indices("True") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Boolean must be lowercase".into(),
                suggestion: Some("Use 'true'".into()),
                fix: Some(Fix {
                    rule_id: self.id(),
                    edits: vec![TextEdit {
                        range: idx..idx + 4,
                        replacement: "true".into(),
                    }],
                }),
            });
        }

        for (idx, _) in source.match_indices("False") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Boolean must be lowercase".into(),
                suggestion: Some("Use 'false'".into()),
                fix: Some(Fix {
                    rule_id: self.id(),
                    edits: vec![TextEdit {
                        range: idx..idx + 5,
                        replacement: "false".into(),
                    }],
                }),
            });
        }
    }
}

//
// RUST006 – unwrap()
//

#[derive(Clone)]
struct UnwrapUsage;

impl Rule for UnwrapUsage {
    fn id(&self) -> &'static str { "RUST006" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig) {

        if !config.is_enabled(self.id()) { return; }

        for (_idx, _) in source.match_indices(".unwrap()") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Avoid unwrap() in production code".into(),
                suggestion: Some("Handle error explicitly".into()),
                fix: None,
            });
        }
    }
}

//
// RUST007 – todo!()
//

#[derive(Clone)]
struct TodoMacro;

impl Rule for TodoMacro {
    fn id(&self) -> &'static str { "RUST007" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig) {

        if !config.is_enabled(self.id()) { return; }

        for (_idx, _) in source.match_indices("todo!()") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "todo!() found".into(),
                suggestion: Some("Remove before production".into()),
                fix: None,
            });
        }
    }
}

//
// RUST008 – dbg!()
//

#[derive(Clone)]
struct DebugMacro;

impl Rule for DebugMacro {
    fn id(&self) -> &'static str { "RUST008" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig) {

        if !config.is_enabled(self.id()) { return; }

        for (_idx, _) in source.match_indices("dbg!(") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "dbg! macro found".into(),
                suggestion: Some("Remove debug macro".into()),
                fix: None,
            });
        }
    }
}

//
// RUST009 – Double semicolon
//

#[derive(Clone)]
struct DoubleSemicolon;

impl Rule for DoubleSemicolon {
    fn id(&self) -> &'static str { "RUST009" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig) {

        if !config.is_enabled(self.id()) { return; }

        for (idx, _) in source.match_indices(";;") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Double semicolon".into(),
                suggestion: Some("Remove extra semicolon".into()),
                fix: Some(Fix {
                    rule_id: self.id(),
                    edits: vec![TextEdit {
                        range: idx..idx + 1,
                        replacement: String::new(),
                    }],
                }),
            });
        }
    }
}

//
// RUST010 – Wildcard import
//

#[derive(Clone)]
struct WildcardImport;

impl Rule for WildcardImport {
    fn id(&self) -> &'static str { "RUST010" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig) {

        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.trim().starts_with("use ") && line.contains("::*") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Wildcard import".into(),
                    suggestion: Some("Import specific items instead".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// RUST011 – Empty block
//

#[derive(Clone)]
struct EmptyBlock;

impl Rule for EmptyBlock {
    fn id(&self) -> &'static str { "RUST011" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig) {

        if !config.is_enabled(self.id()) { return; }

        for (_idx, _) in source.match_indices("{ }") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Empty block".into(),
                suggestion: Some("Remove empty block or add implementation".into()),
                fix: None,
            });
        }
    }
}

//
// RUST012 – Missing trailing newline
//

#[derive(Clone)]
struct TrailingNewline;

impl Rule for TrailingNewline {
    fn id(&self) -> &'static str { "RUST012" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig) {

        if !config.is_enabled(self.id()) { return; }

        if !source.ends_with('\n') {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Missing trailing newline".into(),
                suggestion: Some("Add newline at end of file".into()),
                fix: Some(Fix {
                    rule_id: self.id(),
                    edits: vec![TextEdit {
                        range: source.len()..source.len(),
                        replacement: "\n".into(),
                    }],
                }),
            });
        }
    }
}