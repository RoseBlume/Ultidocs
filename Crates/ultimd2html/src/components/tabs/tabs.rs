use std::iter::Peekable;
use std::str::Lines;

use crate::components::{Component, ComponentParser};
use std::any::Any;

use super::TabItem;
#[derive(Debug)]
pub struct Tabs {
    sync_key: Option<String>,
    items: Vec<TabItem>,
}
use ultidocs_macros::include_web;
include_web! {
    TABS_BASE_CSS  => ("assets/Tabs/base.css");
    TABS_LIGHT_CSS => ("assets/Tabs/light.css");
    TABS_DARK_CSS  => ("assets/Tabs/dark.css");
    TABS_JS        => ("assets/Tabs/script.js");
}

impl Tabs {
    pub fn new(sync_key: Option<String>) -> Self {
        Self {
            sync_key,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: TabItem) {
        self.items.push(item);
    }

    pub const fn tag() -> &'static str {
        "Tabs"
    }
}

impl Component for Tabs {
    fn as_any(&self) -> &dyn Any { self }
    fn html(&self) -> String {
        let mut buttons = String::new();
        let mut panels = String::new();

        for (i, item) in self.items.iter().enumerate() {
            buttons.push_str(&item.render_button(i));
            panels.push_str(&item.render_panel(i));
        }

        let sync_attr = if let Some(key) = &self.sync_key {
            format!(r#" data-sync-key="{}""#, key)
        } else {
            String::new()
        };

        let html = format!(
r#"<div class="tabs"{sync_attr}>
  <div class="tab-buttons">
    {buttons}
  </div>
  <div class="tab-panels">
    {panels}
  </div>
</div>"#,
            sync_attr = sync_attr,
            buttons = buttons,
            panels = panels
        );
        println!("{}", &html);
        html
    }

    fn css(&self, css: &mut ultihighlighter::HighlightCss) {
        css.add_base(TABS_BASE_CSS);
        css.add_light(TABS_LIGHT_CSS);
        css.add_dark(TABS_DARK_CSS);
    }

    fn js(&self, js: &mut crate::Js) {
        js.add(TABS_JS);
    }
}

impl ComponentParser for Tabs {
    fn parse(
        lines: &mut Peekable<Lines>,
        site_root: &str,
    ) -> Option<Self> {
        // --- Consume the opening <Tabs ...> line ---
        let first_line = lines.next()?.trim();
        let tag_lines = vec![first_line];

        // Extract optional syncKey
        let sync_key = crate::helpers::extract_attr(&tag_lines, "syncKey");
        let sync_key = if sync_key.is_empty() { None } else { Some(sync_key) };

        let mut tabs = Tabs::new(sync_key);

        // --- Parse lines until closing </Tabs> ---
        while let Some(line) = lines.peek().cloned() {
            let trimmed = line.trim();

            // Stop at </Tabs>
            if trimmed.starts_with("</Tabs>") {
                lines.next(); // consume closing tag
                break;
            }

            let mut matched = false;

            // Try to match any registered component
            for entry in crate::components::COMPONENTS {
                if trimmed.starts_with(&format!("<{}", entry.tag)) {
                    if let Some(component) = (entry.parse)(lines, site_root) {
                        // Only add TabItem components to Tabs
                        if let Some(tab_item) = component.as_any().downcast_ref::<TabItem>() {
                            tabs.add_item(tab_item.clone());
                        }
                    }
                    matched = true;
                    break;
                }
            }

            // If line is not a component, skip it
            if !matched {
                lines.next();
            }
        }

        Some(tabs)
    }
}