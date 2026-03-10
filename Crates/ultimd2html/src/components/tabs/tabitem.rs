use std::iter::Peekable;
use std::str::Lines;
use std::any::Any;

use crate::components::{Component, ComponentParser};
use crate::helpers::extract_attr;

#[derive(Clone, Debug)]
pub struct TabItem {
    pub label: String,
    pub content: String,
}

impl TabItem {
    pub fn new(label: &str, content: &str) -> Self {
        Self {
            label: label.to_string(),
            content: content.trim().to_string(),
        }
    }

    pub const fn tag() -> &'static str {
        "TabItem"
    }

    pub fn render_panel(&self, index: usize) -> String {
        let mut css = ultihighlighter::HighlightCss::default();
        let html = crate::convert_to_html(&self.content, &mut css);
        let active = if index == 0 { " active" } else { "" };
        format!(
            r#"<div class="tab-panel{active}" data-tab="{index}">
{html}
</div>"#,
            index = index,
            html = html,
            active = active
        )
    }

    pub fn render_button(&self, index: usize) -> String {
        let active = if index == 0 { " active" } else { "" };
        format!(
            r#"<button class="tab-button{active}" data-tab="{index}">
{label}
</button>"#,
            index = index,
            label = self.label,
            active = active
        )
    }
}

impl Component for TabItem {
    fn as_any(&self) -> &dyn Any { self }

    fn html(&self) -> String {
        String::new()
    }

    fn css(&self, _css: &mut ultihighlighter::HighlightCss) { }
    fn js(&self, js: &mut crate::Js) {
        js.add("");
    }
}

impl ComponentParser for TabItem {
    fn parse(
        lines: &mut Peekable<Lines>,
        _site_root: &str,
    ) -> Option<Self> {
        // --- Consume the opening <TabItem ...> line ---
        let first_line = lines.next()?.trim();
        let tag_lines = vec![first_line];

        // Extract label attribute safely
        let label = extract_attr(&tag_lines, "label");

        // --- Collect only the inner content until </TabItem> ---
        let mut content_lines = Vec::new();
        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            
            // Stop at closing tag
            if trimmed.starts_with("</TabItem>") {
                break;
            }

            // Append the line as-is to preserve Markdown/code formatting
            content_lines.push(line);
        }

        // Join content and trim leading/trailing whitespace
        let content = content_lines.join("\n").trim().to_string();

        // Construct TabItem with only inner content
        Some(TabItem::new(&label, &content))
    }
}