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
        .add_rule(Box::new(StrictTypesDeclaration))
        .add_rule(Box::new(LooseComparison))
        .add_rule(Box::new(EvalUsage))
        .add_rule(Box::new(ExecUsage))
        .add_rule(Box::new(GlobalUsage))
        .add_rule(Box::new(UnusedImport))
        .add_rule(Box::new(MissingNamespace))
        .add_rule(Box::new(ArrayLongSyntax))
        .add_rule(Box::new(UnreachableCode))
        .add_rule(Box::new(AssignmentInCondition))
        .add_rule(Box::new(EmptyCatchBlock))
        .add_rule(Box::new(MultipleClassesPerFile))
        .add_rule(Box::new(MagicNumber))
        .add_rule(Box::new(DirectSuperglobalAccess))
}

//
// PHP001 – Trailing Whitespace
//
#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "PHP001" }
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
// PHP042 – Strict Types Declaration
//
#[derive(Clone)]
struct StrictTypesDeclaration;

impl Rule for StrictTypesDeclaration {
    fn id(&self) -> &'static str { "PHP042" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        if !source.contains("declare(strict_types=1);") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Missing strict types declaration".into(),
                suggestion: Some("Add declare(strict_types=1); after <?php".into()),
                fix: None,
            });
        }
    }
}

//
// PHP160 – Loose Comparison
//
#[derive(Clone)]
struct LooseComparison;

impl Rule for LooseComparison {
    fn id(&self) -> &'static str { "PHP160" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("==") && !line.contains("===") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: line.find("==").unwrap_or(0) + 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Loose comparison detected".into(),
                    suggestion: Some("Use === instead of ==".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PHP100 – eval usage
//
#[derive(Clone)]
struct EvalUsage;

impl Rule for EvalUsage {
    fn id(&self) -> &'static str { "PHP100" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("eval(") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: line.find("eval(").unwrap_or(0) + 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Use of eval is forbidden".into(),
                    suggestion: Some("Remove eval usage".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PHP101 – exec usage
//
#[derive(Clone)]
struct ExecUsage;

impl Rule for ExecUsage {
    fn id(&self) -> &'static str { "PHP101" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let forbidden = ["exec(", "shell_exec(", "system("];

        for (i, line) in source.lines().enumerate() {
            for f in &forbidden {
                if line.contains(f) {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: line.find(f).unwrap_or(0) + 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: format!("Forbidden shell execution: {}", f),
                        suggestion: Some("Use safer abstraction".into()),
                        fix: None,
                    });
                }
            }
        }
    }
}

//
// PHP162 – global keyword
//
#[derive(Clone)]
struct GlobalUsage;

impl Rule for GlobalUsage {
    fn id(&self) -> &'static str { "PHP162" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.trim_start().starts_with("global ") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Avoid global keyword".into(),
                    suggestion: Some("Inject dependencies instead".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PHP121 – Unused import (basic detection)
//
#[derive(Clone)]
struct UnusedImport;

impl Rule for UnusedImport {
    fn id(&self) -> &'static str { "PHP121" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.trim_start().starts_with("use ") && !source.matches(line.trim()).skip(1).next().is_some() {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Possibly unused import".into(),
                    suggestion: Some("Remove unused use statement".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PHP200 – Missing namespace
//
#[derive(Clone)]
struct MissingNamespace;

impl Rule for MissingNamespace {
    fn id(&self) -> &'static str { "PHP200" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("class ") && !source.contains("namespace ") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Missing namespace declaration".into(),
                suggestion: Some("Add namespace matching directory #[derive(Clone)]
structure".into()),
                fix: None,
            });
        }
    }
}

//
// PHP181 – Long array syntax
//
#[derive(Clone)]
struct ArrayLongSyntax;

impl Rule for ArrayLongSyntax {
    fn id(&self) -> &'static str { "PHP181" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("array(") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: line.find("array(").unwrap_or(0) + 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Long array syntax used".into(),
                    suggestion: Some("Use [] instead of array()".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PHP260 – Unreachable code
//
#[derive(Clone)]
struct UnreachableCode;

impl Rule for UnreachableCode {
    fn id(&self) -> &'static str { "PHP260" }
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
                    suggestion: Some("Remove unreachable code".into()),
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
// PHP061 – Assignment in condition
//
#[derive(Clone)]
struct AssignmentInCondition;

impl Rule for AssignmentInCondition {
    fn id(&self) -> &'static str { "PHP061" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("if (") && line.contains("=") && !line.contains("==") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Assignment inside condition".into(),
                    suggestion: Some("Separate assignment from condition".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PHP062 – Empty catch block
//
#[derive(Clone)]
struct EmptyCatchBlock;

impl Rule for EmptyCatchBlock {
    fn id(&self) -> &'static str { "PHP062" }
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
// PHP163 – Multiple classes per file
//
#[derive(Clone)]
struct MultipleClassesPerFile;

impl Rule for MultipleClassesPerFile {
    fn id(&self) -> &'static str { "PHP163" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let count = source.matches("class ").count();
        if count > 1 {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Multiple classes in a single file".into(),
                suggestion: Some("Split classes into separate files".into()),
                fix: None,
            });
        }
    }
}

//
// PHP161 – Magic number
//
#[derive(Clone)]
struct MagicNumber;

impl Rule for MagicNumber {
    fn id(&self) -> &'static str { "PHP161" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains(" = 42") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Magic number detected".into(),
                    suggestion: Some("Extract to named constant".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// PHP102 – Direct superglobal access
//
#[derive(Clone)]
struct DirectSuperglobalAccess;

impl Rule for DirectSuperglobalAccess {
    fn id(&self) -> &'static str { "PHP102" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let globals = ["$_GET", "$_POST", "$_REQUEST", "$_COOKIE"];

        for (i, line) in source.lines().enumerate() {
            for g in &globals {
                if line.contains(g) {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: line.find(g).unwrap_or(0) + 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: format!("Direct access to {}", g),
                        suggestion: Some("Use input abstraction layer".into()),
                        fix: None,
                    });
                }
            }
        }
    }
}