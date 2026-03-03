use std::path::Path;
use super::{Severity, LintReport, LintConfig};

pub trait Rule: RuleClone + Send + Sync {
    fn id(&self) -> &'static str;
    fn severity(&self) -> Severity;
    fn check(&self, file: Option<&Path>, source: &str, report: &mut LintReport, config: &LintConfig);
}

// This is just for clone
pub trait RuleClone {
    fn clone_box(&self) -> Box<dyn Rule>;
}

impl<T> RuleClone for T
where
    T: 'static + Rule + Clone,
{
    fn clone_box(&self) -> Box<dyn Rule> {
        Box::new(self.clone())
    }
}

// Now you can implement Clone for Box<dyn Rule>
impl Clone for Box<dyn Rule> {
    fn clone(&self) -> Box<dyn Rule> {
        self.clone_box()
    }
}