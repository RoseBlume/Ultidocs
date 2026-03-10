#[cfg(feature = "minify")]
pub fn minify_js(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let mut chars = code.chars().peekable();

    let mut in_string = false;
    let mut string_delim = '\0';
    let mut in_single_comment = false;
    let mut in_multi_comment = false;

    let mut prev_char: Option<char> = None;

    while let Some(c) = chars.next() {
        // --- Single-line comments ---
        if in_single_comment {
            if c == '\n' {
                in_single_comment = false;
            }
            continue;
        }

        // --- Multi-line comments ---
        if in_multi_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_multi_comment = false;
            }
            continue;
        }

        // --- Strings / template literals ---
        if in_string {
            result.push(c);

            if c == '\\' {
                if let Some(next) = chars.next() {
                    result.push(next);
                }
                continue;
            }

            if c == string_delim {
                in_string = false;
            }

            prev_char = Some(c);
            continue;
        }

        if c == '"' || c == '\'' || c == '`' {
            in_string = true;
            string_delim = c;
            result.push(c);
            prev_char = Some(c);
            continue;
        }

        // --- Comment detection ---
        if c == '/' {
            match chars.peek() {
                Some('/') => {
                    chars.next();
                    in_single_comment = true;
                    continue;
                }
                Some('*') => {
                    chars.next();
                    in_multi_comment = true;
                    continue;
                }
                _ => {}
            }
        }

        // --- Whitespace handling ---
        if c.is_whitespace() {
            let next = chars.peek().copied();

            // Only keep space if merging identifiers/numbers
            if let (Some(prev), Some(next)) = (prev_char, next) {
                if (prev.is_alphanumeric() || prev == '_')
                    && (next.is_alphanumeric() || next == '_')
                {
                    result.push(' ');
                    prev_char = Some(' ');
                }
            }
            continue;
        }

        // --- Remove space before punctuation ---
        if "{}[]();,:+-*/%=<>!&|^~?".contains(c) {
            if result.ends_with(' ') {
                result.pop();
            }
        }

        result.push(c);
        prev_char = Some(c);
    }

    result.trim().to_string()
}

#[cfg(feature = "format")]
pub fn format_js(code: &str) -> String {
    let mut result = String::new();
    let mut chars = code.chars().peekable();

    let mut indent_level = 0;
    let indent = "    ";

    let mut in_string = false;
    let mut string_delim = '\0';
    let mut in_template = false;
    let mut in_comment = false;
    let mut paren_depth: i32 = 0;

    while let Some(c) = chars.next() {
        // Handle comments
        if in_comment {
            result.push(c);
            if c == '\n' {
                in_comment = false;
                result.push_str(&indent.repeat(indent_level));
            }
            continue;
        }

        if c == '/' && chars.peek() == Some(&'/') {
            result.push_str("//");
            chars.next();
            in_comment = true;
            continue;
        }

        // Handle strings
        if in_string {
            result.push(c);
            if c == '\\' {
                if let Some(next) = chars.next() {
                    result.push(next);
                }
                continue;
            }
            if c == string_delim {
                in_string = false;
            }
            continue;
        }

        // Handle template literals
        if in_template {
            result.push(c);
            if c == '\\' {
                if let Some(next) = chars.next() {
                    result.push(next);
                }
                continue;
            }
            if c == '`' {
                in_template = false;
            }
            continue;
        }

        if c == '"' || c == '\'' {
            in_string = true;
            string_delim = c;
            result.push(c);
            continue;
        }

        if c == '`' {
            in_template = true;
            result.push(c);
            continue;
        }

        // Handle operators and punctuation
        if c == '=' {
            // Check for arrow function =>
            if chars.peek() == Some(&'>') {
                chars.next(); // consume '>'
                result.push_str("=>");
                continue;
            }

            // Check for == or ===
            if chars.peek() == Some(&'=') {
                result.push_str("==");
                chars.next();
                if chars.peek() == Some(&'=') {
                    result.push('=');
                    chars.next();
                }
                continue;
            }

            result.push_str(" = ");
            continue;
        }

        if c == '!' {
            if chars.peek() == Some(&'=') {
                result.push('!');
                chars.next();
                if chars.peek() == Some(&'=') {
                    result.push_str("==");
                    chars.next();
                } else {
                    result.push('=');
                }
                continue;
            } else {
                result.push('!');
                continue;
            }
        }

        match c {
            '{' => {
                result.push_str(" {\n");
                indent_level += 1;
                result.push_str(&indent.repeat(indent_level));
            }
            '}' => {
                indent_level = indent_level.saturating_sub(1);
                result.push('\n');
                result.push_str(&indent.repeat(indent_level));
                result.push('}');
                if chars.peek() != Some(&';') {
                    result.push('\n');
                    result.push_str(&indent.repeat(indent_level));
                }
            }
            '(' => {
                paren_depth += 1;
                result.push('(');
            }
            ')' => {
                paren_depth = paren_depth.saturating_sub(1);
                result.push(')');
            }
            ';' => {
                result.push(';');
                if paren_depth == 0 {
                    result.push('\n');
                    result.push_str(&indent.repeat(indent_level));
                } else {
                    result.push(' ');
                }
            }
            ',' => {
                result.push_str(", ");
            }
            c if c.is_whitespace() => {
                if !result.ends_with(' ') && !result.ends_with('\n') {
                    result.push(' ');
                }
            }
            _ => result.push(c),
        }
    }

    result
        .lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}