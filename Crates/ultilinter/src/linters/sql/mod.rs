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
        .add_rule(Box::new(SelectStarUsage))
        .add_rule(Box::new(DeleteWithoutWhere))
        .add_rule(Box::new(UpdateWithoutWhere))
        .add_rule(Box::new(DropTableUsage))
        .add_rule(Box::new(ImplicitJoinSyntax))
        .add_rule(Box::new(NotInWithNullRisk))
        // .add_rule(Box::new(CountStarWherePossibleCountOne))
        // .add_rule(Box::new(OrderByWithoutLimit))
        // .add_rule(Box::new(MissingTransactionControl))
        // .add_rule(Box::new(HardcodedBooleanComparison))
        .add_rule(Box::new(OrInWhereClause))
        // .add_rule(Box::new(CartesianJoinRisk))
        .add_rule(Box::new(CreateTableWithoutPrimaryKey))
        // .add_rule(Box::new(MixedCaseKeywords))
}

//
// SQL001 – Trailing Whitespace
//
#[derive(Clone)]
struct TrailingWhitespace;

impl Rule for TrailingWhitespace {
    fn id(&self) -> &'static str { "SQL001" }
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
// SQL010 – SELECT *
// 
#[derive(Clone)]
struct SelectStarUsage;

impl Rule for SelectStarUsage {
    fn id(&self) -> &'static str { "SQL010" }
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
            if line.to_uppercase().contains("SELECT *") {
                report.push(LintError {
                    file: file.map(|p| p.to_path_buf()),
                    line: i + 1,
                    column: 1,
                    severity: self.severity(),
                    rule_id: self.id(),
                    message: "SELECT * usage detected".into(),
                    suggestion: Some("Specify explicit column names".into()),
                    fix: None,
                });
            }
        }
    }
}

//
// SQL020 – DELETE without WHERE
//
#[derive(Clone)]
struct DeleteWithoutWhere;

impl Rule for DeleteWithoutWhere {
    fn id(&self) -> &'static str { "SQL020" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        let upper = source.to_uppercase();

        if upper.contains("DELETE FROM") && !upper.contains("WHERE") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "DELETE without WHERE clause".into(),
                suggestion: Some("Add WHERE clause to restrict deletion".into()),
                fix: None,
            });
        }
    }
}

//
// SQL021 – UPDATE without WHERE
//
#[derive(Clone)]
struct UpdateWithoutWhere;

impl Rule for UpdateWithoutWhere {
    fn id(&self) -> &'static str { "SQL021" }
    fn severity(&self) -> Severity { Severity::Error }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        let upper = source.to_uppercase();

        if upper.contains("UPDATE ") && !upper.contains("WHERE") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "UPDATE without WHERE clause".into(),
                suggestion: Some("Add WHERE clause to restrict updates".into()),
                fix: None,
            });
        }
    }
}

//
// SQL030 – DROP TABLE usage
//
#[derive(Clone)]
struct DropTableUsage;

impl Rule for DropTableUsage {
    fn id(&self) -> &'static str { "SQL030" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.to_uppercase().contains("DROP TABLE") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "DROP TABLE detected".into(),
                suggestion: Some("Ensure this is intentional and protected".into()),
                fix: None,
            });
        }
    }
}

//
// SQL040 – Implicit join (comma join)
//
#[derive(Clone)]
struct ImplicitJoinSyntax;

impl Rule for ImplicitJoinSyntax {
    fn id(&self) -> &'static str { "SQL040" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.to_uppercase().contains("FROM") && source.contains(",") && !source.to_uppercase().contains("JOIN") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Implicit join detected (comma syntax)".into(),
                suggestion: Some("Use explicit JOIN syntax".into()),
                fix: None,
            });
        }
    }
}

//
// SQL050 – NOT IN with NULL risk
//
#[derive(Clone)]
struct NotInWithNullRisk;

impl Rule for NotInWithNullRisk {
    fn id(&self) -> &'static str { "SQL050" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.to_uppercase().contains("NOT IN") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "NOT IN may behave unexpectedly with NULL values".into(),
                suggestion: Some("Consider NOT EXISTS instead".into()),
                fix: None,
            });
        }
    }
}

//
// SQL060 – OR in WHERE clause (index inefficiency)
//
#[derive(Clone)]
struct OrInWhereClause;

impl Rule for OrInWhereClause {
    fn id(&self) -> &'static str { "SQL060" }
    fn severity(&self) -> Severity { Severity::Info }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        if source.to_uppercase().contains("WHERE") && source.to_uppercase().contains(" OR ") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "OR in WHERE clause may prevent index usage".into(),
                suggestion: Some("Consider UNION or indexed conditions".into()),
                fix: None,
            });
        }
    }
}

//
// SQL070 – CREATE TABLE without PRIMARY KEY
//
#[derive(Clone)]
struct CreateTableWithoutPrimaryKey;

impl Rule for CreateTableWithoutPrimaryKey {
    fn id(&self) -> &'static str { "SQL070" }
    fn severity(&self) -> Severity { Severity::Warning }

    fn check(
        &self,
        file: Option<&Path>,
        source: &str,
        report: &mut LintReport,
        config: &LintConfig,
    ) {
        if !config.is_enabled(self.id()) { return; }

        let upper = source.to_uppercase();
        if upper.contains("CREATE TABLE") && !upper.contains("PRIMARY KEY") {
            report.push(LintError {
                file: file.map(|p| p.to_path_buf()),
                line: 1,
                column: 1,
                severity: self.severity(),
                rule_id: self.id(),
                message: "Table created without PRIMARY KEY".into(),
                suggestion: Some("Define a PRIMARY KEY".into()),
                fix: None,
            });
        }
    }
}