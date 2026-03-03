#[derive(Debug)]
pub struct LintConfig {
    disabled_rules: Vec<&'static str>,
}

impl LintConfig {
    pub fn new() -> Self {
        Self {
            disabled_rules: Vec::new(),
        }
    }

    pub fn disable(mut self, id: &'static str) -> Self {
        self.disabled_rules.push(id);
        self
    }

    pub fn is_enabled(&self, id: &str) -> bool {
        !self.disabled_rules.contains(&id)
    }
}