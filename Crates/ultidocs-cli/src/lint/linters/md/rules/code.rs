use std::path::Path;
use ultilinter::{
    LintConfig, LintError, Fix, TextEdit,
    LintReport, Severity, Rule,
};

pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        // Box::new(CodeBlockLint),
        Box::new(IndentedCodeBlock),
        Box::new(UnclosedCodeBlock),
        Box::new(CodeBlockNoTrailingSpace),
        Box::new(CodeBlockFenceConsistency),
    ]
}

//
// ============================================================
// MD007 – Code block linting (language required)
// ============================================================
//
// #[derive(Clone)]
// pub struct CodeBlockLint;

// impl Rule for CodeBlockLint {
//     fn id(&self) -> &'static str { "MD007" }
//     fn severity(&self) -> Severity { Severity::Warning }

//     fn check(&self, file: Option<&Path>, source: &str,
//              report: &mut LintReport, config: &LintConfig) {
//         if !config.is_enabled(self.id()) { return; }

//         let mut in_code_block = false;

//         for (i, line) in source.lines().enumerate() {
//             if line.trim_start().starts_with("```") {
//                 if in_code_block {
//                     in_code_block = false;
//                 } else {
//                     let lang = line.trim_start().trim_start_matches("```");
//                     if lang.is_empty() {
//                         report.push(LintError {
//                             file: file.map(|p| p.to_path_buf()),
//                             line: i + 1,
//                             column: 1,
//                             severity: self.severity(),
//                             rule_id: self.id(),
//                             message: "Code block missing language specifier".into(),
//                             suggestion: Some("Add language after ```".into()),
//                             fix: None,
//                         });
//                     }
//                     in_code_block = true;
//                 }
//             }
//         }
//     }
// }

//
// ============================================================
// MD012 – Indented code block
// ============================================================
//
#[derive(Clone)]
pub struct IndentedCodeBlock;

impl Rule for IndentedCodeBlock {
    fn id(&self) -> &'static str { "MD012" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.starts_with("    ") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Indented code block".into(),
                    suggestion: Some("Use fenced code block ``` instead".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// ============================================================
// MD013 – Unclosed code block
// ============================================================
//
#[derive(Clone)]
pub struct UnclosedCodeBlock;

impl Rule for UnclosedCodeBlock {
    fn id(&self) -> &'static str { "MD034" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut in_code_block = false;
        let mut start_line = 0;

        for (i, line) in source.lines().enumerate() {
            if line.trim_start().starts_with("```") {
                if in_code_block {
                    in_code_block = false;
                } else {
                    in_code_block = true;
                    start_line = i + 1;
                }
            }
        }

        if in_code_block {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: start_line,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Code block not closed".into(),
                suggestion: Some("Add closing ```".into()),
                fix: None,
            });
        }
    }
}

//
// ============================================================
// MD014 – No trailing whitespace inside code blocks
// ============================================================
//
#[derive(Clone)]
pub struct CodeBlockNoTrailingSpace;

impl Rule for CodeBlockNoTrailingSpace {
    fn id(&self) -> &'static str { "MD035" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut in_code_block = false;

        for (i, line) in source.lines().enumerate() {
            if line.trim_start().starts_with("```") {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block && line.ends_with(' ') {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: line.len(),
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Trailing whitespace in code block".into(),
                    suggestion: Some("Remove trailing spaces".into()),
                    fix: Some(Fix {
                        rule_id: self.id(),
                        edits: vec![TextEdit {
                            range: line.len()-1..line.len(),
                            replacement: "".into(),
                        }],
                    }),
                });
            }
        }
    }
}

//
// ============================================================
// MD015 – Consistent code block fences
// ============================================================
//
#[derive(Clone)]
pub struct CodeBlockFenceConsistency;

impl Rule for CodeBlockFenceConsistency {
    fn id(&self) -> &'static str { "MD036" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut in_code_block = false;

        for (i, line) in source.lines().enumerate() {
            if line.trim_start().starts_with("```") {
                if !in_code_block {
                    in_code_block = true;
                } else {
                    in_code_block = false;
                }

                if line.trim_start().chars().take_while(|c| *c == '`').count() != 3 {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Code block fence should be exactly three backticks".into(),
                        suggestion: Some("Use ``` to fence code blocks".into()),
                        fix: None,
                    });
                }
            }
        }
    }
}