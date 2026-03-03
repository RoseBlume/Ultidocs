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
        .add_rule(Box::new(MissingGlobalStart))
        .add_rule(Box::new(MultipleTextSections))
        .add_rule(Box::new(UnsafeIntUsage))
        .add_rule(Box::new(DeprecatedInstruction))
        .add_rule(Box::new(StackImbalance))
        .add_rule(Box::new(PushWithoutPop))
        .add_rule(Box::new(UnusedLabel))
        .add_rule(Box::new(JumpToNextInstruction))
        // .add_rule(Box::new(MissingSectionDirective))
        .add_rule(Box::new(HardcodedAddress))
        .add_rule(Box::new(MissingBitsDirective))
        // .add_rule(Box::new(InvalidRegisterMix))
        .add_rule(Box::new(RedundantMov))
        .add_rule(Box::new(ZeroWithMovInsteadOfXor))
}

//
// ASM001 – Trailing Whitespace
//
#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "ASM001" }
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
// ASM010 – Missing global _start
//
#[derive(Clone)]
struct MissingGlobalStart;

impl Rule for MissingGlobalStart {
    fn id(&self) -> &'static str { "ASM010" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        if source.contains("_start:") && !source.contains("global _start") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Missing 'global _start' declaration".into(),
                suggestion: Some("Add 'global _start' before label".into()),
                fix: None,
            });
        }
    }
}

//
// ASM011 – Multiple .text sections
//
#[derive(Clone)]
struct MultipleTextSections;

impl Rule for MultipleTextSections {
    fn id(&self) -> &'static str { "ASM011" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let count = source.matches("section .text").count();
        if count > 1 {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Multiple .text sections detected".into(),
                suggestion: Some("Consolidate into a single section".into()),
                fix: None,
            });
        }
    }
}

//
// ASM020 – Unsafe int usage
//
#[derive(Clone)]
struct UnsafeIntUsage;

impl Rule for UnsafeIntUsage {
    fn id(&self) -> &'static str { "ASM020" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("int ") && !line.contains("int 0x80") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Unknown software interrupt used".into(),
                    suggestion: Some("Verify interrupt usage".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// ASM021 – Deprecated instruction
//
#[derive(Clone)]
struct DeprecatedInstruction;

impl Rule for DeprecatedInstruction {
    fn id(&self) -> &'static str { "ASM021" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let deprecated = ["aaa", "aad", "aam", "aas"];

        for (i, line) in source.lines().enumerate() {
            for inst in &deprecated {
                if line.trim_start().starts_with(inst) {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: format!("Deprecated in
struction '{}'", inst),
                        suggestion: Some("Avoid legacy BCD in
structions".into()),
                        fix: None,
                    });
                }
            }
        }
    }
}

//
// ASM030 – Stack imbalance
//
#[derive(Clone)]
struct StackImbalance;

impl Rule for StackImbalance {
    fn id(&self) -> &'static str { "ASM030" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let pushes = source.matches("push ").count();
        let pops = source.matches("pop ").count();

        if pushes != pops {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Stack imbalance detected (push/pop mismatch)".into(),
                suggestion: Some("Ensure pushes match pops".into()),
                fix: None,
            });
        }
    }
}

//
// ASM031 – Push without pop in function
//
#[derive(Clone)]
struct PushWithoutPop;

impl Rule for PushWithoutPop {
    fn id(&self) -> &'static str { "ASM031" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.trim_start().starts_with("push") && !source.contains("ret") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Push without return or pop".into(),
                    suggestion: Some("Check stack discipline".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// ASM040 – Unused label
//
#[derive(Clone)]
struct UnusedLabel;

impl Rule for UnusedLabel {
    fn id(&self) -> &'static str { "ASM040" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for line in source.lines() {
            if line.ends_with(":") {
                let label = line.trim_end_matches(":");
                if source.matches(label).count() == 1 {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: format!("Unused label '{}'", label),
                        suggestion: Some("Remove unused label".into()),
                        fix: None,
                    });
                }
            }
        }
    }
}

//
// ASM050 – Jump to next instruction
//
#[derive(Clone)]
struct JumpToNextInstruction;

impl Rule for JumpToNextInstruction {
    fn id(&self) -> &'static str { "ASM050" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let lines: Vec<&str> = source.lines().collect();
        for i in 0..lines.len().saturating_sub(1) {
            if lines[i].trim_start().starts_with("jmp")
                && lines[i + 1].trim_end().ends_with(":")
            {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Jump to immediately following label".into(),
                    suggestion: Some("Remove redundant jump".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// ASM060 – Hardcoded address
//
#[derive(Clone)]
struct HardcodedAddress;

impl Rule for HardcodedAddress {
    fn id(&self) -> &'static str { "ASM060" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("0x") && line.contains("[") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Possible hardcoded memory address".into(),
                    suggestion: Some("Use labels instead of raw addresses".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// ASM070 – Missing BITS directive
//
#[derive(Clone)]
struct MissingBitsDirective;

impl Rule for MissingBitsDirective {
    fn id(&self) -> &'static str { "ASM070" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        if !source.contains("BITS 64") && !source.contains("BITS 32") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Missing BITS directive".into(),
                suggestion: Some("Add BITS 64 or BITS 32".into()),
                fix: None,
            });
        }
    }
}

//
// ASM080 – Redundant mov
//
#[derive(Clone)]
struct RedundantMov;

impl Rule for RedundantMov {
    fn id(&self) -> &'static str { "ASM080" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("mov") {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() == 2 && parts[0].trim().ends_with(parts[1].trim()) {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Redundant mov in
struction".into(),
                        suggestion: Some("Remove redundant mov".into()),
                        fix: None,
                    });
                }
            }
        }
    }
}

//
// ASM081 – Zero register with mov instead of xor
//
#[derive(Clone)]
struct ZeroWithMovInsteadOfXor;

impl Rule for ZeroWithMovInsteadOfXor {
    fn id(&self) -> &'static str { "ASM081" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("mov") && line.contains(", 0") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Zeroing register with mov".into(),
                    suggestion: Some("Use xor reg, reg for zeroing".into()),
                    fix: None,
                });
            }
        }
    }
}