use crate::components::{Component, ComponentParser};
use crate::helpers::extract_attr;
use std::any::Any;
use super::{
    LINK_CARD_BASE_CSS,
    LINK_CARD_LIGHT_CSS,
    LINK_CARD_DARK_CSS
};

#[derive(Clone)]
pub struct LinkCard {
    title: String,
    href: String,
    site_root: String,
    description: String,
}

impl LinkCard {
    pub fn new(title: &str, href: &str, site_root: &str, description: &str) -> Self { 
        Self {
            title: title.to_string(),
            href: href.to_string(),
            site_root: site_root.to_string(),
            description: description.trim().to_string(),
        }
    }

    pub const fn tag() -> &'static str {
        "LinkCard"
    }
}

impl Component for LinkCard {
    fn as_any(&self) -> &dyn Any { self }

    fn html(&self) -> String {
        let suffix = if self.href.trim_start().to_lowercase().starts_with("http") {
            ""
        } else {
            ".html"
        };
        format!(
r#"<li class="link-card">
    <a class="card-link" href="{site_root}{href}{suffix}">
        <div class="card-body">
            <h5 class="card-title">{title}</h5>
            <p class="card-desc">{desc}</p>
        </div>
    </a>
</li>"#,
            site_root = self.site_root,
            href = self.href,
            suffix = suffix,
            title = self.title,
            desc = self.description
        )
    }

    fn css(&self, css: &mut ultihighlighter::HighlightCss) {
        css.add_base(LINK_CARD_BASE_CSS);
        css.add_light(LINK_CARD_LIGHT_CSS);
        css.add_dark(LINK_CARD_DARK_CSS);
    }

    fn js(&self, js: &mut crate::Js) {
        js.add("");
    }
}

impl ComponentParser for LinkCard {
    fn parse(
        lines: &mut std::iter::Peekable<std::str::Lines>,
        site_root: &str
    ) -> Option<Self> {
        let mut tag_lines = Vec::new();

        // Collect opening tag lines
        while let Some(line) = lines.next() {
            let line = line.trim();
            tag_lines.push(line);

            if line.ends_with(">") {
                break;
            }
        }

        // Extract attributes
        let title = extract_attr(&tag_lines, "title");
        let href = extract_attr(&tag_lines, "href");

        // Read inner content until </LinkCard>
        let mut description = Vec::new();
        while let Some(line) = lines.next() {
            let line = line.trim();
            if line.starts_with("</LinkCard>") {
                break;
            }
            description.push(line);
        }

        let desc_str = description.join(" ").trim().to_string();

        let root = if href.trim_start().to_lowercase().starts_with("http") || site_root == "/" {
            ""
        } else {
            site_root
        };

        Some(LinkCard::new(&title, &href, root, &desc_str))
    }
}