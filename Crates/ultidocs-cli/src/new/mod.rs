mod helpers;
use helpers::{
    prompt,
    create_dir_recursive
};
use std::fs;
use std::path::PathBuf;
use ultidocs_macros::include_dir;
include_dir!(
    Assets,
    "assets/new",
    ignore = ["ulticonfig.json"]
);

const CONFIG_TEMPLATE: &str = include_str!(
    concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/new/ulticonfig.json"
    )
);

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let dir = prompt("Project to be created relative to the current directory")?;

    let base = PathBuf::from(&dir);

    let title = prompt("Project title")?;

    fs::create_dir_all(&base)?;

    let config = CONFIG_TEMPLATE.replacen("{}", &title, 1);
    let file_path = base.join("ulticonfig.json");

    create_dir_recursive(&Assets::ROOT, &base)?;
    fs::write(file_path, config)?;

    println!("Project created successfully.");

    Ok(())
}
