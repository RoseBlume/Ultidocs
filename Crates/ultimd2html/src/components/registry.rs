use std::iter::Peekable;
use std::str::Lines;


use crate::components::{Component, ComponentParser};

use crate::components::{
    cards::{CardGrid, Card, LinkCard},
    tabs::{Tabs, TabItem},
};

// Type for generic parser function
pub type ParseFn =
    fn(&mut Peekable<Lines>, &str) -> Option<Box<dyn Component>>;

// Registry entry
pub struct ComponentEntry {
    pub tag: &'static str,
    pub parse: ParseFn,
}

// ----------------------------
// Wrapper functions for registry
// ----------------------------

// CardGrid parser wrapper
fn parse_cardgrid(
    lines: &mut Peekable<Lines>,
    site_root: &str
) -> Option<Box<dyn Component>> {
    CardGrid::parse(lines, site_root)
        .map(|c| Box::new(c) as Box<dyn Component>)
}

// Tabs parser wrapper
fn parse_tabs(
    lines: &mut Peekable<Lines>,
    site_root: &str
) -> Option<Box<dyn Component>> {
    Tabs::parse(lines, site_root)
        .map(|c| Box::new(c) as Box<dyn Component>)
}

// Card parser wrapper
fn parse_card(
    lines: &mut Peekable<Lines>,
    site_root: &str
) -> Option<Box<dyn Component>> {
    Card::parse(lines, site_root)
        .map(|c| Box::new(c) as Box<dyn Component>)
}

// LinkCard parser wrapper
fn parse_link_card(
    lines: &mut Peekable<Lines>,
    site_root: &str
) -> Option<Box<dyn Component>> {
    LinkCard::parse(lines, site_root)
        .map(|c| Box::new(c) as Box<dyn Component>)
}

fn parse_tab_item(
    lines: &mut Peekable<Lines>,
    site_root: &str
) -> Option<Box<dyn Component>> {
    TabItem::parse(lines, site_root)
        .map(|c| Box::new(c) as Box<dyn Component>)
}



// ----------------------------
// Component registry
// ----------------------------

pub const COMPONENTS: &[ComponentEntry] = &[
    ComponentEntry { tag: Tabs::tag(), parse: parse_tabs },
    ComponentEntry { tag: CardGrid::tag(), parse: parse_cardgrid },
    ComponentEntry { tag: Card::tag(), parse: parse_card },
    ComponentEntry { tag: LinkCard::tag(), parse: parse_link_card },
    ComponentEntry { tag: TabItem::tag(), parse: parse_tab_item}
];