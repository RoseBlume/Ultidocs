

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

    let indent_str = "    ";
    let mut indent_level = 0;

    let mut in_tag = false;
    let mut in_string = false;
    let mut string_delim = '\0';
    let mut tag_buffer = String::new();
    let mut text_buffer = String::new();

    let mut skip_format = false; // for <pre> / <code>

    while let Some(c) = chars.next() {
        // Inside quoted string in tag
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

        // Start of tag
        if c == '<' {
            // Flush text before tag
            if !text_buffer.trim().is_empty() {
                result.push('\n');
                result.push_str(&indent_str.repeat(indent_level));
                result.push_str(text_buffer.trim());
            }
            text_buffer.clear();

            in_tag = true;
            tag_buffer.push(c);
            continue;
        }

        // End of tag
        if c == '>' && in_tag {
            tag_buffer.push(c);
            let tag = tag_buffer.trim().to_string();
            tag_buffer.clear();
            in_tag = false;

            let lower = tag.to_ascii_lowercase();

            // DOCTYPE or comment — don't indent logic
            if lower.starts_with("<!doctype") || lower.starts_with("<!--") {
                result.push('\n');
                result.push_str(&tag);
                continue;
            }

            // Closing tag
            if lower.starts_with("</") {
                indent_level = indent_level.saturating_sub(1);

                result.push('\n');
                result.push_str(&indent_str.repeat(indent_level));
                result.push_str(&tag);

                if lower == "</pre>" || lower == "</code>" {
                    skip_format = false;
                }

                continue;
            }

            // Opening tag
            result.push('\n');
            result.push_str(&indent_str.repeat(indent_level));
            result.push_str(&tag);

            let is_self_closing =
                lower.ends_with("/>")
                    || lower.starts_with("<meta")
                    || lower.starts_with("<link")
                    || lower.starts_with("<br")
                    || lower.starts_with("<hr")
                    || lower.starts_with("<img")
                    || lower.starts_with("<input");

            if lower == "<pre>" || lower == "<code>" {
                skip_format = true;
            }

            if !is_self_closing {
                indent_level += 1;
            }

            continue;
        }

        // Inside tag
        if in_tag {
            tag_buffer.push(c);
        } else {
            // Text content
            if skip_format {
                result.push(c);
            } else {
                text_buffer.push(c);
            }
        }
    }

    // Process <style> and <script> safely after structure is fixed
    let result = format_tag_contents(&result, "style", crate::css::format_css);
    let result = format_tag_contents(&result, "script", crate::js::format_js);

    result.trim_start().to_string()
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