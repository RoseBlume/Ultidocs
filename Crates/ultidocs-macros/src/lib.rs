use proc_macro::{TokenStream, TokenTree, Delimiter};
use std::fs;
use std::path::Path;

/// include_web! macro.
/// Reads the file at compile time, then:
/// - In debug builds: formats with `ultiminify::format_css`
/// - In release builds: minifies with `ultiminify::minify_css`
#[proc_macro]
pub fn include_web(input: TokenStream) -> TokenStream {
    let mut output = String::new();
    let mut iter = input.into_iter();

    while let Some(token) = iter.next() {
        let ident = match token {
            TokenTree::Ident(id) => id.to_string(),
            _ => panic!("Expected identifier"),
        };

        // Expect =>
        match iter.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == '=' => (),
            _ => panic!("Expected '=>' after identifier"),
        }
        match iter.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == '>' => (),
            _ => panic!("Expected '=>' after identifier"),
        }

        // Expect ("path")
        let path_literal = match iter.next() {
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => {
                let mut inner = g.stream().into_iter();
                match inner.next() {
                    Some(TokenTree::Literal(lit)) => lit.to_string(),
                    _ => panic!("Expected string literal inside parentheses"),
                }
            }
            _ => panic!("Expected parentheses with string literal"),
        };

        let path_str = path_literal.trim_matches('"');

        // Read file contents at compile time
        let full_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(path_str);
        let raw = fs::read_to_string(&full_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", full_path.display(), e));

        // Transform content
        let transformed = if cfg!(debug_assertions) {
            // format for debug
            ultiminify::format_css(&raw)
        } else {
            // minify for release
            ultiminify::minify_css(&raw)
        };

        // Escape for a Rust string literal
        let escaped = transformed
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n");

        output.push_str(&format!(
            "pub const {}: &str = \"{}\";\n",
            ident, escaped
        ));

        // Optional semicolon
        if let Some(TokenTree::Punct(p)) = iter.next() {
            if p.as_char() != ';' {
                panic!("Expected ';' after item");
            }
        }
    }

    output.parse().unwrap()
}