use std::io::{self, Write};
use std::fs;
use std::path::Path;
use crate::new::{Dir};
pub fn prompt(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Print prompt without newline
    print!("{prompt}: ");
    io::stdout().flush()?;

    // Store input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Return trimmed input
    Ok(input.trim().to_string())
}



pub fn create_dir_recursive(dir: &Dir, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(path)?;

    // write files
    for file in dir.files {
        let file_path = path.join(file.name);
        fs::write(file_path, file.contents)?;
    }

    // recurse directories
    for sub in dir.dirs {
        let sub_path = path.join(sub.name);
        create_dir_recursive(sub, &sub_path)?;
    }

    Ok(())
}