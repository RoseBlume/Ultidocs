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
        .add_rule(Box::new(EqualsAssignmentUsage))
        .add_rule(Box::new(AttachUsage))
        .add_rule(Box::new(SetwdUsage))
        .add_rule(Box::new(LibraryInsideFunction))
        // .add_rule(Box::new(UnusedVariable))
        // .add_rule(Box::new(MissingReturnInFunction))
        // .add_rule(Box::new(ImplicitGlobalAssignment))
        .add_rule(Box::new(HardcodedFilePath))
        .add_rule(Box::new(LoopInsteadOfVectorization))
        .add_rule(Box::new(PartialArgumentMatching))
        // .add_rule(Box::new(SinkWithoutReset))
        .add_rule(Box::new(UnreachableCode))
        // .add_rule(Box::new(TPrintUsage))
        // .add_rule(Box::new(MagicNumber))
}

//
// R001 – Trailing Whitespace
//
#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "R001" }
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
// R010 – Use <- instead of = for assignment
//
#[derive(Clone)]
struct EqualsAssignmentUsage;

impl Rule for EqualsAssignmentUsage {
    fn id(&self) -> &'static str { "R010" }
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
            if line.contains(" = ") && !line.contains("==") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Use <- for assignment instead of =".into(),
                    suggestion: Some("Replace = with <- for assignment".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// R020 – attach() usage
//
#[derive(Clone)]
struct AttachUsage;

impl Rule for AttachUsage {
    fn id(&self) -> &'static str { "R020" }
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
            if line.contains("attach(") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "attach() usage detected".into(),
                    suggestion: Some("Avoid attach(); use explicit references".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// R021 – setwd() usage
//
#[derive(Clone)]
struct SetwdUsage;

impl Rule for SetwdUsage {
    fn id(&self) -> &'static str { "R021" }
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
            if line.contains("setwd(") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "setwd() usage detected".into(),
                    suggestion: Some("Avoid changing working directory in scripts".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// R030 – Library inside function
//
#[derive(Clone)]
struct LibraryInsideFunction;

impl Rule for LibraryInsideFunction {
    fn id(&self) -> &'static str { "R030" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        let mut in_function = false;

        for (i, line) in source.lines().enumerate() {
            if line.contains("<- function") {
                in_function = true;
            }
            if in_function && line.contains("library(") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "library() inside function".into(),
                    suggestion: Some("Load libraries at top-level scope".into()),
                    fix: None,
                });
            }
            if line.contains("}") {
                in_function = false;
            }
        }
    }
}

//
// R040 – Hardcoded file path
//
#[derive(Clone)]
struct HardcodedFilePath;

impl Rule for HardcodedFilePath {
    fn id(&self) -> &'static str { "R040" }
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
            if line.contains("read.csv(\"/") || line.contains("read.table(\"/") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Hardcoded absolute file path".into(),
                    suggestion: Some("Use relative paths or configuration".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// R050 – Loop instead of vectorization
//
#[derive(Clone)]
struct LoopInsteadOfVectorization;

impl Rule for LoopInsteadOfVectorization {
    fn id(&self) -> &'static str { "R050" }
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
            if line.trim_start().starts_with("for (") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "For-loop detected".into(),
                    suggestion: Some("Consider vectorized alternatives".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// R060 – Partial argument matching
//
#[derive(Clone)]
struct PartialArgumentMatching;

impl Rule for PartialArgumentMatching {
    fn id(&self) -> &'static str { "R060" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("na.rm=T") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Partial argument matching used".into(),
                suggestion: Some("Use full argument name and TRUE".into()),
                fix: None,
            });
        }
    }
}

//
// R070 – Unreachable code
//
#[derive(Clone)]
struct UnreachableCode;

impl Rule for UnreachableCode {
    fn id(&self) -> &'static str { "R070" }
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

            if line.trim_start().starts_with("return(") {
                found_return = true;
            }
        }
    }
}