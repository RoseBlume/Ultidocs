// use ultidedup::{
//     deduper::remove_duplicates,
//     dedupe_css
// };
use ultihighlighter::highlight;
pub use ultihighlighter::HighlightCss;
mod components;
use components::process_components;
mod helpers;
use helpers::*;
mod js;
mod css;
pub use js::Js;
pub use css::Css;
// This is my entry point
pub fn render_markdown(input: &str, title: &str, site_name: &str, site_root: &str) -> (String, Css, Js) {
    let mut css = HighlightCss::default();
    let mut js = Js::new();
    let processed = process_components(input, site_root, &mut css, &mut js);

    // Now feed processed.html into your existing markdown renderer
    let markdown_html = render_markdown_core(&processed, title, site_name, &mut css);

    let mut final_html = String::new();


    final_html.push_str(&markdown_html);
    
    (final_html, Css::from(&css.output()), js)
}

pub fn render_markdown_core(input: &str, title: &str, site_name: &str, css: &mut HighlightCss) -> String {
    let mut html = String::new();

    // let mut in_code_block = false;
    // let mut code_lang = String::new();
    // let mut code_buffer = String::new();

    // let mut in_ul = false;
    // let mut in_ol = false;
    // let mut in_blockquote = false;

    // let lines: Vec<&str> = input.lines().collect();
    // let mut i = 0;

    // ---------- HEADER ----------
    html.push_str(&format!(
        r#"<header>
            <button id="sidebar-toggle" aria-label="Toggle Sidebar">☰</button>
            <h1>{}</h1>
        </header>"#,
        site_name
    ));

    html.push_str(&format!(r#"<div id="content"><h1>{}</h1>"#, title));

    html.push_str(&convert_to_html(input, css));

    html.push_str("</div>");
    html
}


pub fn convert_to_html(input: &str, mut css: &mut HighlightCss) -> String {
    let mut html = String::new();

    let mut in_code_block = false;
    let mut code_lang = String::new();
    let mut code_buffer = String::new();

    let mut in_ul = false;
    let mut in_ol = false;
    let mut in_blockquote = false;

    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;


    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // ---------------- CODE BLOCK ----------------
        if trimmed.starts_with("```") {
            if in_code_block {
                html.push_str(&highlight(&code_buffer, &code_lang, &mut css));
                code_buffer.clear();
                in_code_block = false;
            } else {
                in_code_block = true;
                code_lang = trimmed.trim_start_matches("```").to_string();
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

        // ---------------- RAW HTML PASSTHROUGH ----------------
        if trimmed.starts_with('<') && trimmed.ends_with('>') {
            html.push_str(line);
            html.push('\n');
            i += 1;
            continue;
        }

        // ---------------- TABLE ----------------
        if trimmed.contains('|')
            && i + 1 < lines.len()
            && is_alignment_row(lines[i + 1])
        {
            let headers = split_table_row(trimmed);
            let alignments = parse_alignment(lines[i + 1]);

            html.push_str("<table>\n<thead>\n<tr>");

            for (idx, header) in headers.iter().enumerate() {
                let align = alignments.get(idx).copied().unwrap_or("left");
                html.push_str(&format!(
                    r#"<th style="text-align:{}">{}</th>"#,
                    align,
                    parse_inline(header)
                ));
            }

            html.push_str("</tr>\n</thead>\n<tbody>\n");

            i += 2;

            while i < lines.len() && lines[i].contains('|') {
                let row = split_table_row(lines[i]);
                html.push_str("<tr>");

                for (idx, cell) in row.iter().enumerate() {
                    let align = alignments.get(idx).copied().unwrap_or("left");
                    html.push_str(&format!(
                        r#"<td style="text-align:{}">{}</td>"#,
                        align,
                        parse_inline(cell)
                    ));
                }

                html.push_str("</tr>\n");
                i += 1;
            }

            html.push_str("</tbody>\n</table>\n");
            continue;
        }

        // ---------------- HEADINGS ----------------
        if trimmed.starts_with("# ") {
            let content = parse_inline(&trimmed[2..]);
            let id = trimmed[2..].to_lowercase().replace(' ', "-").replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            html.push_str(&format!(r#"<h1 id="{}">{}</h1>"#, id, content));
        } else if trimmed.starts_with("## ") {
            let content = parse_inline(&trimmed[3..]);
            let id = trimmed[3..].to_lowercase().replace(' ', "-").replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            html.push_str(&format!(r#"<h2 id="{}">{}</h2>"#, id, content));
        }

        // ---------------- BLOCKQUOTE ----------------
        else if trimmed.starts_with(">") {
            if !in_blockquote {
                html.push_str("<blockquote>\n");
                in_blockquote = true;
            }
            html.push_str(&format!("<p>{}</p>\n", parse_inline(&trimmed[1..])));
        }

        // ---------------- ORDERED LIST ----------------
        else if is_ordered_list(trimmed) {
            if !in_ol {
                html.push_str("<ol>\n");
                in_ol = true;
            }
            let content = trimmed.split_once('.').unwrap().1.trim();
            html.push_str(&format!("<li>{}</li>\n", parse_inline(content)));
        }

        // ---------------- UNORDERED LIST ----------------
        else if trimmed.starts_with("- ") {
            if !in_ul {
                html.push_str("<ul>\n");
                in_ul = true;
            }
            html.push_str(&format!("<li>{}</li>\n", parse_inline(&trimmed[2..])));
        }

        // ---------------- PARAGRAPH ----------------
        else if !trimmed.is_empty() {
            html.push_str(&format!("<p>{}</p>\n", parse_inline(trimmed)));
        }

        // ---------------- BLANK LINE ----------------
        else {
            if in_ul {
                html.push_str("</ul>\n");
                in_ul = false;
            }
            if in_ol {
                html.push_str("</ol>\n");
                in_ol = false;
            }
            if in_blockquote {
                html.push_str("</blockquote>\n");
                in_blockquote = false;
            }
        }

        i += 1;
    }

    // Close any open structures
    if in_ul {
        html.push_str("</ul>\n");
    }
    if in_ol {
        html.push_str("</ol>\n");
    }
    if in_blockquote {
        html.push_str("</blockquote>\n");
    }

    html
}


/*
/// Escape HTML inside code blocks
fn escape_html(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}
*/