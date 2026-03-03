use std::path::{Path, PathBuf};
use std::fs;
pub fn collect_files(path: &std::path::Path, files: &mut Vec<(std::path::PathBuf, std::time::SystemTime)>) {
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, files);
            } else if let Ok(metadata) = std::fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    files.push((path, modified));
                }
            }
        }
    }
}
pub fn parse_path(request_line: &str) -> String {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() > 1 {
        parts[1].to_string()
    } else {
        "/".to_string()
    }
}

/// Recursively collect all files in a directory
pub fn collect_files_recursive(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_files_recursive(&path));
            } else {
                files.push(path);
            }
        }
    }
    files
}

