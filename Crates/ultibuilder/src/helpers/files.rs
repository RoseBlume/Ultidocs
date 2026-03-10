use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path};
use std::error::Error;
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



// pub fn load_css(path_opt: &Option<String>) -> Result<Option<String>, Box<dyn Error>> {
//     if let Some(path_str) = path_opt {
//         let path = Path::new(path_str);
//         if path.exists() {
//             return Ok(Some(try_read_string(path)?));
//         } else {
//             println!("Error: File does not exist {}", path.display());
//         }
//     }
//     Ok(None)
// }