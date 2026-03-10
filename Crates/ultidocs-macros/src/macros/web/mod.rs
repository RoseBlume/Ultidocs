use std::fs;
use std::path::Path;
use proc_macro::{TokenTree, Delimiter};
#[cfg(any(feature = "minify_web", feature = "format_web"))]
mod helpers;

pub fn perform_web(input: String) -> String {
    let stream: proc_macro::TokenStream = input.parse().unwrap();
    let mut output = String::new();
    let mut iter = stream.into_iter();

    while let Some(token) = iter.next() {
        let ident = match token {
            TokenTree::Ident(id) => id.to_string(),
            _ => panic!("Expected identifier"),
        };

        // =>
        match iter.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == '=' => (),
            _ => panic!("Expected '=>'"),
        }

        match iter.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == '>' => (),
            _ => panic!("Expected '=>'"),
        }

        // ("path")
        let path_literal = match iter.next() {
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
                let mut inner = g.stream().into_iter();
                match inner.next() {
                    Some(TokenTree::Literal(lit)) => lit.to_string(),
                    _ => panic!("Expected string literal"),
                }
            }
            _ => panic!("Expected parentheses"),
        };

        let path_str = path_literal.trim_matches('"');

        let full_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(path_str);

        let raw = fs::read_to_string(&full_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", full_path.display(), e));
        #[cfg(any(feature = "minify_web", feature = "format_web"))]
        let transformed = helpers::transform_web(&raw, path_str);
        #[cfg(not(any(feature = "minify_web", feature = "format_web")))]
        let transformed = raw;
        let escaped = transformed
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n");

        output.push_str(&format!(
            "pub const {}: &str = \"{}\";\n",
            ident, escaped
        ));

        if let Some(TokenTree::Punct(p)) = iter.next() {
            if p.as_char() != ';' {
                panic!("Expected ';'");
            }
        }
    }

    output
}