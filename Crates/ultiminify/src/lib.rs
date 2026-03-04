mod css;
mod html;
mod js;

pub use css::{format_css, minify_css};
pub use html::{format_html, minify_html};
pub use js::{format_js, minify_js};

use std::fs;
use std::path::Path;
use std::error::Error;

pub fn process_dir(dir: &Path, minify: bool) -> Result<(), Box<dyn Error>> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            process_dir(&path, minify)?;
        } else if path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            // Only process text-based assets
            if !matches!(ext, "html" | "css" | "js" | "mjs") {
                continue;
            }

            let raw = fs::read_to_string(&path)?;

            let processed = match ext {
                "html" => {
                    if minify { minify_html(&raw) } else { format_html(&raw) }
                }
                "css" => {
                    if minify { minify_css(&raw) } else { format_css(&raw) }
                }
                "js" | "mjs" => {
                    if minify { minify_js(&raw) } else { format_js(&raw) }
                }
                _ => unreachable!(),
            };

            if minify {
                println!("Minified: {}", path.display());
            } else {
                println!("Formatted: {}", path.display());
            }

            fs::write(&path, processed)?;
        }
    }

    Ok(())
}