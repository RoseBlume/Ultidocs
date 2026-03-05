use crate::components::Component;
use super::linkcard::LinkCard;
pub struct CardGrid {
    cards: Vec<LinkCard>
}

const CARD_GRID_CSS: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/CardGrid/style.css"
    )
);



impl CardGrid {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: LinkCard) {
        self.cards.push(card)
    }
}

impl Component for CardGrid {
    fn html(&self) -> String {
        let mut card_code = String::new();
        for card in self.cards.iter() {
            card_code.push_str(&card.clone().html())
        }
        format!(r#"<ul class="card-grid">{}</ul>"#, &card_code)
    }
    fn css(&self) -> String {
        CARD_GRID_CSS.to_string()
    }
    fn js(&self) -> String {
        String::new()
    }
}