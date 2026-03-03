use std::path::{Path, PathBuf};
use std::fs;

const CSS: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/styles/style.css"
    )
);

const SIDEBAR_CSS: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/styles/sidebar.css"
    )
);

const HIGHLIGHT_CSS: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/styles/highlight.css"
    )
);

const JS: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/scripts/reload.js"
    )
);

const FAVICON: &[u8; 4286] = include_bytes!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/favicon.ico"
    )
);

// style.css
pub fn write_css(css_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    //let css_path = Path::new(build_dir).join("styles").join("main.css");
    fs::write(css_path.join("main.css"), CSS)?;
    Ok(())
}

// sidebar.css
pub fn write_sidebar_css(css_path: &PathBuf) -> Result<(), Box<dyn std::
error::Error>> {
    fs::create_dir_all(css_path.parent().unwrap())?;
    fs::write(css_path.join("sidebar.css"), SIDEBAR_CSS)?;
    Ok(())
}

// highlight.css
pub fn write_highlight_css(css_path: &PathBuf) -> Result<(), Box<dyn std
::error::Error>> {
    //let css_path = Path::new(build_dir).join("styles").join("highlight.css");
    fs::create_dir_all(css_path.parent().unwrap())?;
    fs::write(css_path.join("highlight.css"), HIGHLIGHT_CSS)?;
    Ok(())
}

// dist/reload.js
pub fn write_dev_script(build_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let js_path = Path::new(build_dir).join("reload.js");
    fs::write(js_path, JS)?;
    Ok(())
}

// dist/favicon.ico
pub fn write_favicon(build_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let favicon_path = Path::new(build_dir).join("favicon.ico");
    fs::write(favicon_path, FAVICON)?;
    Ok(())
}