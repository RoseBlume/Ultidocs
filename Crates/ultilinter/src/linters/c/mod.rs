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
        .add_rule(Box::new(GetsUsage))
        .add_rule(Box::new(StrcpyUsage))
        .add_rule(Box::new(SprintfUsage))
        // .add_rule(Box::new(MallocWithoutFree))
        // .add_rule(Box::new(PrintfWithoutFormatLiteral))
        // .add_rule(Box::new(MagicNumber))
        .add_rule(Box::new(AssignmentInCondition))
        // .add_rule(Box::new(UninitializedVariable))
        // .add_rule(Box::new(MissingBracesInControl))
        .add_rule(Box::new(GlobalVariableUsage))
        .add_rule(Box::new(MissingReturnInNonVoid))
        // .add_rule(Box::new(NullDereferenceRisk))
        // .add_rule(Box::new(IncludeStdlibMissing))
        .add_rule(Box::new(UnreachableCode))
}

//
// C001 – Trailing Whitespace
//
#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "C001" }
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
// C010 – gets() usage
//
#[derive(Clone)]
struct GetsUsage;

impl Rule for GetsUsage {
    fn id(&self) -> &'static str { "C010" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("gets(") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "gets() is unsafe and removed from C11".into(),
                suggestion: Some("Use fgets() instead".into()),
                fix: None,
            });
        }
    }
}

//
// C020 – strcpy usage
//
#[derive(Clone)]
struct StrcpyUsage;

impl Rule for StrcpyUsage {
    fn id(&self) -> &'static str { "C020" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("strcpy(") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "strcpy() may cause buffer overflow".into(),
                suggestion: Some("Use strncpy() or safer alternatives".into()),
                fix: None,
            });
        }
    }
}

//
// C030 – sprintf usage
//
#[derive(Clone)]
struct SprintfUsage;

impl Rule for SprintfUsage {
    fn id(&self) -> &'static str { "C030" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("sprintf(") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "sprintf() may cause buffer overflow".into(),
                suggestion: Some("Use snprintf() instead".into()),
                fix: None,
            });
        }
    }
}

//
// C040 – Assignment inside condition
//
#[derive(Clone)]
struct AssignmentInCondition;

impl Rule for AssignmentInCondition {
    fn id(&self) -> &'static str { "C040" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("if (") && line.contains("=") && !line.contains("==") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Assignment used in condition".into(),
                    suggestion: Some("Use == for comparison or add parentheses for clarity".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// C050 – Global variable usage
//
#[derive(Clone)]
struct GlobalVariableUsage;

impl Rule for GlobalVariableUsage {
    fn id(&self) -> &'static str { "C050" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if !line.starts_with(" ") && line.contains(";") && !line.contains("(") && !line.contains("#") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Possible global variable declaration".into(),
                    suggestion: Some("Limit global variable usage".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// C060 – Missing return in non-void function
//
#[derive(Clone)]
struct MissingReturnInNonVoid;

impl Rule for MissingReturnInNonVoid {
    fn id(&self) -> &'static str { "C060" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("int ") && source.contains("{") && !source.contains("return") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Non-void function missing return statement".into(),
                suggestion: Some("Ensure all control paths return a value".into()),
                fix: None,
            });
        }
    }
}

//
// C090 – Unreachable code
//
#[derive(Clone)]
struct UnreachableCode;

impl Rule for UnreachableCode {
    fn id(&self) -> &'static str { "C090" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
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

            if line.trim_start().starts_with("return ") {
                found_return = true;
            }
        }
    }
}