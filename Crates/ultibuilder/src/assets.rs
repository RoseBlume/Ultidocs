
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

const PROD_JS: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/js/prod.js"
    )
);

const DEV_JS: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/js/dev.js"
    )
);


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
    pub sidebar_css: String,
    pub highlight_css: String
}

impl Assets {
    pub fn new(production: bool) -> Self {
        let js = if production {
            PROD_JS.to_string()
        } else {
            DEV_JS.to_string()
        };
        Self {
            sidebar_html: String::new(),
            favicon: FAVICON.to_vec(),
            js,
            main_css: CSS.to_string(),
            sidebar_css: SIDEBAR_CSS.to_string(),
            highlight_css: HIGHLIGHT_CSS.to_string(),
        }
    }
}


// // Creates favicon in dist/favicon.ico
// pub fn write_favicon(build_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let favicon_path = Path::new(build_dir).join("favicon.ico");
//     fs::write(favicon_path, FAVICON)?;
//     Ok(())
// }