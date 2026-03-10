use std::any::Any;
mod registry;
use registry::*;
mod cards;
mod tabs;
use ultihighlighter::HighlightCss;


pub trait Component: Any {
    fn as_any(&self) -> &dyn Any;
    fn html(&self) -> String;
    fn css(&self, css: &mut HighlightCss);
    fn js(&self, js: &mut crate::Js);
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







pub fn process_components(input: &str, site_root: &str, mut css: &mut HighlightCss, mut js: &mut crate::Js) -> String {
    let mut output = String::new();
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
                    component.js(&mut js);
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

    output
}