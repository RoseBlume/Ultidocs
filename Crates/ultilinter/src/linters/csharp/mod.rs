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
        .add_rule(Box::new(ConsoleWriteLineUsage))
        // .add_rule(Box::new(VarOveruse))
        .add_rule(Box::new(AsyncVoidUsage))
        .add_rule(Box::new(TaskResultUsage))
        .add_rule(Box::new(EmptyCatchBlock))
        .add_rule(Box::new(NullForgivingOperator))
        // .add_rule(Box::new(EqualsNullComparison))
        .add_rule(Box::new(PublicFieldUsage))
        // .add_rule(Box::new(MagicNumber))
        // .add_rule(Box::new(RegionUsage))
        .add_rule(Box::new(UnreachableCode))
        // .add_rule(Box::new(DisposableNotDisposed))
        // .add_rule(Box::new(LockThisUsage))
        // .add_rule(Box::new(StringConcatenationInLoop))
}

//
// CS001 – Trailing Whitespace
//
#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "CS001" }
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
// CS010 – Console.WriteLine usage
//
#[derive(Clone)]
struct ConsoleWriteLineUsage;

impl Rule for ConsoleWriteLineUsage {
    fn id(&self) -> &'static str { "CS010" }
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
            if line.contains("Console.WriteLine") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Console.WriteLine detected".into(),
                    suggestion: Some("Remove debug logging in production code".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// CS020 – async void usage
//
#[derive(Clone)]
struct AsyncVoidUsage;

impl Rule for AsyncVoidUsage {
    fn id(&self) -> &'static str { "CS020" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("async void") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Avoid async void methods".into(),
                    suggestion: Some("Use async Task instead".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// CS030 – Task.Result usage
//
#[derive(Clone)]
struct TaskResultUsage;

impl Rule for TaskResultUsage {
    fn id(&self) -> &'static str { "CS030" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains(".Result") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Blocking on Task.Result detected".into(),
                suggestion: Some("Use await instead of .Result".into()),
                fix: None,
            });
        }
    }
}

//
// CS040 – Empty catch block
//
#[derive(Clone)]
struct EmptyCatchBlock;

impl Rule for EmptyCatchBlock {
    fn id(&self) -> &'static str { "CS040" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("catch") && source.contains("{}") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Empty catch block detected".into(),
                suggestion: Some("Handle or log the exception".into()),
                fix: None,
            });
        }
    }
}

//
// CS050 – Null-forgiving operator
//
#[derive(Clone)]
struct NullForgivingOperator;

impl Rule for NullForgivingOperator {
    fn id(&self) -> &'static str { "CS050" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("!.") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Null-forgiving operator used".into(),
                suggestion: Some("Handle null explicitly instead of using !".into()),
                fix: None,
            });
        }
    }
}

//
// CS060 – Public field usage
//
#[derive(Clone)]
struct PublicFieldUsage;

impl Rule for PublicFieldUsage {
    fn id(&self) -> &'static str { "CS060" }
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
            if line.trim_start().starts_with("public ") && line.contains(";") && !line.contains("(") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Public field detected".into(),
                    suggestion: Some("Use properties instead of public fields".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// CS090 – Unreachable code
//
#[derive(Clone)]
struct UnreachableCode;

impl Rule for UnreachableCode {
    fn id(&self) -> &'static str { "CS090" }
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