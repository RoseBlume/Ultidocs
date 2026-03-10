

use std::collections::HashSet;

#[derive(Clone)]
pub struct Js {
    code: HashSet<String>
}

impl Js {
    pub fn new() -> Self {
        Self {
            code: HashSet::new()
        }
    }

    pub fn from(code: &str) -> Self {
        Self {
            code: HashSet::from([code.to_string()])
        }
    }

    pub fn add(&mut self, code: &str) {
        self.code.insert(code.to_string());
    }

    pub fn combine(&mut self, codes: &Self) {
        self.code.extend(codes.code.iter().cloned());
    }

    pub fn output(&self) -> String {
        let mut output = String::new();
        for code in &self.code {
            output.push_str(&format!("\n{}\n", code));
        }
        output
    }
}