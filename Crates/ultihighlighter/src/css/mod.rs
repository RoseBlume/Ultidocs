use crate::langs::consts::{
    BATCH_LIGHT, BATCH_DARK,
    CSHARP_LIGHT, CSHARP_DARK,
    CPP_LIGHT, CPP_DARK,
    C_LIGHT, C_DARK,
    CSS_LIGHT, CSS_DARK,
    DART_LIGHT, DART_DARK,
    HTML_LIGHT, HTML_DARK,
    JAVA_LIGHT, JAVA_DARK,
    JAVASCRIPT_LIGHT, JAVASCRIPT_DARK,
    NASM_LIGHT, NASM_DARK,
    PHP_LIGHT, PHP_DARK,
    PYTHON_LIGHT, PYTHON_DARK,
    R_LIGHT, R_DARK,
    RUST_LIGHT, RUST_DARK,
    SHELL_LIGHT, SHELL_DARK,
    SQL_LIGHT, SQL_DARK,
    TYPESCRIPT_LIGHT, TYPESCRIPT_DARK,
};

use std::collections::HashSet;
use ultidocs_macros::include_web;
include_web! {
    BASE       => ("assets/base.css");
    BASE_LIGHT => ("assets/light/base.css");
    BASE_DARK  => ("assets/dark/base.css");
}

pub struct HighlightCss {
    base: HashSet<String>,
    light: HashSet<String>,
    dark: HashSet<String>
}

impl HighlightCss {

    pub fn default() -> Self {
        let mut base = HashSet::new(); 
        base.insert(BASE.to_string());
        
        let mut light = HashSet::new();
        light.insert(BASE_LIGHT.to_string());

        let mut dark = HashSet::new();
        dark.insert(BASE_DARK.to_string());
        
        Self {
            base,
            light,
            dark
        }
    }

    pub fn add_lang(&mut self, lang: &str) {
        let light = match lang.to_lowercase().as_str() {
            "rust" | "rs" => RUST_LIGHT,
            "python" | "py" => PYTHON_LIGHT,
            "javascript" | "js" => JAVASCRIPT_LIGHT,
            "typescript" | "ts" => TYPESCRIPT_LIGHT,

            "html" => HTML_LIGHT,
            "css" => CSS_LIGHT,

            "csharp" | "cs" | "c#" => CSHARP_LIGHT,
            "java" => JAVA_LIGHT,
            "php" => PHP_LIGHT,
            "dart" => DART_LIGHT,

            "c" => C_LIGHT,
            "cpp" | "c++" => CPP_LIGHT,

            "nasm" | "asm" => NASM_LIGHT,
            "r" => R_LIGHT,
            "sql" => SQL_LIGHT,

            "shell" | "bash" | "sh" => SHELL_LIGHT,
            "bat" | "batch" | "cmd" => BATCH_LIGHT,

            _ => "",
        };

        let dark = match lang.to_lowercase().as_str() {
            "rust" | "rs" => RUST_DARK,
            "python" | "py" => PYTHON_DARK,
            "javascript" | "js" => JAVASCRIPT_DARK,
            "typescript" | "ts" => TYPESCRIPT_DARK,

            "html" => HTML_DARK,
            "css" => CSS_DARK,

            "csharp" | "cs" | "c#" => CSHARP_DARK,
            "java" => JAVA_DARK,
            "php" => PHP_DARK,
            "dart" => DART_DARK,

            "c" => C_DARK,
            "cpp" | "c++" => CPP_DARK,

            "nasm" | "asm" => NASM_DARK,
            "r" => R_DARK,
            "sql" => SQL_DARK,

            "shell" | "bash" | "sh" => SHELL_DARK,
            "bat" | "batch" | "cmd" => BATCH_DARK,

            _ => "",
        };

        self.light.insert(light.to_string());
        self.dark.insert(dark.to_string());
    }

    pub fn add_base(&mut self, code: &str) {
        self.base.insert(code.to_string());
    }

    pub fn add_light(&mut self, code: &str) {
        self.light.insert(code.to_string());
    }

    pub fn add_dark(&mut self, code: &str) {
        self.dark.insert(code.to_string());
    }

    pub fn output(&self) -> String {
        let mut output = String::new();
        for css in &self.base {
            output.push_str(css);
        }
        output.push_str("@media (prefers-color-scheme: dark) {");
        for css in &self.dark {
            output.push_str(css);
        }
        output.push_str("}");

        output.push_str("@media (prefers-color-scheme: light) {");
        for css in &self.light {
            output.push_str(css);
        }
        output.push_str("}");
        output
    }

}