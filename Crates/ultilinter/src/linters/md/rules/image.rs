use std::path::Path;
use crate::{
    LintConfig, LintError,
    LintReport, Severity, Rule,
};
use std::collections::HashMap;


pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(IncorrectImageSyntax),
        Box::new(MissingAltText),
        Box::new(ImageUrlNotHttps),
        Box::new(SpacesInImageSyntax),
        Box::new(MissingImageTitle),
        Box::new(ImageFileNotExist),
        Box::new(ImageFilenameSpaces),
        Box::new(DuplicateAltText),
        Box::new(ExternalImageHttps),
    ]
}

//
// ============================================================
// MD005 – Incorrect image syntax
// ============================================================
//
#[derive(Clone)]
pub struct IncorrectImageSyntax;

impl Rule for IncorrectImageSyntax {
    fn id(&self) -> &'static str { "MD005" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.contains("![](") || line.contains("![") {
                let open_bracket = line.find("![");
                let close_paren = line.find(')');
                if open_bracket.is_none() || close_paren.is_none() {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Malformed image syntax".into(),
                        suggestion: Some("Use ![alt text](url)".into()),
                        fix: None,
                    });
                }
            }
        }
    }
}

//
// ============================================================
// MD016 – Missing alt text
// ============================================================
//
#[derive(Clone)]
pub struct MissingAltText;

impl Rule for MissingAltText {
    fn id(&self) -> &'static str { "MD016" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if let Some(start) = line.find("![") {
                let end_bracket = line[start..].find(']').map(|v| v + start);
                if let Some(end) = end_bracket {
                    let alt_text = &line[start+2..end];
                    if alt_text.trim().is_empty() {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: start + 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Image missing alt text".into(),
                            suggestion: Some("Add descriptive alt text".into()),
                            fix: None,
                        });
                    }
                }
            }
        }
    }
}

//
// ============================================================
// MD017 – Image URL is not HTTPS
// ============================================================
//
#[derive(Clone)]
pub struct ImageUrlNotHttps;

impl Rule for ImageUrlNotHttps {
    fn id(&self) -> &'static str { "MD017" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if let Some(start) = line.find("](") {
                let end_paren = line[start+2..].find(')').map(|v| v + start + 2);
                if let Some(end) = end_paren {
                    let url = &line[start+2..end];
                    if !url.starts_with("https://") && !url.is_empty() {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: start + 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Image URL is not HTTPS".into(),
                            suggestion: Some("Use https:// URL".into()),
                            fix: None,
                        });
                    }
                }
            }
        }
    }
}

//
// ============================================================
// MD018 – Spaces inside brackets or parentheses
// ============================================================
//
#[derive(Clone)]
pub struct SpacesInImageSyntax;

impl Rule for SpacesInImageSyntax {
    fn id(&self) -> &'static str { "MD018" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if let Some(start) = line.find("![") {
                let end_bracket = line[start..].find(']').map(|v| v + start);
                let start_paren = line[start..].find('(').map(|v| v + start);
                let end_paren = line[start..].find(')').map(|v| v + start);

                if let (Some(end_b), Some(start_p), Some(end_p)) = (end_bracket, start_paren, end_paren) {
                    if line[start+2..end_b].starts_with(' ') || line[start+2..end_b].ends_with(' ') {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: start + 3,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Spaces inside image alt brackets".into(),
                            suggestion: Some("Remove spaces inside brackets".into()),
                            fix: None,
                        });
                    }
                    if line[start_p+1..end_p].starts_with(' ') || line[start_p+1..end_p].ends_with(' ') {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: start_p + 2,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Spaces inside image URL parentheses".into(),
                            suggestion: Some("Remove spaces inside parentheses".into()),
                            fix: None,
                        });
                    }
                }
            }
        }
    }
}

//
// ============================================================
// MD019 – Missing image title
// ============================================================
//
#[derive(Clone)]
pub struct MissingImageTitle;

impl Rule for MissingImageTitle {
    fn id(&self) -> &'static str { "MD019" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if let Some(start) = line.find("](") {
                let end_paren = line[start+2..].find(')').map(|v| v + start + 2);
                if let Some(end) = end_paren {
                    let content = &line[start+2..end];
                    if !content.contains("\"") {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: start + 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Image missing title".into(),
                            suggestion: Some("Add a title like ![alt](url \"title\")".into()),
                            fix: None,
                        });
                    }
                }
            }
        }
    }
}


//
// ============================================================
// MD020 – Local image file does not exist
// ============================================================
//
#[derive(Clone)]
pub struct ImageFileNotExist;

impl Rule for ImageFileNotExist {
    fn id(&self) -> &'static str { "MD020" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if let Some(start) = line.find("](") {
                let end_paren = line[start+2..].find(')').map(|v| v + start + 2);
                if let Some(end) = end_paren {
                    let path_str = &line[start+2..end];
                    if !path_str.starts_with("http://") && !path_str.starts_with("https://") {
                        if let Some(base_path) = file {
                            let parent = base_path.parent().unwrap_or_else(|| Path::new("."));
                            let image_path = parent.join(path_str);
                            if !image_path.exists() {
                                report.push(LintError {
                                    file: Some(image_path),
                                    line: i + 1,
                                    column: start + 1,
                                    severity: self.severity(),
                                    rule_id: self.id(),
                                    message: "Local image file does not exist".into(),
                                    suggestion: Some("Ensure the image file exists".into()),
                                    fix: None,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

//
// ============================================================
// MD021 – Image filename contains spaces
// ============================================================
//
#[derive(Clone)]
pub struct ImageFilenameSpaces;

impl Rule for ImageFilenameSpaces {
    fn id(&self) -> &'static str { "MD021" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self,
             _file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if let Some(start) = line.find("](") {
                let end_paren = line[start+2..].find(')').map(|v| v + start + 2);
                if let Some(end) = end_paren {
                    let path_str = &line[start+2..end];
                    if path_str.contains(' ') {
                        report.push(LintError {
                            file: None,
                            line: i + 1,
                            column: start + 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Image filename contains spaces".into(),
                            suggestion: Some("Replace spaces with underscores or hyphens".into()),
                            fix: None,
                        });
                    }
                }
            }
        }
    }
}

//
// ============================================================
// MD022 – Duplicate image alt text
// ============================================================
//
#[derive(Clone)]
pub struct DuplicateAltText;

impl Rule for DuplicateAltText {
    fn id(&self) -> &'static str { "MD022" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self,
             _file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut seen: HashMap<String, usize> = HashMap::new();

        for (i, line) in source.lines().enumerate() {
            if let Some(start) = line.find("![") {
                if let Some(end) = line[start..].find(']').map(|v| v + start) {
                    let alt_text = line[start+2..end].trim().to_string();
                    if !alt_text.is_empty() {
                        if let Some(prev_line) = seen.get(&alt_text) {
                            report.push(LintError {
                                file: None,
                                line: i + 1,
                                column: start + 1,
                                severity: self.severity(),
                                rule_id: self.id(),
                                message: format!("Duplicate alt text '{}' previously used on line {}", alt_text, prev_line),
                                suggestion: Some("Use unique alt text for each image".into()),
                                fix: None,
                            });
                        } else {
                            seen.insert(alt_text, i + 1);
                        }
                    }
                }
            }
        }
    }
}

//
// ============================================================
// MD024 – External images must be HTTPS
// ============================================================
//
#[derive(Clone)]
pub struct ExternalImageHttps;

impl Rule for ExternalImageHttps {
    fn id(&self) -> &'static str { "MD024" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self,
             _file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if let Some(start) = line.find("](") {
                let end_paren = line[start+2..].find(')').map(|v| v + start + 2);
                if let Some(end) = end_paren {
                    let url = &line[start+2..end];
                    if url.starts_with("http://") {
                        report.push(LintError {
                            file: None,
                            line: i + 1,
                            column: start + 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "External image URL should use HTTPS".into(),
                            suggestion: Some("Change http:// to https://".into()),
                            fix: None,
                        });
                    }
                }
            }
        }
    }
}