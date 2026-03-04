use std::path::Path;
use crate::{
    LintConfig, LintError, Fix, TextEdit,
    LintReport, Severity, Rule,
};
use crate::helpers::line_starts;

pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(MissingColonSpace),
        Box::new(ImportantUsage),
    ]
}

//
// CSS002 – Missing space after colon
//

#[derive(Clone)]
pub struct MissingColonSpace;

impl Rule for MissingColonSpace {
    fn id(&self) -> &'static str { "CSS002" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig
    ) {
        if !config.is_enabled(self.id()) { return; }

        let starts = line_starts(source);
        let mut brace_depth: i32 = 0;

        for (line_index, line) in source.lines().enumerate() {
            let trimmed = line.trim();

            // Track braces first
            brace_depth += trimmed.matches('{').count() as i32;
            brace_depth -= trimmed.matches('}').count() as i32;

            // Only check inside blocks
            if brace_depth <= 0 {
                continue;
            }

            // Skip non-declaration lines
            if trimmed.starts_with('@')
                || trimmed.contains('{')
                || trimmed.contains('}')
                || trimmed.starts_with("/*")
                || !trimmed.contains(':')
            {
                continue;
            }

            // Now check colon spacing
            for (idx, _) in line.match_indices(':') {
                // Ignore pseudo-elements
                if line.get(idx + 1..idx + 2) == Some(":") {
                    continue;
                }

                if line.get(idx + 1..idx + 2) != Some(" ") {
                    let global_offset = starts[line_index] + idx;

                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: line_index + 1,
                        column: idx + 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Missing space after ':'".into(),
                        suggestion: Some("Add space after ':'".into()),
                        fix: Some(Fix {
                            rule_id: self.id(),
                            edits: vec![TextEdit {
                                range: global_offset + 1..global_offset + 1,
                                replacement: " ".into(),
                            }],
                        }),
                    });
                }
            }
        }
    }
}

//
// CSS005 – !important usage
//

#[derive(Clone)]
pub struct ImportantUsage;

impl Rule for ImportantUsage {
    fn id(&self) -> &'static str { "CSS005" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig
    ) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("!important") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Avoid using !important".into(),
                    suggestion: Some("Refactor specificity instead".into()),
                    fix: None,
                });
            }
        }
    }
}