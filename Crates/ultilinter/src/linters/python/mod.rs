use std::path::Path;
use crate::{
    LintConfig,
    LintError,
    Fix,
    TextEdit,
    LintReport,
    Severity,
    Rule,
    Linter,
};

pub fn linter() -> Linter {
    Linter::new()
        .add_rule(Box::new(TrailingWhitespace))
        .add_rule(Box::new(MissingShebang))
        .add_rule(Box::new(ExecUsage))
        .add_rule(Box::new(EvalUsage))
        .add_rule(Box::new(WildcardImport))
        .add_rule(Box::new(BareExcept))
        // .add_rule(Box::new(PrintUsage))
        // .add_rule(Box::new(GlobalUsage))
        .add_rule(Box::new(MutableDefaultArgument))
        // .add_rule(Box::new(UnusedImport))
        // .add_rule(Box::new(MissingTypeHints))
        // .add_rule(Box::new(RedundantPass))
        .add_rule(Box::new(ComparisonToNoneUsingDoubleEquals))
        .add_rule(Box::new(AssertUsedInProduction))
        .add_rule(Box::new(UnreachableCode))
}

//
// PY001 – Trailing Whitespace
//
#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "PY001" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut offset = 0;

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
                        edits: vec![TextEdit {
                            range: offset + trimmed.len()..offset + line.len(),
                            replacement: String::new(),
                        }],
                    }),
                });
            }
            offset += line.len() + 1;
        }
    }
}

//
// PY010 – Missing Shebang
//
#[derive(Clone)]
struct MissingShebang;

impl Rule for MissingShebang {
    fn id(&self) -> &'static str { "PY010" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        if file.is_some() && !source.starts_with("#!") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Missing shebang line".into(),
                suggestion: Some("Add #!/usr/bin/env python3".into()),
                fix: None,
            });
        }
    }
}

//
// PY020 – exec usage
//
#[derive(Clone)]
struct ExecUsage;

impl Rule for ExecUsage {
    fn id(&self) -> &'static str { "PY020" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("exec(") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Use of exec() detected".into(),
                    suggestion: Some("Avoid dynamic code execution".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PY021 – eval usage
//
#[derive(Clone)]
struct EvalUsage;

impl Rule for EvalUsage {
    fn id(&self) -> &'static str { "PY021" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("eval(") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Use of eval() detected".into(),
                    suggestion: Some("Avoid evaluating dynamic expressions".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PY030 – Wildcard import
//
#[derive(Clone)]
struct WildcardImport;

impl Rule for WildcardImport {
    fn id(&self) -> &'static str { "PY030" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.trim_start().starts_with("from ") && line.contains(" import *") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Wildcard import used".into(),
                    suggestion: Some("Import specific names instead of *".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PY031 – Bare except
//
#[derive(Clone)]
struct BareExcept;

impl Rule for BareExcept {
    fn id(&self) -> &'static str { "PY031" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.trim() == "except:" {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Bare except detected".into(),
                    suggestion: Some("Catch specific exception types".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PY040 – Mutable default argument
//
#[derive(Clone)]
struct MutableDefaultArgument;

impl Rule for MutableDefaultArgument {
    fn id(&self) -> &'static str { "PY040" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("def ") && (line.contains("=[]") || line.contains("={}")) {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Mutable default argument".into(),
                    suggestion: Some("Use None and initialize inside function".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PY050 – Comparison to None using ==
//
#[derive(Clone)]
struct ComparisonToNoneUsingDoubleEquals;

impl Rule for ComparisonToNoneUsingDoubleEquals {
    fn id(&self) -> &'static str { "PY050" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("== None") || line.contains("!= None") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Comparison to None using ==".into(),
                    suggestion: Some("Use 'is None' or 'is not None'".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PY060 – Assert used outside tests
//
#[derive(Clone)]
struct AssertUsedInProduction;

impl Rule for AssertUsedInProduction {
    fn id(&self) -> &'static str { "PY060" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.trim_start().starts_with("assert ") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Assert statement detected".into(),
                    suggestion: Some("Avoid assert in production code".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PY070 – Unreachable code
//
#[derive(Clone)]
struct UnreachableCode;

impl Rule for UnreachableCode {
    fn id(&self) -> &'static str { "PY070" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut found_return = false;

        for (i, line) in source.lines().enumerate() {
            if found_return && !line.trim().is_empty() {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Unreachable code detected".into(),
                    suggestion: Some("Remove unreachable statements".into()),
                    fix: None,
                });
                break;
            }

            if line.trim_start().starts_with("return") || line.trim_start().starts_with("raise") {
                found_return = true;
            }
        }
    }
}