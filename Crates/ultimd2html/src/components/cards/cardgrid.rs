use crate::components::{Component, ComponentParser, COMPONENTS};
use super::{Card, LinkCard};
use std::any::Any;
pub struct CardGrid {
    cards: Vec<Box<dyn Component>>
}

use super::{
    LINK_CARD_BASE_CSS,
    LINK_CARD_LIGHT_CSS,
    LINK_CARD_DARK_CSS,
    CARD_BASE_CSS,
    CARD_LIGHT_CSS,
    CARD_DARK_CSS,
    CARD_GRID_BASE_CSS
};


impl CardGrid {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub const fn tag() -> &'static str {
        "CardGrid"
    }
}

impl Component for CardGrid {
    fn as_any(&self) -> &dyn Any { self }

    fn html(&self) -> String {
        let mut card_code = String::new();

        for card in &self.cards {
            card_code.push_str(&card.html());
        }

        format!(r#"<ul class="card-grid">{}</ul>"#, card_code)
    }

    fn css(&self, css: &mut ultihighlighter::HighlightCss) {
        css.add_base(CARD_GRID_BASE_CSS);

        let mut has_card = false;
        let mut has_link_card = false;

        for card in &self.cards {
            if card.as_any().is::<Card>() {
                has_card = true;
            }

            if card.as_any().is::<LinkCard>() {
                has_link_card = true;
            }

            if has_card && has_link_card {
                break;
            }
        }

        if has_card {
            css.add_base(CARD_BASE_CSS);
            css.add_light(CARD_LIGHT_CSS);
            css.add_dark(CARD_DARK_CSS);
        }

        if has_link_card {
            css.add_base(LINK_CARD_BASE_CSS);
            css.add_light(LINK_CARD_LIGHT_CSS);
            css.add_dark(LINK_CARD_DARK_CSS);
        }
    }

    fn js(&self, js: &mut crate::Js) {
        js.add("");
    }
}



impl ComponentParser for CardGrid {
    fn parse(
        lines: &mut std::iter::Peekable<std::str::Lines>,
        site_root: &str
    ) -> Option<Self> {

        let mut grid = CardGrid::new();

        while let Some(line) = lines.peek().cloned() {
            let trimmed = line.trim();

            if trimmed.starts_with("</CardGrid>") { lines.next(); break; }

            let mut matched = false;

            for entry in COMPONENTS {
                if trimmed.starts_with(&format!("<{}", entry.tag)) {
                    if let Some(component) = (entry.parse)(lines, site_root) {
                        // Push all components inside CardGrid
                        grid.cards.push(component);
                    }
                    matched = true;
                    break;
                }
            }

            if !matched { lines.next(); }
        }

        Some(grid)
    }
}