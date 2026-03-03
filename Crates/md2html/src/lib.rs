use highlighter::highlight;

pub fn render_markdown(input: &str, title: &str, site_name: &str) -> String {
    let mut html = String::new();

    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_buffer = String::new();

    let mut in_ul = false;
    let mut in_ol = false;
    let mut in_blockquote = false;
    let mut table_alignments: Vec<&'static str>;

    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    // ---------- HEADER WITH SIDEBAR TOGGLE BUTTON ----------
    html.push_str(&format!(
        r#"<header>
            <button id="sidebar-toggle" aria-label="Toggle Sidebar">☰</button>
            <h1>{}</h1>
        </header>"#,
        site_name
    ));

    html.push_str(&format!(r#"<div id="content"><h1>{}</h1>"#, title));

    while i < lines.len() {
        let line = lines[i];

        // ---------------- CODE BLOCK ----------------
        if line.starts_with("```") {
            if in_code_block {
                html.push_str(&highlight(&code_buffer, &code_lang));
                code_buffer.clear();
                in_code_block = false;
            } else {
                in_code_block = true;
                code_lang = line.trim_start_matches("```").to_string();
            }
            i += 1;
            continue;
        }

        if in_code_block {
            code_buffer.push_str(line);
            code_buffer.push('\n');
            i += 1;
            continue;
        }

        // ---------------- TABLE ----------------
        if line.contains('|') && i + 1 < lines.len() && is_alignment_row(lines[i + 1]) {
            let headers = split_table_row(line);
            table_alignments = parse_alignment(lines[i + 1]);

            html.push_str("<table>\n<thead>\n<tr>");
            for (idx, header) in headers.iter().enumerate() {
                let align = table_alignments.get(idx).copied().unwrap_or("left");
                html.push_str(&format!(
                    r#"<th style="text-align:{}">{}</th>"#,
                    align,
                    parse_inline(header.trim())
                ));
            }
            html.push_str("</tr>\n</thead>\n<tbody>\n");

            i += 2;
            while i < lines.len() && lines[i].contains('|') {
                let row = split_table_row(lines[i]);
                html.push_str("<tr>");
                for (idx, cell) in row.iter().enumerate() {
                    let align = table_alignments.get(idx).copied().unwrap_or("left");
                    html.push_str(&format!(
                        r#"<td style="text-align:{}">{}</td>"#,
                        align,
                        parse_inline(cell.trim())
                    ));
                }
                html.push_str("</tr>\n");
                i += 1;
            }
            html.push_str("</tbody>\n</table>\n");
            continue;
        }

        // ---------------- HEADINGS ----------------
        if line.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", parse_inline(&line[2..])));
        } else if line.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", parse_inline(&line[3..])));
        }

        // ---------------- BLOCKQUOTE ----------------
        else if line.starts_with("> ") {
            if !in_blockquote {
                html.push_str("<blockquote>\n");
                in_blockquote = true;
            }
            html.push_str(&format!("<p>{}</p>\n", parse_inline(&line[2..])));
        }

        // ---------------- ORDERED LIST ----------------
        else if is_ordered_list(line) {
            if !in_ol {
                html.push_str("<ol>\n");
                in_ol = true;
            }
            let content = line.split_once('.').unwrap().1.trim();
            html.push_str(&format!("<li>{}</li>\n", parse_inline(content)));
        }

        // ---------------- UNORDERED LIST ----------------
        else if line.starts_with("- ") {
            if !in_ul {
                html.push_str("<ul>\n");
                in_ul = true;
            }
            html.push_str(&format!("<li>{}</li>\n", parse_inline(&line[2..])));
        }

        // ---------------- PARAGRAPH ----------------
        else if !line.trim().is_empty() {
            html.push_str(&format!("<p>{}</p>\n", parse_inline(line)));
        }

        // ---------------- BLANK LINE ----------------
        else {
            if in_ul { html.push_str("</ul>\n"); in_ul = false; }
            if in_ol { html.push_str("</ol>\n"); in_ol = false; }
            if in_blockquote { html.push_str("</blockquote>\n"); in_blockquote = false; }
        }

        i += 1;
    }

    if in_ul { html.push_str("</ul>\n"); }
    if in_ol { html.push_str("</ol>\n"); }
    if in_blockquote { html.push_str("</blockquote>\n"); }

    html.push_str("</div>");

    html
}


/// Parses inline Markdown (links, bold/italic, inline code, images)
fn parse_inline(text: &str) -> String {
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
fn apply_formatting(text: &str) -> String {
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
fn replace_simple(mut text: String, marker: &str, open: &str, close: &str) -> String {
    let mut toggle = true;
    while let Some(pos) = text.find(marker) {
        text.replace_range(pos..pos + marker.len(), if toggle { open } else { close });
        toggle = !toggle;
    }
    text
}

/// Replaces Markdown-style links or images: [text](url) or ![alt](url)
fn replace_md<F>(text: String, start: &str, mid: &str, builder: F) -> String
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
fn is_ordered_list(line: &str) -> bool {
    if let Some((num, rest)) = line.split_once('.') {
        return num.trim().parse::<usize>().is_ok() && !rest.trim().is_empty();
    }
    false
}

/// Checks if a line is a table alignment row (---, :---, etc.)
fn is_alignment_row(line: &str) -> bool {
    line.split('|').all(|cell| {
        let cell = cell.trim();
        !cell.is_empty() && cell.chars().all(|c| c == '-' || c == ':')
    })
}

/// Parses table alignment from alignment row
fn parse_alignment(line: &str) -> Vec<&'static str> {
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
fn split_table_row(line: &str) -> Vec<String> {
    line.split('|')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/*
/// Escape HTML inside code blocks
fn escape_html(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}
*/