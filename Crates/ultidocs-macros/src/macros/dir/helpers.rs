use std::fs;
use std::path::Path;

pub fn parse_input(input: &str) -> (String, String, Vec<String>) {
    
    // extremely simple parser
    // Example expected:
    // MyAssets, "assets", ignore = ["target","temp.txt"]

    let mut parts = input.split(',');

    let name = parts.next().unwrap().trim().to_string();
    let path = parts
        .next()
        .unwrap()
        .trim()
        .trim_matches('"')
        .to_string();

    let mut ignores = Vec::new();

    if let Some(ignore_part) = parts.next() {
        if ignore_part.contains("ignore") {
            if let Some(start) = ignore_part.find('[') {
                if let Some(end) = ignore_part.find(']') {
                    let list = &ignore_part[start + 1..end];
                    for item in list.split(',') {
                        ignores.push(item.trim().trim_matches('"').to_string());
                    }
                }
            }
        }
    }

    (name, path, ignores)
}

pub fn generate_runtime_types() -> String {
    r#"
pub struct File {
    pub name: &'static str,
    pub contents: &'static [u8],
}

pub struct Dir {
    pub name: &'static str,
    pub files: &'static [File],
    pub dirs: &'static [Dir],
}
"#
    .to_string()
}

pub fn generate_dir(root: &Path, dir: &Path, ignores: &[String]) -> String {
    let mut files = Vec::new();
    let mut dirs = Vec::new();

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if should_ignore(&path, ignores) {
            continue;
        }

        if path.is_dir() {
            dirs.push(generate_dir(root, &path, ignores));
        } else {
            let name = path.file_name().unwrap().to_str().unwrap();
            let full_path = dir.join(name);
            let path_str = full_path.to_string_lossy();

            files.push(format!(
                "File {{ name: \"{}\", contents: include_bytes!(r#\"{}\"#) }}",
                name,
                path_str
            ));
        }
    }

    let dir_name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("root");

    format!(
        "Dir {{
            name: \"{}\",
            files: &[{}],
            dirs: &[{}],
        }}",
        dir_name,
        files.join(","),
        dirs.join(",")
    )
}

fn should_ignore(path: &Path, ignores: &[String]) -> bool {
    if let Some(name) = path.file_name().and_then(|x| x.to_str()) {
        ignores.iter().any(|i| i == name)
    } else {
        false
    }
}