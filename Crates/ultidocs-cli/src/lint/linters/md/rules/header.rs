use std::path::Path;
use std::collections::HashSet;
use ultilinter::helpers::line_starts;
use ultilinter::{
    LintConfig, LintError, Fix, TextEdit,
    LintReport, Severity, Rule,
};


pub fn rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(HeaderSpacing),
        Box::new(HeaderLevelJump),
        Box::new(HeaderTrailingHashes),
        Box::new(HeaderIncrement),
        Box::new(HeaderSpace),
        Box::new(HeaderDuplicate),
        Box::new(HeaderLevelStart),
    ]
}


//
// ============================================================
// MD003 – Insert space after '#'
// ============================================================
//

#[derive(Clone)]
pub struct HeaderSpacing;

impl Rule for HeaderSpacing {
    fn id(&self) -> &'static str { "MD003" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        let starts = line_starts(source);

        for (i, line) in source.lines().enumerate() {
            if line.starts_with('#') {
                let hash_count = line.chars().take_while(|c| *c == '#').count();

                if line.chars().nth(hash_count) != Some(' ') {
                    let insert_at = starts[i] + hash_count;

                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Missing space after '#'".into(),
                        suggestion: Some("Insert space after '#'".into()),
                        fix: Some(Fix {
                            rule_id: self.id(),
                            edits: vec![TextEdit {
                                range: insert_at..insert_at,
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
// ============================================================
// MD010 – Header level jump
// ============================================================
//

#[derive(Clone)]
pub struct HeaderLevelJump;

impl Rule for HeaderLevelJump {
    fn id(&self) -> &'static str { "MD010" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut last_level = 0;

        for (i, line) in source.lines().enumerate() {
            if let Some(first) = line.chars().next() {
                if first == '#' {
                    let level = line.chars().take_while(|c| *c == '#').count();
                    if last_level != 0 && (level as i32 - last_level as i32).abs() > 1 {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Header level jumps more than one".into(),
                            suggestion: Some("Adjust header level to increment by 1".into()),
                            fix: None,
                        });
                    }
                    last_level = level;
                }
            }
        }
    }
}

//
// ============================================================
// MD014 – Header formatting (trailing hashes)
// ============================================================
//

#[derive(Clone)]
pub struct HeaderTrailingHashes;

impl Rule for HeaderTrailingHashes {
    fn id(&self) -> &'static str { "MD014" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.starts_with('#') && line.trim_end().ends_with('#') {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: line.len(),
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Header should not have trailing '#'".into(),
                    suggestion: Some("Remove trailing '#'".into()),
                    fix: Some(Fix {
                        rule_id: self.id(),
                        edits: vec![TextEdit {
                            range: (line.len() - 1)..line.len(),
                            replacement: String::new(),
                        }],
                    }),
                });
            }
        }
    }
}


//
// ============================================================
// MD037 – Header levels should increment by one
// ============================================================
//
#[derive(Clone)]
pub struct HeaderIncrement;

impl Rule for HeaderIncrement {
    fn id(&self) -> &'static str { "MD037" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut prev_level = 0;

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                let level = trimmed.chars().take_while(|c| *c == '#').count();
                if prev_level != 0 && level > prev_level + 1 {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: format!("Header level jumps from {} to {}", prev_level, level),
                        suggestion: Some("Increment header by one level at a time".into()),
                        fix: None,
                    });
                }
                prev_level = level;
            }
        }
    }
}

//
// ============================================================
// MD038 – No space after # in headers
// ============================================================
//
#[derive(Clone)]
pub struct HeaderSpace;

impl Rule for HeaderSpace {
    fn id(&self) -> &'static str { "MD038" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            if line.starts_with('#') {
                let hash_count = line.chars().take_while(|c| *c == '#').count();
                if line.chars().nth(hash_count) != Some(' ') {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: hash_count + 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Missing space after '#' in header".into(),
                        suggestion: Some("Add a space after the # characters".into()),
                        fix: None,
                    });
                }
            }
        }
    }
}

//
// ============================================================
// MD039 – No duplicate headers
// ============================================================
//
#[derive(Clone)]
pub struct HeaderDuplicate;

impl Rule for HeaderDuplicate {
    fn id(&self) -> &'static str { "MD039" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut seen: HashSet<String> = HashSet::new();

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                let header_text = trimmed.trim_start_matches('#').trim().to_string();
                if seen.contains(&header_text) {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: format!("Duplicate header: '{}'", header_text),
                        suggestion: Some("Ensure header text is unique".into()),
                        fix: None,
                    });
                } else {
                    seen.insert(header_text);
                }
            }
        }
    }
}

//
// ============================================================
// MD040 – Headers should start at level 1 or 2 (# or ##)
// ============================================================
//
#[derive(Clone)]
pub struct HeaderLevelStart;

impl Rule for HeaderLevelStart {
    fn id(&self) -> &'static str { "MD040" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                let level = trimmed.chars().take_while(|c| *c == '#').count();
                if level > 2 {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Header should start at level 1 (#) or 2 (##)".into(),
                        suggestion: Some("Use # or ## for top-level headers".into()),
                        fix: None,
                    });
                }
                break; // only first header matters
            }
        }
    }
}