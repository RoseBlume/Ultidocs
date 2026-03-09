use crate::components::{Component, ComponentParser};
use crate::helpers::extract_attr;
use std::any::Any;
use super::{
    CARD_BASE_CSS,
    CARD_LIGHT_CSS,
    CARD_DARK_CSS
};
#[derive(Clone)]
pub struct Card {
    title: String,
    body: String
}


impl Card {
    pub fn new(title: &str, body: &str) -> Self {
        Self {
            title: title.to_string(),
            body: body.trim().to_string()
        }
    }

    pub const fn tag() -> &'static str {
        "Card"
    }
}

impl Component for Card {
    fn as_any(&self) -> &dyn Any { self }
    fn html(&self) -> String {
        format!(
r#"<li class="card">
    <div class="card-body">
        <h5 class="card-title">{}</h5>
        <p class="card-desc">{}</p>
    </div>
</li>"#,
            self.title,
            self.body
        )
    }

    fn css(&self, css: &mut ultihighlighter::Css) {
        css.add_base(CARD_BASE_CSS);
        css.add_light(CARD_LIGHT_CSS);
        css.add_dark(CARD_DARK_CSS);
    }

    fn js(&self) -> String {
        String::new()
    }
}


impl ComponentParser for Card {

    fn parse(
        lines: &mut std::iter::Peekable<std::str::Lines>,
        _site_root: &str
    ) -> Option<Self> {

        let mut tag_lines = Vec::new();

        while let Some(line) = lines.next() {
            tag_lines.push(line.trim());

            if line.contains(">") {
                break;
            }
        }

        let title = extract_attr(&tag_lines, "title");

        let mut body = Vec::new();

        while let Some(line) = lines.next() {
            let line = line.trim();

            if line.starts_with("</Card>") {
                break;
            }

            body.push(line);
        }

        Some(Card::new(&title, &body.join(" ")))
    }
}