mod css;
mod html;
mod js;

#[cfg(feature = "dir")]
use std::error::Error;
#[cfg(feature = "dir")]
use std::fs;
#[cfg(feature = "dir")]
use std::path::Path;

#[cfg(not(any(feature = "minify", feature = "format")))]
compile_error!("At least one of the features `minify` or `format` must be enabled.");

#[cfg(not(any(feature = "html", feature = "css", feature = "js")))]
compile_error!("At least one language feature must be enabled: html, css, js");


// CSS exports
#[cfg(all(feature = "css", feature = "minify"))]
pub use css::minify_css;

#[cfg(all(feature = "css", feature = "format"))]
pub use css::format_css;


// HTML exports
#[cfg(all(feature = "html", feature = "minify"))]
pub use html::minify_html;

#[cfg(all(feature = "html", feature = "format"))]
pub use html::format_html;


// JS exports
#[cfg(all(feature = "js", feature = "minify"))]
pub use js::minify_js;

#[cfg(all(feature = "js", feature = "format"))]
pub use js::format_js;

#[cfg(feature = "dir")]
pub fn process_dir(dir: &Path, minify: bool) -> Result<(), Box<dyn Error>> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            process_dir(&path, minify)?;
            continue;
        }

        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let raw = fs::read_to_string(&path)?;
        let mut processed = raw;

        match ext {

            #[cfg(feature = "html")]
            "html" => {
                if minify {
                    #[cfg(feature = "minify")]
                    {
                        processed = minify_html(&processed);
                        println!("Minified HTML: {}", path.display());
                    }
                } else {
                    #[cfg(feature = "format")]
                    {
                        processed = format_html(&processed);
                        println!("Formatted HTML: {}", path.display());
                    }
                }
            }

            #[cfg(feature = "css")]
            "css" => {
                if minify {
                    #[cfg(feature = "minify")]
                    {
                        processed = minify_css(&processed);
                        println!("Minified CSS: {}", path.display());
                    }
                } else {
                    #[cfg(feature = "format")]
                    {
                        processed = format_css(&processed);
                        println!("Formatted CSS: {}", path.display());
                    }
                }
            }

            #[cfg(feature = "js")]
            "js" | "mjs" => {
                if minify {
                    #[cfg(feature = "minify")]
                    {
                        processed = minify_js(&processed);
                        println!("Minified JS: {}", path.display());
                    }
                } else {
                    #[cfg(feature = "format")]
                    {
                        processed = format_js(&processed);
                        println!("Formatted JS: {}", path.display());
                    }
                }
            }

            _ => continue,
        }

        fs::write(&path, processed)?;
    }

    Ok(())
}