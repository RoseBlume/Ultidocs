use std::path::Path;
use crate::helpers::line_starts;
use crate::{
    LintConfig,
    LintError,
    Fix,
    TextEdit,
    LintReport,
    Severity,
    Rule,
    Linter
};

pub fn linter() -> Linter {
    Linter::new()
        .add_rule(Box::new(TrailingWhitespace))
        .add_rule(Box::new(DoubleBlankLines))
        .add_rule(Box::new(HeaderSpacing))
        .add_rule(Box::new(MissingTitleFrontMatter))
        .add_rule(Box::new(IncorrectImageSyntax))
        .add_rule(Box::new(ListFormatting))
        .add_rule(Box::new(CodeBlockLint))
        .add_rule(Box::new(TableLint))
        .add_rule(Box::new(LineLengthLimit))
        .add_rule(Box::new(HeaderLevelJump))
        .add_rule(Box::new(MixedListMarkers))
        .add_rule(Box::new(IndentedCodeBlock))
        .add_rule(Box::new(EOFBlankLine))
        .add_rule(Box::new(HeaderTrailingHashes))
        .add_rule(Box::new(TableCellAlignment))
}



//
// ============================================================
// MD001 – Trailing whitespace
// ============================================================
//

#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "MD001" }
    fn severity(&self) -> Severity { Severity::Info }

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
            let trimmed = line.trim_end();

            if trimmed.len() != line.len() {
                let start = starts[i] + trimmed.len();
                let end = starts[i] + line.len();

                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: trimmed.len() + 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "Trailing whitespace".into(),
                    suggestion: Some("Remove trailing spaces".into()),
                    fix: Some(Fix {
                        rule_id: self.id(),
                        edits: vec![TextEdit {
                            range: start..end,
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
// MD002 – Collapse multiple blank lines
// ============================================================
//

#[derive(Clone)]
struct DoubleBlankLines;

impl Rule for DoubleBlankLines {
    fn id(&self) -> &'static str { "MD002" }
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

        let mut blank_count = 0;

        for (i, line) in source.lines().enumerate() {
            let is_blank = line.trim().is_empty();

            if is_blank {
                blank_count += 1;

                if blank_count > 1 {
                    let start = starts[i];
                    let end = start + line.len();

                    // Also remove the newline following the blank line
                    let newline_len = if source.as_bytes().get(end) == Some(&b'\r')
                        && source.as_bytes().get(end + 1) == Some(&b'\n')
                    {
                        2
                    } else if source.as_bytes().get(end) == Some(&b'\n') {
                        1
                    } else {
                        0
                    };

                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Multiple consecutive blank lines".into(),
                        suggestion: Some("Remove extra blank line".into()),
                        fix: Some(Fix {
                            rule_id: self.id(),
                            edits: vec![TextEdit {
                                range: start..(end + newline_len),
                                replacement: String::new(),
                            }],
                        }),
                    });
                }
            } else {
                blank_count = 0;
            }
        }
    }
}

//
// ============================================================
// MD003 – Insert space after '#'
// ============================================================
//

#[derive(Clone)]
struct HeaderSpacing;

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
// MD004 – Insert missing title in front matter
// ============================================================
//

#[derive(Clone)]
struct MissingTitleFrontMatter;

impl Rule for MissingTitleFrontMatter {
    fn id(&self) -> &'static str { "MD004" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if !source.starts_with("---") {
            return;
        }

        // Find closing front matter marker
        let after_open = &source[3..];
        let Some(close_pos) = after_open.find("\n---") else { return; };

        let close_index = 3 + close_pos + 1; // position of '\n' before closing ---

        let front_matter = &source[..close_index];

        if !front_matter.contains("title:") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Missing title in front matter".into(),
                suggestion: Some("Insert title field".into()),
                fix: Some(Fix {
                    rule_id: self.id(),
                    edits: vec![TextEdit {
                        range: close_index..close_index,
                        replacement: "title: \"Untitled\"\n".into(),
                    }],
                }),
            });
        }
    }
}

//
// ============================================================
// MD005 – Incorrect image syntax
// ============================================================
//

#[derive(Clone)]
struct IncorrectImageSyntax;

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
// MD006 – List formatting
// ============================================================
//

#[derive(Clone)]
struct ListFormatting;

impl Rule for ListFormatting {
    fn id(&self) -> &'static str { "MD006" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut prev_was_list = false;

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            let is_list = trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ");

            if is_list {
                if !prev_was_list && i > 0 && !source.lines().nth(i-1).unwrap_or("").trim().is_empty() {
                    report.push(LintError {
                        file: file.map(|p| p.to_path_buf()),
                        line: i + 1,
                        column: 1,
                        severity: self.severity(),
                        rule_id: self.id(),
                        message: "Missing blank line before list".into(),
                        suggestion: Some("Insert blank line before list".into()),
                        fix: Some(Fix {
                            rule_id: self.id(),
                            edits: vec![TextEdit {
                                range: 0..0, // calculated per line in full implementation
                                replacement: "\n".into(),
                            }],
                        }),
                    });
                }
            }

            prev_was_list = is_list;
        }
    }
}

//
// ============================================================
// MD007 – Code block linting
// ============================================================
//

#[derive(Clone)]
struct CodeBlockLint;

impl Rule for CodeBlockLint {
    fn id(&self) -> &'static str { "MD007" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut in_code_block = false;

        for (i, line) in source.lines().enumerate() {
            if line.trim_start().starts_with("```") {
                if in_code_block {
                    in_code_block = false;
                } else {
                    let lang = line.trim_start().trim_start_matches("```");
                    if lang.is_empty() {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Code block missing language specifier".into(),
                            suggestion: Some("Add language after ```".into()),
                            fix: None,
                        });
                    }
                    in_code_block = true;
                }
            }
        }
    }
}

//
// ============================================================
// MD008 – Table linting
// ============================================================
//

#[derive(Clone)]
struct TableLint;

impl Rule for TableLint {
    fn id(&self) -> &'static str { "MD008" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self,
             file: Option<&Path>,
             source: &str,
             report: &mut LintReport,
             config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut in_table = false;
        let mut header_parsed = false;
        let mut column_count = 0;

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.contains('|') {
                if !in_table {
                    in_table = true;
                    header_parsed = false;
                    column_count = trimmed.matches('|').count() - 1;
                } else if !header_parsed {
                    // Expect header separator like |---|---|
                    if !trimmed.chars().all(|c| c == '|' || c == '-' || c == ':' || c.is_whitespace()) {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Table missing proper header separator".into(),
                            suggestion: Some("Add |---|---| line after header".into()),
                            fix: None,
                        });
                    }
                    header_parsed = true;
                } else {
                    let cols = trimmed.matches('|').count() - 1;
                    if cols != column_count {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Table row has inconsistent column count".into(),
                            suggestion: Some("Ensure all rows have same number of columns".into()),
                            fix: None,
                        });
                    }
                }
            } else {
                in_table = false;
            }
        }
    }
}

//
// ============================================================
// MD009 – Line length limit (default 80 chars)
// ============================================================
//

#[derive(Clone)]
struct LineLengthLimit;

impl Rule for LineLengthLimit {
    fn id(&self) -> &'static str { "MD009" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self, file: Option<&Path>, source: &str,
        report: &mut LintReport, config: &LintConfig
    ) {
        if !config.is_enabled(self.id()) { return; }

        let max_length = 80;
        let _starts = line_starts(source);

        for (i, line) in source.lines().enumerate() {
            if line.len() > max_length {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: max_length + 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: format!("Line exceeds {} characters", max_length),
                    suggestion: Some("Split long line".into()),
                    fix: None,
                });
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
struct HeaderLevelJump;

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
// MD011 – Mixed list markers
// ============================================================
//

#[derive(Clone)]
struct MixedListMarkers;

impl Rule for MixedListMarkers {
    fn id(&self) -> &'static str { "MD011" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        let mut current_marker: Option<&str> = None;

        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ") {
                let marker = &trimmed[0..1];
                if let Some(prev) = current_marker {
                    if prev != marker {
                        report.push(LintError {
                            file: file.map(|p| p.to_path_buf()),
                            line: i + 1,
                            column: 1,
                            severity: self.severity(),
                            rule_id: self.id(),
                            message: "Mixed list markers".into(),
                            suggestion: Some("Use consistent list marker".into()),
                            fix: None,
                        });
                    }
                }
                current_marker = Some(marker);
            } else if trimmed.is_empty() {
                current_marker = None;
            }
        }
    }
}

//
// ============================================================
// MD012 – Indented code block
// ============================================================
//

#[derive(Clone)]
struct IndentedCodeBlock;

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
// MD013 – No trailing blank line at EOF
// ============================================================
//

#[derive(Clone)]
struct EOFBlankLine;

impl Rule for EOFBlankLine {
    fn id(&self) -> &'static str { "MD013" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }

        if !source.ends_with('\n') {
            let len = source.len();
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: source.lines().count(),
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "File should end with newline".into(),
                suggestion: Some("Add newline at EOF".into()),
                fix: Some(Fix {
                    rule_id: self.id(),
                    edits: vec![TextEdit { range: len..len, replacement: "\n".into() }],
                }),
            });
        }
    }
}

//
// ============================================================
// MD014 – Header formatting (trailing hashes)
// ============================================================
//

#[derive(Clone)]
struct HeaderTrailingHashes;

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
// MD015 – Table cell alignment
// ============================================================
//

#[derive(Clone)]
struct TableCellAlignment;

impl Rule for TableCellAlignment {
    fn id(&self) -> &'static str { "MD015" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(&self, file: Option<&Path>, source: &str,
             report: &mut LintReport, config: &LintConfig) {
        if !config.is_enabled(self.id()) { return; }


        for (i, line) in source.lines().enumerate() {
            if line.contains('|') {
                if line.trim_start().starts_with('|') && !line.trim_end().ends_with('|') {
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