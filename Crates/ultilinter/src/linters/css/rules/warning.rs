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

        for (idx, _) in source.match_indices(':') {
            if source.get(idx + 1..idx + 2) != Some(" ") {

                let line_number = match starts.binary_search(&idx) {
                    Ok(line) => line,
                    Err(next) => next - 1,
                };

                let column = idx - starts[line_number];

                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: line_number + 1,
                    column: column + 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Missing space after ':'".into(),
                    suggestion: Some("Add space after ':'".into()),
                    fix: Some(Fix {
                        rule_id: self.id(),
                        edits: vec![TextEdit {
                            range: idx + 1..idx + 1,
                            replacement: " ".into(),
                        }],
                    }),
                });
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