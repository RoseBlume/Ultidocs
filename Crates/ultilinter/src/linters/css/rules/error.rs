use std::path::Path;
use crate::{
    LintConfig, LintError,
    LintReport, Severity, Rule,
};

pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(MissingSemicolon),
    ]
}

//
// CSS003 – Missing semicolon
//

#[derive(Clone)]
pub struct MissingSemicolon;

impl Rule for MissingSemicolon {
    fn id(&self) -> &'static str { "CSS003" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig
    ) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.contains(':')
                && !trimmed.ends_with(';')
                && !trimmed.ends_with('{')
                && !trimmed.ends_with('}')
            {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: line.len(),
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Missing semicolon at end of property".into(),
                    suggestion: Some("Add ';'".into()),
                    fix: None,
                });
            }
        }
    }
}