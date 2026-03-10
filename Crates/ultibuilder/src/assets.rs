use ultidocs_macros::include_web;
use ultimd2html::{Js, Css};
use crate::Config;
use crate::helpers::files::try_read_string;

use std::path::Path;

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
    pub js: Js,
    pub main_css: String,
    pub sidebar_css: String,
    pub custom_css: Css,
}

impl Assets {
    pub fn from_config(config: &Config) -> Self {
        let mut js = Js::from(JS);
        if let Some(files) = &config.custom_js {
            for file in files {
                let new_code = try_read_string(&Path::new(&file)).unwrap();
                js.add(&new_code);
            }
        }
        
        let mut css = Css::new();

        if let Some(files) = &config.custom_css {
            for file in files {
                let new_code = try_read_string(&Path::new(&file)).unwrap();
                css.add(&new_code);
            }
        }
        Self {
            sidebar_html: String::new(),
            favicon: FAVICON.to_vec(),
            js,
            main_css: CSS.to_string(),
            sidebar_css: SIDEBAR_CSS.to_string(),
            custom_css: css

        }
    }

    pub fn get_css(&self) -> String {
        let mut output = String::from(&self.main_css);
        output.push_str(&self.sidebar_css);
        output.push_str(&self.custom_css.output());
        output
    }
}

