use std::collections::HashSet;
use std::any::Any;
mod registry;
use registry::*;
mod cards;
mod tabs;
use ultihighlighter::Css;


pub trait Component: Any {
    fn as_any(&self) -> &dyn Any;
    fn html(&self) -> String;
    fn css(&self, css: &mut Css);
    fn js(&self) -> String;
}

// impl dyn Component {
//     pub fn downcast_ref<T: Component>(&self) -> Option<&T> {
//         self.as_any().downcast_ref::<T>()
//     }
// }

pub trait ComponentParser: Component {
    // const fn tag() -> &'static str;

    fn parse(lines: &mut std::iter::Peekable<std::str::Lines>, site_root: &str)
        -> Option<Self>
    where
        Self: Sized;
}




pub struct ComponentAssets {
    pub html: String,
    pub js: Vec<String>,
}




pub fn process_components(input: &str, site_root: &str, mut css: &mut Css) -> ComponentAssets {
    let mut output = String::new();
    let mut js_set = HashSet::new();

    let mut lines = input.lines().peekable();
    // Use `next()` directly to ensure each iteration consumes a line
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        let mut matched = false;

        for entry in COMPONENTS {

            if trimmed.starts_with(&format!("<{}", entry.tag)) {
                // Parse the component; the parser must consume its lines
                if let Some(component) = (entry.parse)(&mut lines, site_root) {
                    component.css(&mut css);
                    js_set.insert(component.js());
                    output.push_str(&component.html());

                }
                matched = true;
                break;
            }
        }

        // If no component matched, treat this line as plain text
        if !matched {
            output.push_str(line);
            output.push('\n');
        }
    }

    ComponentAssets {
        html: output,
        js: js_set.into_iter().collect(),
    }
}