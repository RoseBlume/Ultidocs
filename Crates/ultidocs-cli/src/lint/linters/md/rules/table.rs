use std::path::Path;
use ultilinter::{
    LintConfig, LintError, 
    LintReport, Severity, Rule,
};

pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(TableRowPipes),
        Box::new(TrailingPipe),
        Box::new(MissingHeaderSeparator),
        Box::new(HeaderEmptyCell),
        Box::new(InconsistentColumns),
    ]
}

// //
// // ============================================================
// // MD008 – Table linting
// // ============================================================
// //

// #[derive(Clone)]
// pub struct TableLint;

// impl Rule for TableLint {
//     fn id(&self) -> &'static str { "MD008" }
//     fn severity(&self) -> Severity { Severity::Warning }

//     fn check(&self,
//              file: Option<&Path>,
//              source: &str,
//              report: &mut LintReport,
//              config: &LintConfig) {
//         if !config.is_enabled(self.id()) { return; }

//         let mut in_table = false;
//         let mut header_parsed = false;
//         let mut column_count = 0;

//         for (i, line) in source.lines().enumerate() {
//             let trimmed = line.trim();
//             if trimmed.contains('|') {
//                 if !in_table {
//                     in_table = true;
//                     header_parsed = false;
//                     column_count = trimmed.matches('|').count() - 1;
//                 } else if !header_parsed {
//                     // Expect header separator like |---|---|
//                     if !trimmed.chars().all(|c| c == '|' || c == '-' || c == ':' || c.is_whitespace()) {
//                         report.push(LintError {
//                             file: file.map(|p| p.to_path_buf()),
//                             line: i + 1,
//                             column: 1,
//                             severity: self.severity(),
//                             rule_id: self.id(),
//                             message: "Table missing proper header separator".into(),
//                             suggestion: Some("Add |---|---| line after header".into()),
//                             fix: None,
//                         });
//                     }
//                     header_parsed = true;
//                 } else {
//                     let cols = trimmed.matches('|').count() - 1;
//                     if cols != column_count {
//                         report.push(LintError {
//                             file: file.map(|p| p.to_path_buf()),
//                             line: i + 1,
//                             column: 1,
//                             severity: self.severity(),
//                             rule_id: self.id(),
//                             message: "Table row has inconsistent column count".into(),
//                             suggestion: Some("Ensure all rows have same number of columns".into()),
//                             fix: None,
//                         });
//                     }
//                 }
//             } else {
//                 in_table = false;
//             }
//         }
//     }
// }


// //
// // ============================================================
// // MD015 – Table cell alignment
// // ============================================================
// //

// #[derive(Clone)]
// pub struct TableCellAlignment;

// impl Rule for TableCellAlignment {
//     fn id(&self) -> &'static str { "MD015" }
//     fn severity(&self) -> Severity { Severity::Info }

//     fn check(&self, file: Option<&Path>, source: &str,
//              report: &mut LintReport, config: &LintConfig) {
//         if !config.is_enabled(self.id()) { return; }


//         for (i, line) in source.lines().enumerate() {
//             if line.contains('|') {
//                 if line.trim_start().starts_with('|') && !line.trim_end().ends_with('|') {
//                     report.push(LintError {
//                         file: file.map(|p| p.to_path_buf()),
//                         line: i + 1,
//                         column: 1,
//                         severity: self.severity(),
//                         rule_id: self.id(),
//                         message: "Table row should start and end with '|'".into(),
//                         suggestion: Some("Add leading/trailing '|'".into()),
//                         fix: None,
//                     });
//                 }
//             } 
//         }
//     }
// }

//
// ============================================================
// MD025 – Table rows should start and end with '|'
// ============================================================
//
#[derive(Clone)]
pub struct TableRowPipes;

impl Rule for TableRowPipes {
    fn id(&self) -> &'static str { "MD025" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.contains('|') {
                if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Table row should start and end with '|'".into(),
                        suggestion: Some("Add leading/trailing '|'".into()),
                        fix: None,
                    });
                }
            }
        }
    }
}

//
// ============================================================
// MD026 – Table rows should not have extra trailing pipes
// ============================================================
//
#[derive(Clone)]
pub struct TrailingPipe;

impl Rule for TrailingPipe {
    fn id(&self) -> &'static str { "MD026" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.trim_end().ends_with("||") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: line.len(),
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Table row has extra trailing pipe".into(),
                    suggestion: Some("Remove redundant trailing '|'".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// ============================================================
// MD027 – Table must have a header separator
// ============================================================
//
#[derive(Clone)]
pub struct MissingHeaderSeparator;

impl Rule for MissingHeaderSeparator {
    fn id(&self) -> &'static str { "MD027" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut previous_line_is_header = false;

        for (i, line) in source.lines().enumerate() {
            if line.contains('|') {
                if !previous_line_is_header {
                    previous_line_is_header = true;
                } else {
                    let trimmed = line.trim();
                    if !trimmed.chars().all(|c| c == '|' || c == '-' || c == ':' || c.is_whitespace()) {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Missing table header separator".into(),
                            suggestion: Some("Add |---|---| line after header".into()),
                            fix: None,
                        });
                    }
                    break;
                }
            }
        }
    }
}

//
// ============================================================
// MD028 – Header row contains empty cells
// ============================================================
//
#[derive(Clone)]
pub struct HeaderEmptyCell;

impl Rule for HeaderEmptyCell {
    fn id(&self) -> &'static str { "MD028" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains('|') && line.chars().all(|c| c != '-') {
                let cells: Vec<&str> = line.split('|').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
                if cells.len() == 0 {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Header row contains empty cells".into(),
                        suggestion: Some("Add text to header cells".into()),
                        fix: None,
                    });
                }
            }
        }
    }
}

//
// ============================================================
// MD029 – Table rows should have consistent column counts
// ============================================================
//
#[derive(Clone)]
pub struct InconsistentColumns;

impl Rule for InconsistentColumns {
    fn id(&self) -> &'static str { "MD029" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut expected_columns = None;

        for (i, line) in source.lines().enumerate() {
            if line.contains('|') {
                let col_count = line.matches('|').count() - 1;
                match expected_columns {
                    Some(expected) => {
                        if col_count != expected {
                            report.push(LintError {
                                file: file.map(|p| p.to_path_buf()),
                                line: i + 1,
                                column: 1,
                                severity: self.severity(),
                                rule_id: self.id(),
                                message: format!("Table row has {} columns; expected {}", col_count, expected),
                                suggestion: Some("Ensure all rows have same number of columns".into()),
                                fix: None,
                            });
                        }
                    },
                    None => expected_columns = Some(col_count),
                }
            }
        }
    }
}