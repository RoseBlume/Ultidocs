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
        .add_rule(Box::new(VarUsage))
        .add_rule(Box::new(AnyTypeUsage))
        .add_rule(Box::new(ConsoleLogUsage))
        .add_rule(Box::new(DoubleEqualsUsage))
        .add_rule(Box::new(NonNullAssertionUsage))
        // .add_rule(Box::new(ImplicitAnyParameter))
        .add_rule(Box::new(MissingReturnType))
        .add_rule(Box::new(UnhandledPromise))
        // .add_rule(Box::new(EmptyInterface))
        .add_rule(Box::new(EmptyCatchBlock))
        // .add_rule(Box::new(MagicNumber))
        // .add_rule(Box::new(NamespaceUsage))
        // .add_rule(Box::new(DefaultExportUsage))
        .add_rule(Box::new(UnreachableCode))
}

//
// TS001 – Trailing Whitespace
//
#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "TS001" }
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
// TS010 – var usage
//
#[derive(Clone)]
struct VarUsage;

impl Rule for VarUsage {
    fn id(&self) -> &'static str { "TS010" }
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
            if line.contains("var ") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Use let or const instead of var".into(),
                    suggestion: Some("Replace var with let or const".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// TS020 – any type usage
//
#[derive(Clone)]
struct AnyTypeUsage;

impl Rule for AnyTypeUsage {
    fn id(&self) -> &'static str { "TS020" }
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
            if line.contains(": any") || line.contains("<any>") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Usage of 'any' type detected".into(),
                    suggestion: Some("Use a specific type instead of any".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// TS030 – console.log usage
//
#[derive(Clone)]
struct ConsoleLogUsage;

impl Rule for ConsoleLogUsage {
    fn id(&self) -> &'static str { "TS030" }
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
            if line.contains("console.log") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "console.log detected".into(),
                    suggestion: Some("Remove debug logging for production".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// TS040 – == usage
//
#[derive(Clone)]
struct DoubleEqualsUsage;

impl Rule for DoubleEqualsUsage {
    fn id(&self) -> &'static str { "TS040" }
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
            if line.contains("==") && !line.contains("===") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Use strict equality (===) instead of ==".into(),
                    suggestion: Some("Replace == with ===".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// TS050 – Non-null assertion (!)
//
#[derive(Clone)]
struct NonNullAssertionUsage;

impl Rule for NonNullAssertionUsage {
    fn id(&self) -> &'static str { "TS050" }
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
                    message: "Non-null assertion operator used".into(),
                    suggestion: Some("Handle null/undefined explicitly".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// TS060 – Missing return type
//
#[derive(Clone)]
struct MissingReturnType;

impl Rule for MissingReturnType {
    fn id(&self) -> &'static str { "TS060" }
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
            if line.contains("function ") && !line.contains("):") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Missing explicit return type".into(),
                    suggestion: Some("Add return type annotation".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// TS070 – Unhandled Promise
//
#[derive(Clone)]
struct UnhandledPromise;

impl Rule for UnhandledPromise {
    fn id(&self) -> &'static str { "TS070" }
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
            if line.contains("new Promise") && !line.contains("await") && !line.contains(".then") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Promise created without handling".into(),
                    suggestion: Some("Await or handle promise result".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// TS080 – Empty catch block
//
#[derive(Clone)]
struct EmptyCatchBlock;

impl Rule for EmptyCatchBlock {
    fn id(&self) -> &'static str { "TS080" }
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
                suggestion: Some("Handle or log the error".into()),
                fix: None,
            });
        }
    }
}

//
// TS090 – Unreachable code
//
#[derive(Clone)]
struct UnreachableCode;

impl Rule for UnreachableCode {
    fn id(&self) -> &'static str { "TS090" }
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