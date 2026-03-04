pub fn minify_css(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let mut chars = code.chars().peekable();

    let mut in_string = false;
    let mut string_delim = '\0';
    let mut in_comment = false;

    while let Some(c) = chars.next() {

        // --- Handle comments ---
        if in_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_comment = false;
            }
            continue;
        }

        if !in_string && c == '/' && chars.peek() == Some(&'*') {
            chars.next();
            in_comment = true;
            continue;
        }

        // --- Handle strings ---
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

        if c == '"' || c == '\'' {
            in_string = true;
            string_delim = c;
            result.push(c);
            continue;
        }

        match c {

            // Remove whitespace before these
            '{' | '}' | ':' | ';' | ',' => {
                while result.ends_with(' ') {
                    result.pop();
                }

                if c == '}' && result.ends_with(';') {
                    result.pop();
                }

                result.push(c);
            }

            // Collapse whitespace
            c if c.is_whitespace() => {
                if !result.ends_with(['{', '}', ':', ';', ',', ' ']) {
                    result.push(' ');
                }
            }

            _ => result.push(c),
        }
    }

    result.trim().to_string()
}

pub fn format_css(code: &str) -> String {
    let mut result = String::new();
    let mut chars = code.chars().peekable();

    let mut indent_level = 0;
    let indent = "    ";

    let mut in_string = false;
    let mut string_delim = '\0';
    let mut in_selector = true;

    while let Some(c) = chars.next() {

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

        // Detect string start
        if c == '"' || c == '\'' {
            in_string = true;
            string_delim = c;
            result.push(c);
            continue;
        }

        match c {

            // Enter declaration block
            '{' => {
                in_selector = false;

                // Ensure single space before {
                if !result.ends_with(' ') {
                    result.push(' ');
                }

                result.push_str("{\n");
                indent_level += 1;
                result.push_str(&indent.repeat(indent_level));
            }

            // Exit declaration block
            '}' => {
                indent_level = indent_level.saturating_sub(1);
                result.push('\n');
                result.push_str(&indent.repeat(indent_level));
                result.push('}');
                result.push('\n');
                result.push_str(&indent.repeat(indent_level));

                in_selector = true;
            }

            // Property separator
            ';' => {
                result.push_str(";\n");
                result.push_str(&indent.repeat(indent_level));
            }

            // Multiple selectors
            ',' => {
                result.push_str(", ");
            }

            // Property value separator
            ':' => {
                if in_selector {
                    // part of pseudo-class or pseudo-element
                    result.push(':');
                } else {
                    // normal property:value
                    result.push_str(": ");
                }
            }

            // Whitespace handling
            c if c.is_whitespace() => {
                if in_selector {
                    // Preserve exactly one space in selectors
                    if !result.ends_with(' ') {
                        result.push(' ');
                    }
                }
                // Ignore whitespace inside declarations
            }

            _ => result.push(c),
        }

    }

    result.trim().to_string()
}