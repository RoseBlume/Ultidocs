use std::collections::HashSet;
use crate::helpers::extract_attr;
mod cards;
use cards::{LinkCard, CardGrid};
pub trait Component {
    fn html(&self) -> String;
    fn css(&self) -> String;
    fn js(&self) -> String;
}

pub struct ComponentAssets {
    pub html: String,
    pub css: Vec<String>,
    pub js: Vec<String>,
}


pub fn process_components(input: &str, site_root: &str) -> ComponentAssets {
    let mut output = String::new();
    let mut css_set = HashSet::new();
    let mut js_set = HashSet::new();

    let mut remaining = input;

    while let Some(start) = remaining.find("<CardGrid>") {
        // Push content before component
        output.push_str(&remaining[..start]);
        remaining = &remaining[start + "<CardGrid>".len()..];

        if let Some(end) = remaining.find("</CardGrid>") {
            let inner = &remaining[..end];

            let mut grid = CardGrid::new();

            let mut tag_lines = Vec::new();
            let mut inside_tag = false;

            for line in inner.lines() {
                let line = line.trim();
                if line.starts_with("<LinkCard") {
                    tag_lines.clear();
                    tag_lines.push(line);
                    inside_tag = true;
                    continue;
                }

                if inside_tag {
                    tag_lines.push(line);
                    if line.ends_with("/>") {
                        // parse attributes from all lines of this tag
                        let title = extract_attr(&tag_lines, "title");
                        let href = extract_attr(&tag_lines, "href");
                        let root = if href.trim_start().to_lowercase().starts_with("http") {
                            ""
                        }
                        else if site_root == "/" {
                            ""
                        }
                        else {
                            site_root
                        };
                        let description = extract_attr(&tag_lines, "description");

                        let mut card = LinkCard::new(&title, &href, root);
                        if !description.is_empty() {
                            card = card.with_description(&description);
                        }

                        css_set.insert(card.css());
                        js_set.insert(card.js());

                        grid.add_card(card);

                        inside_tag = false;
                    }
                }
            }

            // Add grid CSS/JS
            css_set.insert(grid.css());
            js_set.insert(grid.js());

            // Append raw HTML without Markdown parsing
            output.push_str(&grid.html());

            remaining = &remaining[end + "</CardGrid>".len()..];
        } else {
            break;
        }
    }

    // Append any remaining Markdown
    output.push_str(remaining);

    ComponentAssets {
        html: output,
        css: css_set.into_iter().collect(),
        js: js_set.into_iter().collect(),
    }
}