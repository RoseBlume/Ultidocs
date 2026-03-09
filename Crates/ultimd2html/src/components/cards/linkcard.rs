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
    description: Option<String>
}




impl LinkCard {
    pub fn new(title: &str, href: &str, site_root: &str) -> Self { 
        Self {
            title: title.to_string(),
            href: href.to_string(),
            site_root: site_root.to_string(), 
            description: None
        }
    }
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub const fn tag() -> &'static str {
        "LinkCard"
    }
}

impl Component for LinkCard {
    fn as_any(&self) -> &dyn Any { self }
    fn html(&self) -> String {
        let desc = if let Some(desc) = &self.description {
            format!(r#"<p class="card-desc">{}</p>"#, desc)
        } else {
            String::new()
        };
        let suffix = if self.href.trim_start().to_lowercase().starts_with("http") {
            ""
        }
        else {
            ".html"
        };
        format!(r#"<li class="link-card">
    <a class="card-link" href="{site_root}{href}{suffix}">
        <h5 class="card-title">{title}</h5>
        {desc}
    </a>
</li>"#, 
            title = self.title, 
            href = self.href, 
            site_root = self.site_root,
            desc = desc,
            suffix = suffix
        )
    }

    fn css(&self, css: &mut ultihighlighter::Css) {
        css.add_base(LINK_CARD_BASE_CSS);
        css.add_light(LINK_CARD_LIGHT_CSS);
        css.add_dark(LINK_CARD_DARK_CSS);
    }

    fn js(&self) -> String {
        String::new()
    }
}

impl ComponentParser for LinkCard {
    fn parse(
        lines: &mut std::iter::Peekable<std::str::Lines>,
        site_root: &str
    ) -> Option<Self> {

        let mut tag_lines = Vec::new();

        while let Some(line) = lines.next() {
            tag_lines.push(line.trim());

            if line.ends_with("/>") {
                break;
            }
        }

        let title = extract_attr(&tag_lines, "title");
        let href = extract_attr(&tag_lines, "href");
        let description = extract_attr(&tag_lines, "description");

        let root = if href.trim_start().to_lowercase().starts_with("http") {
            ""
        } else if site_root == "/" {
            ""
        } else {
            site_root
        };

        let mut card = LinkCard::new(&title, &href, root);

        if !description.is_empty() {
            card = card.with_description(&description);
        }

        Some(card)
    }
}