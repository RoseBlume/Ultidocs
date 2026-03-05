use crate::config::SidebarItem;
use crate::helpers::{
    parse_front_matter,
    normalize_path_segment
};
use std::path::{Path};
use std::fs;





/// Expand sidebar and normalize slugs
pub fn expand_sidebar(
    items: &mut Vec<SidebarItem>,
    content_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    for item in items.iter_mut() {
        if let Some(auto) = &item.autogenerate {
            item.slug = None;

            let dir_path = Path::new(content_dir).join(&auto.directory);
            if !dir_path.exists() {
                return Err(format!("Missing directory: {}", dir_path.display()).into());
            }

            let mut generated: Vec<(SidebarItem, i32)> = Vec::new();
            for entry in fs::read_dir(&dir_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md") {
                    let stem = path.file_stem().unwrap().to_string_lossy();
                    if stem == "index" {
                        continue;
                    }

                    let raw = fs::read_to_string(&path)?;
                    let (meta, _) = parse_front_matter(&raw, &path)?;

                    // Normalize slug with underscores
                    generated.push((
                        SidebarItem {
                            label: meta.title.clone(),
                            slug: Some(format!(
                                "{}/{}",
                                normalize_path_segment(&auto.directory),
                                normalize_path_segment(&stem)
                            )),
                            collapsed: None,
                            items: None,
                            autogenerate: None,
                        },
                        meta.order,
                    ));
                }
            }

            generated.sort_by_key(|(_, order)| *order);
            item.items = Some(generated.into_iter().map(|(i, _)| i).collect());
        }

        if let Some(children) = &mut item.items {
            expand_sidebar(children, content_dir)?;
        }
    }

    Ok(())
}

/// Validate that sidebar slugs correspond to existing normalized markdown files
// pub fn validate_sidebar(
//     items: &Vec<SidebarItem>,
//     content_dir: &str,
// ) -> Result<(), Box<dyn std::error::Error>> {

//     for item in items {
//         if let Some(slug) = &item.slug {
//             let components: Vec<&str> = slug.split('/').collect();
//             let mut file_path = PathBuf::from(content_dir);
//             for comp in components {
//                 file_path.push(comp);
//             }
//             file_path.set_extension("md");

//             if !file_path.exists() {
//                 return Err(format!("Missing file for slug: {}", file_path.display()).into());
//             }
//         }

//         if let Some(children) = &item.items {
//             validate_sidebar(children, content_dir)?;
//         }
//     }

//     Ok(())
// }

/// Generate sidebar HTML
pub fn generate_sidebar_html(items: &Vec<SidebarItem>, site_root: &str) -> String {
    let tree = generate_sidebar_html_inner(items, true, &site_root);
    format!(r#"<aside class="sidebar">{}</aside>"#, tree)
}

fn generate_sidebar_html_inner(items: &Vec<SidebarItem>, is_root: bool, site_root: &str) -> String {
    let mut root = site_root.trim().to_string();

    if !root.is_empty() {
        if !root.starts_with('/') {
            root.insert(0, '/');
        }
        if !root.ends_with('/') {
            root.push('/');
        }
    }
    
    let mut html = if is_root { String::from(r#"<ul id="rootbar">"#) } else { String::from("<ul>") };

    for item in items {
        let has_children = item.items.as_ref().map(|c| !c.is_empty()).unwrap_or(false);
        if item.slug.is_none() && !has_children {
            continue;
        }

        html.push_str("<li>");

        if has_children {
            let open_attr = if item.collapsed.unwrap_or(false) { "" } else { " open" };
            html.push_str(&format!(r#"<details{}><summary>{}</summary>"#, open_attr, item.label));
            html.push_str(&generate_sidebar_html_inner(item.items.as_ref().unwrap(), false, site_root));
            html.push_str("</details>");
        } else if let Some(slug) = &item.slug {
            html.push_str(&format!(r#"<a href="{}{}.html">{}</a>"#, root, slug, item.label));
        }

        html.push_str("</li>");
    }

    html.push_str("</ul>");
    html
}

