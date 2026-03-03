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
        .add_rule(Box::new(PrintUsage))
        // .add_rule(Box::new(VarUsage))
        .add_rule(Box::new(DynamicUsage))
        .add_rule(Box::new(AsyncVoidUsage))
        // .add_rule(Box::new(EmptyCatchBlock))
        .add_rule(Box::new(NullAssertionUsage))
        // .add_rule(Box::new(LateWithoutInitializer))
        .add_rule(Box::new(MissingReturnType))
        // .add_rule(Box::new(DoubleEqualsNull))
        .add_rule(Box::new(SetStateWithoutMountedCheck))
        // .add_rule(Box::new(BuildContextAcrossAsyncGap))
        // .add_rule(Box::new(MagicNumber))
        // .add_rule(Box::new(ConstConstructorSuggestion))
        .add_rule(Box::new(UnreachableCode))
}

//
// DART001 – Trailing Whitespace
//
#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "DART001" }
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
// DART010 – print() usage
//
#[derive(Clone)]
struct PrintUsage;

impl Rule for PrintUsage {
    fn id(&self) -> &'static str { "DART010" }
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
            if line.contains("print(") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "print() detected".into(),
                    suggestion: Some("Remove debug prints in production".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// DART020 – dynamic usage
//
#[derive(Clone)]
struct DynamicUsage;

impl Rule for DynamicUsage {
    fn id(&self) -> &'static str { "DART020" }
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
            if line.contains(" dynamic ") || line.contains(": dynamic") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Usage of dynamic detected".into(),
                    suggestion: Some("Use specific types instead of dynamic".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// DART030 – async void usage
//
#[derive(Clone)]
struct AsyncVoidUsage;

impl Rule for AsyncVoidUsage {
    fn id(&self) -> &'static str { "DART030" }
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
            if line.contains("async") && line.contains("void ") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Avoid async void functions".into(),
                    suggestion: Some("Use Future<void> instead".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// DART040 – Null assertion operator (!)
//
#[derive(Clone)]
struct NullAssertionUsage;

impl Rule for NullAssertionUsage {
    fn id(&self) -> &'static str { "DART040" }
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
            if line.contains("!.") || line.trim_end().ends_with('!') {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Null assertion operator used".into(),
                    suggestion: Some("Handle null safely instead of using !".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// DART050 – setState without mounted check (Flutter)
//
#[derive(Clone)]
struct SetStateWithoutMountedCheck;

impl Rule for SetStateWithoutMountedCheck {
    fn id(&self) -> &'static str { "DART050" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("setState(") && !source.contains("mounted") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "setState called without checking mounted".into(),
                suggestion: Some("Check if (mounted) before calling setState".into()),
                fix: None,
            });
        }
    }
}

//
// DART060 – Missing return type
//
#[derive(Clone)]
struct MissingReturnType;

impl Rule for MissingReturnType {
    fn id(&self) -> &'static str { "DART060" }
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
            if line.contains("(") && line.contains(")") && line.contains("{") && !line.contains("=>") {
                if !line.contains("Future") && !line.contains("void") && !line.contains("int") {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Missing explicit return type".into(),
                        suggestion: Some("Declare explicit return type".into()),
                        fix: None,
                    });
                }
            }
        }
    }
}

//
// DART090 – Unreachable code
//
#[derive(Clone)]
struct UnreachableCode;

impl Rule for UnreachableCode {
    fn id(&self) -> &'static str { "DART090" }
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
                    suggestion: Some("Remove unreachable code".into()),
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