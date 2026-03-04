use std::path::{Path, PathBuf};
use std::fs;
use crate::try_write;

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
pub fn write_dev_script(
    build_dir: &str,
    site_root: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    // Normalize root
    let mut root = site_root.trim().to_string();

    if !root.is_empty() {
        if !root.starts_with('/') {
            root.insert(0, '/');
        }
        if !root.ends_with('/') {
            root.push('/');
        }
    }

    // if root == String::from("/") {
    //     root = String::new();
    // }

    let sidebar_path = if root.is_empty() {
        "sidebar.html".to_string()
    } else {
        format!("{root}sidebar.html")
    };

    let script = format!(r#"document.addEventListener('DOMContentLoaded', async () => {{
    const container = document.querySelector('#sidebar-container');
    if (container) {{
        const res = await fetch('{sidebar}');
        const html = await res.text();
        container.innerHTML = html;
    }}

    const sidebar = document.querySelector('.sidebar');
    const toggleBtn = document.querySelector('#sidebar-toggle');
    if (!sidebar) return;

    function expandParents(element) {{
        let parent = element.closest('details');
        while (parent) {{
            parent.open = true;
            parent = parent.parentElement.closest('details');
        }}
    }}

    function scrollToCurrentLink() {{
        const currentPath = window.location.pathname;
        const allLinks = sidebar.querySelectorAll('a[href]');
        for (const link of allLinks) {{
            const linkPath = new URL(link.href).pathname;
            if (linkPath === currentPath) {{
                link.classList.add('current');
                expandParents(link);
                link.scrollIntoView({{
                    behavior: 'smooth',
                    block: 'center'
                }});
                break;
            }}
        }}
    }}

    if (toggleBtn) {{
        toggleBtn.addEventListener('click', () => {{
            document.body.classList.toggle('sidebar-open');
            if (document.body.classList.contains('sidebar-open')) {{
                scrollToCurrentLink();
            }}
        }});
    }}

    scrollToCurrentLink();
}});"#, sidebar = sidebar_path);

    try_write(
        &std::path::Path::new(build_dir).join("reload.js"),
        &script,
    )?;

    Ok(())
}

// dist/favicon.ico
pub fn write_favicon(build_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let favicon_path = Path::new(build_dir).join("favicon.ico");
    fs::write(favicon_path, FAVICON)?;
    Ok(())
}