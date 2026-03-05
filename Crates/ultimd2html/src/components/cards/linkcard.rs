use crate::components::Component;
#[derive(Clone)]
pub struct LinkCard {
    title: String,
    href: String,
    site_root: String,
    description: Option<String>
}

const LINK_CARD_CSS: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/LinkCard/style.css"
    )
);


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
}

impl Component for LinkCard {
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

    fn css(&self) -> String {
        LINK_CARD_CSS.to_string()
    }

    fn js(&self) -> String {
        String::new()
    }
}