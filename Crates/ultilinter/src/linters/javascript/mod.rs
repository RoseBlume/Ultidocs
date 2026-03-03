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
        .add_rule(Box::new(MissingSemicolon))
        .add_rule(Box::new(DoubleSemicolon))
        .add_rule(Box::new(MissingSpaceAfterComma))
        .add_rule(Box::new(VarInsteadOfLetConst))
        .add_rule(Box::new(DebuggerStatement))
        .add_rule(Box::new(UppercaseBooleanJS))
        .add_rule(Box::new(ExtraSpacesBeforeBrace))
}



//
// JS001 – Trailing whitespace
//

#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "JS001" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, _config: &LintConfig) {
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
                    fix: Some(Fix { rule_id: self.id(), edits: vec![TextEdit { range: starts[i]+trimmed.len()..starts[i]+line.len(), replacement: String::new() }] })
                });
            }
        }
    }
}

//
// JS002 – Missing semicolon
//

#[derive(Clone)]
struct MissingSemicolon;

impl Rule for MissingSemicolon {
    fn id(&self) -> &'static str { "JS002" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, _config: &LintConfig) {
        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.ends_with("}") || trimmed.ends_with("{") { continue; }
            if !trimmed.ends_with(";") && !trimmed.is_empty() {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: line.len(),
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Missing semicolon".into(),
                    suggestion: Some("Add semicolon at end".into()),
                    fix: Some(Fix { rule_id: self.id(), edits: vec![TextEdit { range: source.lines().take(i+1).map(|l| l.len()+1).sum::<usize>() - 1..source.lines().take(i+1).map(|l| l.len()+1).sum::<usize>() -1, replacement: ";".into() }] }),
                });
            }
        }
    }
}

//
// JS003 – Double semicolon
//

#[derive(Clone)]
struct DoubleSemicolon;

impl Rule for DoubleSemicolon {
    fn id(&self) -> &'static str { "JS003" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, _config: &LintConfig) {
        for (idx, _) in source.match_indices(";;") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Double semicolon".into(),
                suggestion: Some("Remove extra semicolon".into()),
                fix: Some(Fix { rule_id: self.id(), edits: vec![TextEdit { range: idx..idx+1, replacement: String::new() }] }),
            });
        }
    }
}

//
// JS004 – Missing space after comma
//

#[derive(Clone)]
struct MissingSpaceAfterComma;

impl Rule for MissingSpaceAfterComma {
    fn id(&self) -> &'static str { "JS004" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, _config: &LintConfig) {
        for (idx, _) in source.match_indices(',') {
            if source.get(idx+1..idx+2) != Some(" ") && source.get(idx+1..idx+2) != Some("\n") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: 0,
                    column: 0,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Missing space after comma".into(),
                    suggestion: Some("Add space after comma".into()),
                    fix: Some(Fix { rule_id: self.id(), edits: vec![TextEdit { range: idx+1..idx+1, replacement: " ".into() }] }),
                });
            }
        }
    }
}

//
// JS005 – var instead of let/const
//

#[derive(Clone)]
struct VarInsteadOfLetConst;

impl Rule for VarInsteadOfLetConst {
    fn id(&self) -> &'static str { "JS005" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, _config: &LintConfig) {
        for (i, line) in source.lines().enumerate() {
            if line.trim_start().starts_with("var ") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Avoid 'var', use 'let' or 'const'".into(),
                    suggestion: Some("Replace 'var' with 'let' or 'const'".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// JS006 – debugger statement
//

#[derive(Clone)]
struct DebuggerStatement;

impl Rule for DebuggerStatement {
    fn id(&self) -> &'static str { "JS006" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, _config: &LintConfig) {
        for (i, line) in source.lines().enumerate() {
            if line.contains("debugger;") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Debugger statement found".into(),
                    suggestion: Some("Remove debugger statement".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// JS007 – Uppercase boolean
//

#[derive(Clone)]
struct UppercaseBooleanJS;

impl Rule for UppercaseBooleanJS {
    fn id(&self) -> &'static str { "JS007" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, _config: &LintConfig) {
        for (idx, _) in source.match_indices("True") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Boolean should be lowercase".into(),
                suggestion: Some("Use 'true'".into()),
                fix: Some(Fix { rule_id: self.id(), edits: vec![TextEdit { range: idx..idx+4, replacement: "true".into() }] }),
            });
        }
        for (idx, _) in source.match_indices("False") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 0,
                column: 0,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Boolean should be lowercase".into(),
                suggestion: Some("Use 'false'".into()),
                fix: Some(Fix { rule_id: self.id(), edits: vec![TextEdit { range: idx..idx+5, replacement: "false".into() }] }),
            });
        }
    }
}

//
// JS008 – Extra spaces before brace
//

#[derive(Clone)]
struct ExtraSpacesBeforeBrace;

impl Rule for ExtraSpacesBeforeBrace {
    fn id(&self) -> &'static str { "JS008" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, _config: &LintConfig) {
        for (idx, _) in source.match_indices('{') {
            if idx > 0 && &source[idx-1..idx] == " " {
                continue;
            }
            if idx > 0 {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: 0,
                    column: 0,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Missing space before '{'".into(),
                    suggestion: Some("Add space before '{'".into()),
                    fix: Some(Fix { rule_id: self.id(), edits: vec![TextEdit { range: idx..idx, replacement: " ".into() }] }),
                });
            }
        }
    }
}