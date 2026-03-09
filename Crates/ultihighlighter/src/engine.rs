use crate::html::escape_html;

/// Flush current token and wrap in keyword class if needed
pub fn flush_token_class(output: &mut String, token: &mut String, keywords: &[&str]) {
    if token.is_empty() {
        return;
    }

    if keywords.contains(&token.as_str()) {
        output.push_str(&format!(r#"<span class="keyword">{}</span>"#, escape_html(token)));
    } else {
        output.push_str(&escape_html(token));
    }

    token.clear();
}

pub fn highlight_with_classes(
    code: &str,
    keywords: &[&str],
    symbols: &[&str],
    language: &str,
) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut string_char = ' ';
    let mut in_style_block = false;
    let mut in_script_block = false;

    for line in code.lines() {
        let trimmed = line.trim_start();

        // Detect start/end of style/script blocks for HTML
        if language == "html" {
            if trimmed.starts_with("<style") {
                in_style_block = true;
            }
            if trimmed.starts_with("<script") {
                in_script_block = true;
            }

            if in_style_block || in_script_block {
                result.push_str(&escape_html(line));
                result.push('\n');

                if trimmed.ends_with("</style>") {
                    in_style_block = false;
                }
                if trimmed.ends_with("</script>") {
                    in_script_block = false;
                }
                continue; // skip normal highlighting inside these blocks
            }
        }

        let mut token = String::new();
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            // Handle string literals
            if in_string {
                token.push(ch);
                if ch == string_char {
                    result.push_str(&format!(
                        r#"<span class="string">{}</span>"#,
                        escape_html(&token)
                    ));
                    token.clear();
                    in_string = false;
                }
                continue;
            }

            // Start of string
            if ch == '"' || ch == '\'' {
                flush_token_class(&mut result, &mut token, keywords);
                in_string = true;
                string_char = ch;
                token.push(ch);
                continue;
            }

            // Logical operators &&, ||, !
            if (ch == '&' && chars.peek() == Some(&'&'))
                || (ch == '|' && chars.peek() == Some(&'|'))
            {
                flush_token_class(&mut result, &mut token, keywords);
                let op = format!("{}{}", ch, chars.next().unwrap());
                result.push_str(&format!(r#"<span class="logical">{}</span>"#, op));
                continue;
            }

            if ch == '!' {
                flush_token_class(&mut result, &mut token, keywords);
                result.push_str(r#"<span class="logical">!</span>"#);
                continue;
            }

            // Symbols
            let ch_str = ch.to_string();
            if symbols.contains(&ch_str.as_str()) {
                flush_token_class(&mut result, &mut token, keywords);
                result.push_str(&format!(r#"<span class="symbol">{}</span>"#, escape_html(&ch_str)));
                continue;
            }

            // Build alphanumeric token
            if ch.is_alphanumeric() || ch == '_' || ch == '#' {
                token.push(ch);
            } else {
                flush_token_class(&mut result, &mut token, keywords);
                result.push(ch);
            }
        }

        flush_token_class(&mut result, &mut token, keywords);
        result.push('\n');
    }

    format!(
        r#"<pre class="highlight lang-{}"><code>{}</code></pre>"#,
        language, result
    )
}

