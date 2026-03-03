use super::LintError;

#[derive(Debug, Default)]
pub struct LintReport {
    pub errors: Vec<LintError>,
}

impl LintReport {
    pub fn push(&mut self, error: LintError) {
        self.errors.push(error);
    }

    pub fn is_clean(&self) -> bool {
        self.errors.is_empty()
    }
}