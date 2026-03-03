mod config;
mod defaults;
mod helpers;
mod sidebar;

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::error::Error;

use sidebar::{expand_sidebar, generate_sidebar_html, process_directory};
pub use config::Config;
use config::SidebarItem;
use helpers::{parse_front_matter, wrap_html, try_fs, try_read_string, try_write};
use md2html::render_markdown;
use ultiminify::{minify_css, minify_js, format_css, format_js};

#[derive(Debug)]
struct PageMeta {
    title: String,
    description: Option<String>,
    order: i32,
}

pub struct Builder {
    config: Config,
    production: bool,
}

impl Builder {
    pub fn build_fresh(config: &Config, production: bool) -> Result<Self, Box<dyn Error>> {
        let build_path = Path::new(&config.build_dir);

        if build_path.exists() {
            fn unlock_dir(path: &Path) -> io::Result<()> {
                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        unlock_dir(&path)?;
                    } else {
                        #[cfg(windows)]
                        {
                            let mut perms = fs::metadata(&path)?.permissions();
                            perms.set_readonly(false);
                            fs::set_permissions(&path, perms)?;
                        }
                    }
                }
                Ok(())
            }
            unlock_dir(build_path)?;
            try_fs(build_path, |p| fs::remove_dir_all(p))?;
        }

        try_fs(build_path, |p| fs::create_dir_all(p))?;
        try_fs(&build_path.join("styles"), |p| fs::create_dir_all(p))?;

        let mut builder = Self { config: config.clone(), production };
        builder.write_assets()?;
        builder.build_all_markdown()?;
        builder.regenerate_sidebar()?;

        if production {
            ultiminify::process_dir(build_path, true)?;
        }

        Ok(builder)
    }

    fn build_all_markdown(&mut self) -> Result<(), Box<dyn Error>> {
        process_directory(
            Path::new(&self.config.content_dir),
            Path::new(&self.config.build_dir),
            &self.config.title,
        )
    }

    pub fn rebuild_markdown(&mut self, md_path: &Path) -> Result<(), Box<dyn Error>> {
        let raw = try_read_string(md_path)?;
        let (meta, markdown) = parse_front_matter(&raw, md_path)?;
        let body = render_markdown(&markdown, &meta.title, &self.config.title);
        let desc_block = meta.description
            .as_ref()
            .map(|d| format!(r#"<meta name="description" content="{}">"#, d))
            .unwrap_or_default();
        let html = wrap_html(&meta.title, &desc_block, &body);

        let relative = md_path.strip_prefix(&self.config.content_dir)?;
        let mut output_path = PathBuf::from(&self.config.build_dir);

        // Normalize folder components
        let comps: Vec<_> = relative.components().collect();
        for comp in &comps[..comps.len() - 1] {
            output_path.push(normalize_path_component(comp.as_os_str()));
        }

        // Normalize filename
        if let Some(stem) = relative.file_stem() {
            output_path.push(format!("{}.html", normalize_path_component(stem)));
        }

        if let Some(parent) = output_path.parent() {
            try_fs(parent, |p| fs::create_dir_all(p))?;
        }

        try_write(&output_path, &html)?;
        Ok(())
    }

    pub fn add_markdown(&mut self, md_path: &Path) -> Result<(), Box<dyn Error>> {
        self.rebuild_markdown(md_path)?;
        self.regenerate_sidebar()?;
        Ok(())
    }

    pub fn remove_page(&mut self, md_path: &Path) -> Result<(), Box<dyn Error>> {
        let relative = md_path.strip_prefix(&self.config.content_dir)?;
        let mut output_path = PathBuf::from(&self.config.build_dir);

        // Normalize folder components
        let comps: Vec<_> = relative.components().collect();
        for comp in &comps[..comps.len() - 1] {
            output_path.push(normalize_path_component(comp.as_os_str()));
        }

        // Normalize filename
        if let Some(stem) = relative.file_stem() {
            output_path.push(format!("{}.html", normalize_path_component(stem)));
        }

        if output_path.exists() {
            #[cfg(windows)]
            {
                let mut perms = fs::metadata(&output_path)?.permissions();
                perms.set_readonly(false);
                fs::set_permissions(&output_path, perms)?;
            }
            try_fs(&output_path, |p| fs::remove_file(p))?;
        }

        self.regenerate_sidebar()?;
        Ok(())
    }

    fn regenerate_sidebar(&mut self) -> Result<(), Box<dyn Error>> {
        expand_sidebar(&mut self.config.sidebar, &self.config.content_dir)?;

        // Validate paths with normalized underscores
        validate_sidebar_normalized(&self.config.sidebar, &self.config.content_dir)?;

        let html = generate_sidebar_html(&self.config.sidebar);
        try_write(&Path::new(&self.config.build_dir).join("sidebar.html"), &html)?;
        Ok(())
    }

    fn write_assets(&mut self) -> Result<(), Box<dyn Error>> {
        let styles = Path::new(&self.config.build_dir).join("styles");

        // Favicon
        if let Some(path) = &self.config.favicon {
            let data = fs::read(path).map_err(|e| format!("Failed to read '{}': {}", path, e))?;
            try_write(&Path::new(&self.config.build_dir).join("favicon.ico"), &String::from_utf8_lossy(&data))?;
        } else {
            defaults::write_favicon(&self.config.build_dir)?;
        }

        // CSS & JS assets
        for (src, dest) in [
            (&self.config.custom_css, styles.join("main.css")),
            (&self.config.sidebar_css, styles.join("sidebar.css")),
            (&self.config.highlight_css, styles.join("highlight.css")),
            (&self.config.custom_js, Path::new(&self.config.build_dir).join("reload.js")),
        ] {
            if let Some(path) = src {
                let content = try_read_string(Path::new(path))?;
                let processed_content = if self.production {
                    if dest.extension().map_or(false, |ext| ext == "css") { minify_css(&content) }
                    else if dest.extension().map_or(false, |ext| ext == "js") { minify_js(&content) }
                    else { content }
                } else {
                    if dest.extension().map_or(false, |ext| ext == "css") { format_css(&content) }
                    else if dest.extension().map_or(false, |ext| ext == "js") { format_js(&content) }
                    else { content }
                };
                try_write(&dest, &processed_content)?;
            }
        }

        if self.config.custom_js.is_none() { defaults::write_dev_script(&self.config.build_dir)?; }
        if self.config.custom_css.is_none() { defaults::write_css(&styles)?; }
        if self.config.sidebar_css.is_none() { defaults::write_sidebar_css(&styles)?; }
        if self.config.sidebar_css.is_none() { defaults::write_highlight_css(&styles)?; }

        Ok(())
    }

    pub fn rebuild_custom(&mut self, target: &str) -> Result<(), Box<dyn Error>> {
        let opt_path = match target {
            "css" => self.config.custom_css.as_ref(),
            "js" => self.config.custom_js.as_ref(),
            _ => None,
        };

        if let Some(path) = opt_path {
            let content = try_read_string(Path::new(path))?;
            let processed_content = if self.production {
                match target {
                    "css" => minify_css(&content),
                    "js" => minify_js(&content),
                    _ => content,
                }
            } else {
                match target {
                    "css" => format_css(&content),
                    "js" => format_js(&content),
                    _ => content,
                }
            };

            let dest = match target {
                "css" => Path::new(&self.config.build_dir).join("styles/main.css"),
                "js" => Path::new(&self.config.build_dir).join("reload.js"),
                _ => Path::new("dummy").to_path_buf(),
            };
            try_write(&dest, &processed_content)?;
        }

        Ok(())
    }
}

// Normalize folder/file names
fn normalize_path_component(component: &OsStr) -> String {
    component.to_string_lossy().replace(' ', "_")
}

// Sidebar validation that handles underscores
fn validate_sidebar_normalized(items: &Vec<SidebarItem>, content_dir: &str) -> Result<(), Box<dyn Error>> {
    for item in items {
        if let Some(slug) = &item.slug {
            let path = Path::new(content_dir)
                .join(slug.replace("/", std::path::MAIN_SEPARATOR.to_string().as_str()) + ".md");
            if !path.exists() {
                return Err(format!("Missing file for slug: {}", path.display()).into());
            }
        }

        if let Some(children) = &item.items {
            validate_sidebar_normalized(children, content_dir)?;
        }
    }
    Ok(())
}