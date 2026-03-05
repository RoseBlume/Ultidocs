/// Parses inline Markdown (links, bold/italic, inline code, images)
pub fn parse_inline(text: &str) -> String {
    let mut result = String::new();
    let mut remaining = text;

    while let Some(start) = remaining.find('`') {
        // Add everything before the backtick, with formatting applied
        result.push_str(&apply_formatting(&remaining[..start]));

        // Move past opening backtick
        remaining = &remaining[start + 1..];

        if let Some(end) = remaining.find('`') {
            let code_content = &remaining[..end];

            // Escape HTML *inside* the code span only
            let escaped = code_content
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;");

            result.push_str(&format!(
                r#"<code class="inline">{}</code>"#,
                escaped
            ));

            remaining = &remaining[end + 1..]; // skip closing backtick
        } else {
            // No closing backtick found, treat as normal text
            result.push('`');
        }
    }

    // Apply formatting to any remaining text after last backtick
    result.push_str(&apply_formatting(remaining));

    result
}

/// Applies bold, italic, links, images formatting outside code spans
pub fn apply_formatting(text: &str) -> String {
    let mut t = text.to_string();

    // Images ![alt](url)
    t = replace_md(t, "![", "](", |alt, url| {
        format!(r#"<img src="{}" alt="{}" />"#, url, alt)
    });

    // Links [text](url)
    t = replace_md(t, "[", "](", |txt, url| {
        format!(r#"<a href="{}">{}</a>"#, url, txt)
    });

    // Bold + Italic
    t = replace_simple(t, "***", "<strong><em>", "</em></strong>");

    // Bold
    t = replace_simple(t, "**", "<strong>", "</strong>");

    // Italic
    t = replace_simple(t, "*", "<em>", "</em>");

    t
}

/// Replaces simple paired markers like **bold** or *italic*
pub fn replace_simple(mut text: String, marker: &str, open: &str, close: &str) -> String {
    let mut toggle = true;
    while let Some(pos) = text.find(marker) {
        text.replace_range(pos..pos + marker.len(), if toggle { open } else { close });
        toggle = !toggle;
    }
    text
}

/// Replaces Markdown-style links or images: [text](url) or ![alt](url)
pub fn replace_md<F>(text: String, start: &str, mid: &str, builder: F) -> String
where
    F: Fn(&str, &str) -> String,
{
    let mut result = String::new();
    let mut remaining = text.as_str();

    while let Some(start_idx) = remaining.find(start) {
        result.push_str(&remaining[..start_idx]);
        remaining = &remaining[start_idx + start.len()..];

        if let Some(mid_idx) = remaining.find(mid) {
            let label = &remaining[..mid_idx];
            remaining = &remaining[mid_idx + mid.len()..];

            if let Some(end_idx) = remaining.find(')') {
                let url = &remaining[..end_idx];
                result.push_str(&builder(label, url));
                remaining = &remaining[end_idx + 1..];
            } else {
                break;
            }
        } else {
            break;
        }
    }

    result.push_str(remaining);
    result
}

/// Checks if a line is an ordered list item
pub fn is_ordered_list(line: &str) -> bool {
    if let Some((num, rest)) = line.split_once('.') {
        return num.trim().parse::<usize>().is_ok() && !rest.trim().is_empty();
    }
    false
}

/// Checks if a line is a table alignment row (---, :---, etc.)
pub fn is_alignment_row(line: &str) -> bool {
    line.split('|').all(|cell| {
        let cell = cell.trim();
        !cell.is_empty() && cell.chars().all(|c| c == '-' || c == ':')
    })
}

/// Parses table alignment from alignment row
pub fn parse_alignment(line: &str) -> Vec<&'static str> {
    line.split('|').map(|cell| {
        let cell = cell.trim();
        match (cell.starts_with(':'), cell.ends_with(':')) {
            (true, true) => "center",
            (true, false) => "left",
            (false, true) => "right",
            _ => "left",
        }
    }).collect()
}

/// Splits a table row into individual cells
pub fn split_table_row(line: &str) -> Vec<String> {
    line.split('|')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}


pub fn extract_attr(tag_lines: &[&str], attr: &str) -> String {
    let mut value = String::new();

    for line in tag_lines {
        let line = line.trim();
        let pattern = format!(r#"{attr}=""#);
        if let Some(start) = line.find(&pattern) {
            let rest = &line[start + pattern.len()..];
            if let Some(end) = rest.find('"') {
                value = rest[..end].to_string();
                break;
            }
        }
    }

    value
}