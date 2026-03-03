use std::path::Path;
use crate::{LintConfig, LintReport, Rule};

pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

impl Linter {
    /// Create a new empty Linter
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a single rule
    pub fn add_rule(mut self, rule: Box<dyn Rule>) -> Self {
        // Only add if a rule with the same ID doesn't already exist
        if !self.rules.iter().any(|r| r.id() == rule.id()) {
            self.rules.push(rule);
        }
        self
    }

    /// Add multiple rules at once
    pub fn add_rules(mut self, rules: Vec<Box<dyn Rule>>) -> Self {
        for rule in rules {
            if !self.rules.iter().any(|r| r.id() == rule.id()) {
                self.rules.push(rule);
            }
        }
        self
    }

    /// Add multiple rules from a slice, mutating self
    pub fn with_rules(&mut self, rules: &[Box<dyn Rule>]) {
        for rule in rules {
            // Only add if a rule with the same ID doesn't exist
            if !self.rules.iter().any(|r| r.id() == rule.id()) {
                // Move a clone of the Box into the vector
                // Since Box<dyn Rule> is not cloneable by default, we can just push the Box itself if we own it
                // But here it's a reference, so we need a cloneable pattern if needed
                // For simplicity, we assume the caller can provide owned Boxes
                self.rules.push(rule.clone_box());
            }
        }
    }

    /// Run all rules on the given source code
    pub fn run(
        &self,
        file: Option<&Path>,
        source: &str,
        config: &LintConfig,
    ) -> LintReport {
        let mut report = LintReport::default();

        for rule in &self.rules {
            rule.check(file, source, &mut report, config);
        }

        report
    }
}