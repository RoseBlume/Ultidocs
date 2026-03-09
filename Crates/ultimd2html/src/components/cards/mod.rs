
mod cardgrid;
mod linkcard;
mod card;
pub use cardgrid::CardGrid;
pub use linkcard::LinkCard;
pub use card::Card;

use ultidocs_macros::include_web;

include_web! {
    LINK_CARD_BASE_CSS => ("assets/LinkCard/base.css");
    LINK_CARD_LIGHT_CSS => ("assets/LinkCard/light.css");
    LINK_CARD_DARK_CSS => ("assets/LinkCard/dark.css");

    CARD_GRID_BASE_CSS => ("assets/CardGrid/base.css");

    CARD_BASE_CSS => ("assets/Card/base.css");
    CARD_LIGHT_CSS => ("assets/Card/light.css");
    CARD_DARK_CSS => ("assets/Card/dark.css");
}