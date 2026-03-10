use crate::PageMeta;
use crate::Assets;
use std::path::{Path};
pub mod files;

pub use files::{
    try_fs, 
    try_read_string, 
    try_write,
    // load_css
};

/// Recursively process markdown files and generate normalized HTML



pub fn wrap_html(
    title: &str,
    desc_block: &str,
    body: &str,
    mut site_root: &str,
    assets: &Assets,
) -> String {
    if site_root == "/" {
        site_root = "";
    }

    // let combined_js = format!("{}\n{}", extra_js);
    // let combined_css = format!("{}\n", highlight_css.output(), &assets.get_css());
    let body_html = format!(
r#"<div class="layout">
    <div id="sidebar-container">
        {}
    </div>
    <main class="main">
        {}
    </main>
</div>"#,
        &assets.sidebar_html,
        body,
    );
    format!(
r#"<!DOCTYPE html>
<html>
    <head>
        <meta charset="UTF-8">
        <title>{title}</title>
        {desc}
        <link rel="icon" type="image/x-icon" href="{root}/favicon.ico">

        <style>
            {css}
        </style>
    
    </head>
    <body>

        {body_html}

        <script>
            {js}
        </script>

    </body>
</html>"#,
        title = title,
        desc = desc_block,
        root = site_root,
        body_html = body_html,
        css = assets.get_css(),
        js = assets.js.output()
    )
}

pub fn parse_front_matter(raw: &str, path: &Path)
    -> Result<(PageMeta, String), Box<dyn std::error::Error>>
{
    if !raw.starts_with("---") {
        return Err(format!("Missing front matter in: {}", path.display()).into());
    }

    let parts: Vec<&str> = raw.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(format!("Invalid front matter format in: {}", path.display()).into());
    }

    let meta_block = parts[1];
    let content = parts[2];

    let mut title = None;
    let mut description = None;
    let mut order = None;

    for line in meta_block.lines() {
        let line = line.trim();
        if let Some(value) = line.strip_prefix("title:") {
            title = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("description:") {
            description = Some(value.trim().to_string());
        } else if let Some(value) = line.strip_prefix("order:") {
            order = Some(value.trim().parse()?);
        }
    }

    Ok((
        PageMeta {
            title: title.ok_or(format!("Missing title in front matter: {}", path.display()))?,
            description,
            order: order.ok_or(format!("Missing order in front matter: {}", path.display()))?,
        },
        content.to_string(),
    ))
}



/// Normalize folder/file names to underscores
pub fn normalize_path_segment(segment: &str) -> String {
    segment.replace(' ', "_")
}