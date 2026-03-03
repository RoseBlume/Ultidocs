

pub fn minify_html(code: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let mut chars = code.chars().peekable();
    let mut in_tag = false;
    let mut in_string = false;
    let mut string_delim = '\0';
    let mut skip_minify = false;
    let mut tag_buffer = String::new();

    while let Some(c) = chars.next() {
        if in_string {
            tag_buffer.push(c);
            if c == '\\' {
                if let Some(n) = chars.next() {
                    tag_buffer.push(n);
                }
                continue;
            }
            if c == string_delim {
                in_string = false;
            }
            continue;
        }

        if in_tag && (c == '"' || c == '\'') {
            in_string = true;
            string_delim = c;
            tag_buffer.push(c);
            continue;
        }

        if c == '<' {
            if !tag_buffer.is_empty() {
                result.push_str(tag_buffer.trim());
                tag_buffer.clear();
            }
            in_tag = true;
            tag_buffer.push(c);
            continue;
        }

        if c == '>' && in_tag {
            tag_buffer.push(c);
            let tag = tag_buffer.trim();

            if tag.eq_ignore_ascii_case("<pre>")
                || tag.eq_ignore_ascii_case("<code>")
            {
                skip_minify = true;
            }

            if tag.eq_ignore_ascii_case("</pre>")
                || tag.eq_ignore_ascii_case("</code>")
            {
                skip_minify = false;
            }

            result.push_str(tag);
            tag_buffer.clear();
            in_tag = false;
            continue;
        }

        if in_tag {
            tag_buffer.push(c);
        } else if skip_minify {
            result.push(c);
        } else {
            if !c.is_whitespace() || !result.ends_with(' ') {
                result.push(c);
            }
        }
    }

    let result = format_tag_contents(&result, "style", crate::css::minify_css);
    let result = format_tag_contents(&result, "script", crate::js::minify_js);

    result.trim().to_string()
}

pub fn format_html(code: &str) -> String {
    let mut result = String::new();
    let mut chars = code.chars().peekable();
    let mut indent_level = 0;
    let indent = "    ";
    let mut in_tag = false;
    let mut in_string = false;
    let mut string_delim = '\0';
    let mut tag_buffer = String::new();
    let mut skip_format = false; // skip formatting inside <pre> or <code>

    while let Some(c) = chars.next() {
        // Handle strings inside tags
        if in_string {
            tag_buffer.push(c);
            if c == '\\' {
                if let Some(next) = chars.next() {
                    tag_buffer.push(next);
                }
                continue;
            }
            if c == string_delim {
                in_string = false;
            }
            continue;
        }

        // Detect string start inside tag
        if in_tag && (c == '"' || c == '\'') {
            in_string = true;
            string_delim = c;
            tag_buffer.push(c);
            continue;
        }

        // Detect tag start
        if c == '<' {
            flush_tag_buffer(&mut result, &mut tag_buffer, indent_level);

            in_tag = true;
            tag_buffer.push(c);
            continue;
        }

        // Detect tag end
        if c == '>' && in_tag {
            tag_buffer.push(c);
            let tag_str = tag_buffer.trim();

            // Check if opening <pre> or <code> to skip formatting
            if !skip_format && (tag_str.eq_ignore_ascii_case("<pre>") || tag_str.eq_ignore_ascii_case("<code>")) {
                skip_format = true;
            }

            // Closing tag reduces indent
            if tag_str.starts_with("</") {
                indent_level = indent_level.saturating_sub(1);

                // Detect closing of <pre> or <code>
                if skip_format && (tag_str.eq_ignore_ascii_case("</pre>") || tag_str.eq_ignore_ascii_case("</code>")) {
                    skip_format = false;
                    result.push_str(tag_str);
                    tag_buffer.clear();
                    in_tag = false;
                    continue;
                }
            }

            // Normal formatting for other tags
            if !skip_format {
                result.push('\n');
                result.push_str(&indent.repeat(indent_level));
                result.push_str(tag_str);
                result.push('\n');

                // Opening tag increases indent if not self-closing
                if !tag_str.starts_with("</") && !tag_str.ends_with("/>") {
                    let tag_name = tag_str
                        .trim_start_matches('<')
                        .split_whitespace()
                        .next()
                        .unwrap_or("");
                    if tag_name != "br" && tag_name != "hr" && tag_name != "meta" && tag_name != "link" {
                        indent_level += 1;
                    }
                }
            } else {
                // Inside <pre>/<code>, just append the tag as-is
                result.push_str(tag_str);
            }

            tag_buffer.clear();
            in_tag = false;
            continue;
        }

        if in_tag {
            tag_buffer.push(c);
        } else {
            if !skip_format {
                // Regular text outside tags
                if !c.is_whitespace() || !result.ends_with(' ') {
                    result.push(c);
                }
            } else {
                // Inside <pre>/<code>, preserve text exactly
                result.push(c);
            }
        }
    }

    // Process <style> blocks
    let result = format_tag_contents(&result, "style", crate::css::format_css);

    // Process <script> blocks
    let result = format_tag_contents(&result, "script", crate::js::format_js);

    result.trim().to_string()
}

fn flush_tag_buffer(result: &mut String, buffer: &mut String, _indent_level: usize) {
    if !buffer.is_empty() {
        result.push_str(buffer);
        buffer.clear();
    }
}

pub fn format_tag_contents<F>(html: &str, tag: &str, unminifier: F) -> String
where
    F: Fn(&str) -> String,
{
    let mut result = String::with_capacity(html.len());
    let mut remaining = html;

    let open_tag = format!("<{}", tag);
    let close_tag = format!("</{}>", tag);

    while let Some(start) = remaining.find(&open_tag) {
        // Push content before the tag
        let (before, after_start) = remaining.split_at(start);
        result.push_str(before);

        // Find end of opening tag '>'
        if let Some(end_open) = after_start.find('>') {
            let (tag_open_part, after_tag_open) = after_start.split_at(end_open + 1);
            result.push_str(tag_open_part);

            // Find the closing tag
            if let Some(end) = after_tag_open.find(&close_tag) {
                let (inner, after_inner) = after_tag_open.split_at(end);

                // Only unminify if NOT <pre> or <code>
                let unminified = if tag.eq_ignore_ascii_case("pre") || tag.eq_ignore_ascii_case("code") {
                    inner.to_string() // preserve exactly
                } else {
                    unminifier(inner)
                };

                result.push_str(&unminified);
                result.push_str(&close_tag);

                remaining = &after_inner[close_tag.len()..];
                continue;
            }
        }

        // Fallback: just append remaining
        result.push_str(after_start);
        return result;
    }

    result.push_str(remaining);
    result
}