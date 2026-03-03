use crate::PageMeta;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path};
use std::error::Error;


pub fn wrap_html(
    title: &str,
    desc_block: &str,
    body: &str,
) -> String {
    format!(
r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<title>{}</title>
{}
<link rel="stylesheet" href="/styles/main.css">
<link rel="stylesheet" href="/styles/sidebar.css">
<link rel="stylesheet" href="/styles/highlight.css">
</head>
<body>

<div class="layout">
    <div id="sidebar-container"></div>
    <main class="main">
        {}
    </main>
</div>

<script src="/reload.js"></script>

</body>
</html>"#,
        title,
        desc_block,
        body
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


// === Helper functions for robust FS operations ===

// General path-aware FS wrapper
pub fn try_fs<F, T>(path: &Path, op: F) -> Result<T, Box<dyn Error>>
where
    F: FnOnce(&Path) -> io::Result<T>,
{
    op(path).map_err(|e| {
        let msg = format!("Error accessing path '{}': {}", path.display(), e);
        Box::<dyn Error>::from(msg)
    })
}

// Read file safely with path context
pub fn try_read_string(path: &Path) -> Result<String, Box<dyn Error>> {
    fs::read_to_string(path).map_err(|e| {
        format!("Failed to read '{}': {}", path.display(), e).into()
    })
}

// Write file safely, handling read-only files on Windows
pub fn try_write(path: &Path, data: &str) -> Result<(), Box<dyn Error>> {
    // Remove read-only attribute if present (Windows)
    #[cfg(windows)]
    {
        
        if path.exists() {
            let mut perms = fs::metadata(path)?.permissions();
            perms.set_readonly(false);
            fs::set_permissions(path, perms)?;
        }
    }

    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .and_then(|mut f| f.write_all(data.as_bytes()))
        .map_err(|e| format!("Failed to write '{}': {}", path.display(), e).into())
}



