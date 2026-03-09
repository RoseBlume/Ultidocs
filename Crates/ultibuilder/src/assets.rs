use ultidocs_macros::include_web;

include_web! {
    CSS         => ("assets/styles/style.css");
    SIDEBAR_CSS => ("assets/styles/sidebar.css");
    // HIGHLIGHT_CSS => ("assets/styles/highlight.css"); // Uncomment if needed
    JS          => ("assets/js/main.js");
}

const FAVICON: &[u8; 4286] = include_bytes!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/favicon.ico"
    )
);
#[derive(Clone)]
pub struct Assets {
    pub sidebar_html: String,
    pub favicon: Vec<u8>,
    pub js: String,
    pub main_css: String,
    pub sidebar_css: String
}

impl Assets {
    pub fn new() -> Self {

        Self {
            sidebar_html: String::new(),
            favicon: FAVICON.to_vec(),
            js: JS.to_string(),
            main_css: CSS.to_string(),
            sidebar_css: SIDEBAR_CSS.to_string(),
        }
    }
}

