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
        .add_rule(Box::new(MissingPackageDeclaration))
        .add_rule(Box::new(MultiplePublicClasses))
        .add_rule(Box::new(SystemOutUsage))
        .add_rule(Box::new(EmptyCatchBlock))
        // .add_rule(Box::new(PrintStackTraceUsage))
        .add_rule(Box::new(EqualsWithoutHashCode))
        // .add_rule(Box::new(HashCodeWithoutEquals))
        // .add_rule(Box::new(NullComparisonUsingEquals))
        .add_rule(Box::new(RawTypeUsage))
        // .add_rule(Box::new(ThreadSleepUsage))
        // .add_rule(Box::new(SynchronizedOnThis))
        // .add_rule(Box::new(MagicNumber))
        // .add_rule(Box::new(UnusedImport))
        // .add_rule(Box::new(FinallyWithoutTry))
        .add_rule(Box::new(ThrowingGenericException))
        .add_rule(Box::new(UnreachableCode))
        .add_rule(Box::new(MutablePublicField))
        .add_rule(Box::new(StringComparisonUsingDoubleEquals))
}

//
// JAVA001 – Trailing Whitespace
//
#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "JAVA001" }
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
// JAVA010 – Missing package declaration
//
#[derive(Clone)]
struct MissingPackageDeclaration;

impl Rule for MissingPackageDeclaration {
    fn id(&self) -> &'static str { "JAVA010" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("class ") && !source.trim_start().starts_with("package ") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Missing package declaration".into(),
                suggestion: Some("Add package declaration at top of file".into()),
                fix: None,
            });
        }
    }
}

//
// JAVA011 – Multiple public classes
//
#[derive(Clone)]
struct MultiplePublicClasses;

impl Rule for MultiplePublicClasses {
    fn id(&self) -> &'static str { "JAVA011" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let count = source.matches("public class ").count();
        if count > 1 {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Multiple public classes in a single file".into(),
                suggestion: Some("Only one public class per file".into()),
                fix: None,
            });
        }
    }
}

//
// JAVA020 – System.out usage
//
#[derive(Clone)]
struct SystemOutUsage;

impl Rule for SystemOutUsage {
    fn id(&self) -> &'static str { "JAVA020" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("System.out.println") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "System.out.println used".into(),
                    suggestion: Some("Use logging framework instead".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// JAVA021 – Empty catch block
//
#[derive(Clone)]
struct EmptyCatchBlock;

impl Rule for EmptyCatchBlock {
    fn id(&self) -> &'static str { "JAVA021" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("catch") && source.contains("{}") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Empty catch block".into(),
                suggestion: Some("Handle or log exception".into()),
                fix: None,
            });
        }
    }
}

//
// JAVA030 – equals without hashCode
//
#[derive(Clone)]
struct EqualsWithoutHashCode;

impl Rule for EqualsWithoutHashCode {
    fn id(&self) -> &'static str { "JAVA030" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("boolean equals(") && !source.contains("int hashCode(") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "equals() defined without hashCode()".into(),
                suggestion: Some("Override hashCode() when overriding equals()".into()),
                fix: None,
            });
        }
    }
}

//
// JAVA031 – Raw type usage
//
#[derive(Clone)]
struct RawTypeUsage;

impl Rule for RawTypeUsage {
    fn id(&self) -> &'static str { "JAVA031" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("List ") && !source.contains("List<") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Raw generic type used".into(),
                suggestion: Some("Specify generic type parameters".into()),
                fix: None,
            });
        }
    }
}

//
// JAVA040 – Throwing generic Exception
//
#[derive(Clone)]
struct ThrowingGenericException;

impl Rule for ThrowingGenericException {
    fn id(&self) -> &'static str { "JAVA040" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("throw new Exception(") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Throwing generic Exception".into(),
                suggestion: Some("Use specific exception type".into()),
                fix: None,
            });
        }
    }
}

//
// JAVA050 – Unreachable code
//
#[derive(Clone)]
struct UnreachableCode;

impl Rule for UnreachableCode {
    fn id(&self) -> &'static str { "JAVA050" }
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

            if line.trim_start().starts_with("return") || line.trim_start().starts_with("throw") {
                found_return = true;
            }
        }
    }
}

//
// JAVA060 – Mutable public field
//
#[derive(Clone)]
struct MutablePublicField;

impl Rule for MutablePublicField {
    fn id(&self) -> &'static str { "JAVA060" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.trim_start().starts_with("public ")
                && line.contains(";")
                && !line.contains("final")
                && !line.contains("class")
                && !line.contains("(")
            {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Mutable public field".into(),
                    suggestion: Some("Make field private and provide accessors".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// JAVA070 – String comparison using ==
//
#[derive(Clone)]
struct StringComparisonUsingDoubleEquals;

impl Rule for StringComparisonUsingDoubleEquals {
    fn id(&self) -> &'static str { "JAVA070" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("==") && line.contains("\"") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "String comparison using ==".into(),
                    suggestion: Some("Use .equals() for String comparison".into()),
                    fix: None,
                });
            }
        }
    }
}