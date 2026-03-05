use ultimd2html::render_markdown;
use crate::PageMeta;
use crate::Assets;
use std::path::{Path};
use std::fs;
mod files;

pub use files::{
    try_fs, 
    try_read_string, 
    try_write,
    load_css
};

/// Recursively process markdown files and generate normalized HTML
pub fn process_directory(
    input_dir: &Path,
    output_dir: &Path,
    site_name: &str,
    site_root: &str,
    assets: &Assets
) -> Result<(), Box<dyn std::error::Error>> {

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Normalize folder names
            let normalized_dir = normalize_path_segment(&path.file_name().unwrap().to_string_lossy());
            let new_output = output_dir.join(&normalized_dir);
            fs::create_dir_all(&new_output)?;
            process_directory(&path, &new_output, site_name, site_root, assets)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            let raw = fs::read_to_string(&path)?;
            let (meta, markdown) = parse_front_matter(&raw, &path)?;
            let body = render_markdown(&markdown, &meta.title, site_name, site_root);
            let desc_block = meta.description
                .as_ref()
                .map(|d| format!(r#"<meta name="description" content="{}">"#, d))
                .unwrap_or_default();

            let html = wrap_html(&format!("{} | {}", site_name, &meta.title), &desc_block, &body, site_root, assets);

            // Normalize file names
            let normalized_name = normalize_path_segment(&path.file_stem().unwrap().to_string_lossy());
            let mut output_path = output_dir.join(&normalized_name);
            output_path.set_extension("html");

            fs::write(&output_path, html)?;
            println!("Generated: {} => {}", &meta.title, output_path.display());
        }
    }

    Ok(())
}


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

    let main_css = &assets.main_css;
    let sidebar_css = &assets.sidebar_css;
    let highlight_css = &assets.highlight_css;
    let reload_js = &assets.js;
    let sidebar_html = &assets.sidebar_html;

    format!(
r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<title>{title}</title>
{desc}
<link rel="icon" type="image/x-icon" href="{root}/favicon.ico">

<style>
{main_css}

{sidebar_css}

{highlight_css}
</style>

</head>
<body>

<div class="layout">
    <div id="sidebar-container">
    {sidebar_html}
    </div>
    <main class="main">
        {body}
    </main>
</div>

<script>
{reload_js}
</script>

</body>
</html>"#,
        title = title,
        desc = desc_block,
        body = body,
        root = site_root,
        sidebar_html = sidebar_html,
        main_css = main_css,
        sidebar_css = sidebar_css,
        highlight_css = highlight_css,
        reload_js = reload_js,
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