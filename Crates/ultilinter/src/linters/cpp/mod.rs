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
        .add_rule(Box::new(UsingNamespaceStd))
        .add_rule(Box::new(RawNewUsage))
        .add_rule(Box::new(RawDeleteUsage))
        .add_rule(Box::new(MallocUsage))
        .add_rule(Box::new(CStyleCastUsage))
        .add_rule(Box::new(PrintfUsage))
        .add_rule(Box::new(NullMacroUsage))
        // .add_rule(Box::new(MissingVirtualDestructor))
        // .add_rule(Box::new(PassByValueLargeObject))
        // .add_rule(Box::new(ConstCorrectness))
        // .add_rule(Box::new(IncludeUsingQuotesForSystem))
        // .add_rule(Box::new(MagicNumber))
        // .add_rule(Box::new(UninitializedMember))
        // .add_rule(Box::new(EmptyCatchBlock))
        .add_rule(Box::new(UnreachableCode))
}

//
// CPP001 – Trailing Whitespace
//
#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "CPP001" }
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
// CPP010 – using namespace std;
//
#[derive(Clone)]
struct UsingNamespaceStd;

impl Rule for UsingNamespaceStd {
    fn id(&self) -> &'static str { "CPP010" }
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
            if line.contains("using namespace std") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Avoid 'using namespace std;'".into(),
                    suggestion: Some("Use explicit std:: prefixes".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// CPP020 – Raw new usage
//
#[derive(Clone)]
struct RawNewUsage;

impl Rule for RawNewUsage {
    fn id(&self) -> &'static str { "CPP020" }
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
            if line.contains("new ") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Raw new detected".into(),
                    suggestion: Some("Use std::make_unique or std::make_shared".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// CPP021 – Raw delete usage
//
#[derive(Clone)]
struct RawDeleteUsage;

impl Rule for RawDeleteUsage {
    fn id(&self) -> &'static str { "CPP021" }
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
            if line.contains("delete ") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Raw delete detected".into(),
                    suggestion: Some("Use RAII containers instead of manual delete".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// CPP030 – malloc usage
//
#[derive(Clone)]
struct MallocUsage;

impl Rule for MallocUsage {
    fn id(&self) -> &'static str { "CPP030" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("malloc(") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "malloc() used in C++ code".into(),
                suggestion: Some("Use new or STL containers instead".into()),
                fix: None,
            });
        }
    }
}

//
// CPP040 – C-style cast
//
#[derive(Clone)]
struct CStyleCastUsage;

impl Rule for CStyleCastUsage {
    fn id(&self) -> &'static str { "CPP040" }
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
            if line.contains("(") && line.contains(")") && line.contains(")") && line.contains(")") {
                if line.contains(")") && line.contains(");") && line.contains("(") {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Potential C-style cast detected".into(),
                        suggestion: Some("Use static_cast, dynamic_cast, etc.".into()),
                        fix: None,
                    });
                }
            }
        }
    }
}

//
// CPP050 – printf usage
//
#[derive(Clone)]
struct PrintfUsage;

impl Rule for PrintfUsage {
    fn id(&self) -> &'static str { "CPP050" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("printf(") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "printf used in C++ code".into(),
                suggestion: Some("Use std::cout or fmt library".into()),
                fix: None,
            });
        }
    }
}

//
// CPP060 – NULL macro usage
//
#[derive(Clone)]
struct NullMacroUsage;

impl Rule for NullMacroUsage {
    fn id(&self) -> &'static str { "CPP060" }
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
            if line.contains("NULL") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Use nullptr instead of NULL".into(),
                    suggestion: Some("Replace NULL with nullptr".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// CPP090 – Unreachable code
//
#[derive(Clone)]
struct UnreachableCode;

impl Rule for UnreachableCode {
    fn id(&self) -> &'static str { "CPP090" }
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