use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{SystemTime};
use std::sync::{Arc, Mutex};
type Clients = Arc<Mutex<Vec<TcpStream>>>;
use crate::helpers::collect_files;
use crate::helpers::parse_path;
use ultibuilder::{Builder, Config};


pub fn run(config: Arc<Config>, host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let clients: Clients = Arc::new(Mutex::new(Vec::new()));
    let clients_watcher = clients.clone();
    let host_owned = host.to_string();
    let config_watcher = config.clone();

    thread::spawn(move || {
        watch_and_rebuild(&config_watcher, clients_watcher, &host_owned, port)
    });

    serve(clients, host, port)
}

fn serve(clients: Arc<Mutex<Vec<TcpStream>>>, host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let link = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&link)?;
    println!("Serving at http://{}:{}", host, port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = clients.clone();
                thread::spawn(move || {
                    if let Err(e) = handle_connection(stream, clients) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}


fn handle_connection(mut stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer);
    let first_line = request.lines().next().unwrap_or("");
    let path = parse_path(first_line); // your existing helper

    // SSE endpoint for live reload
    if path == "/reload" {
        let stream_clone = stream.try_clone()?;
        stream.write_all(
            b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\n\r\n"
        )?;
        clients.lock().unwrap().push(stream_clone);
        return Ok(());
    }

    // Map URL path to file in dist/
    let file_path = if path == "/" {
        "dist/index.html".to_string()
    } else {
        format!("dist{}", path)
    };

    if Path::new(&file_path).exists() {
        let mut contents_bytes = fs::read(&file_path)?;

        // Inject live-reload JS into HTML files (skip sidebar.html)
        if file_path.ends_with(".html") && !file_path.ends_with("sidebar.html") {
            let mut contents = String::from_utf8(contents_bytes)?;
            let snippet = r#"
<script>
  const evtSource = new EventSource('/reload');
  evtSource.onmessage = () => { location.reload(); };
</script>
"#;
            if let Some(pos) = contents.rfind("</body>") {
                contents.insert_str(pos, snippet);
            } else {
                contents.push_str(snippet);
            }
            contents_bytes = contents.into_bytes();
        }

        // Simple content type detection
        let content_type = if file_path.ends_with(".html") {
            "text/html"
        } else if file_path.ends_with(".css") {
            "text/css"
        } else if file_path.ends_with(".js") {
            "application/javascript"
        } else if file_path.ends_with(".ico") {
            "image/x-icon"
        } else if file_path.ends_with(".png") {
            "image/png"
        } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
            "image/jpeg"
        } else if file_path.ends_with(".svg") {
            "image/svg+xml"
        } else {
            "application/octet-stream"
        };

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            content_type,
            contents_bytes.len()
        );
        stream.write_all(response.as_bytes())?;
        stream.write_all(&contents_bytes)?;
    } else {
        let body = "404 Not Found";
        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes())?;
    }

    Ok(())
}

fn watch_and_rebuild(config: &Config, clients: Clients, host: &str, port: u16) {
    use std::collections::HashMap;

    // Initial snapshot of relevant paths
    let mut last_state: HashMap<PathBuf, SystemTime> = {
        let mut map = HashMap::new();

        if !config.content_dir.is_empty() {
            for (path, time) in snapshot(Path::new(&config.content_dir)) {
                map.insert(path, time);
            }
        }

        for opt_file in [
            &config.favicon,
            &config.custom_css,
            &config.sidebar_css,
            &config.highlight_css
        ] {
            if let Some(file) = opt_file {
                if let Ok(metadata) = std::fs::metadata(file) {
                    if let Ok(modified) = metadata.modified() {
                        map.insert(PathBuf::from(file), modified);
                    }
                }
            }
        }

        map
    };

    let mut builder = match Builder::build_fresh(&config, false) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Initial build failed: {}", e);
            return;
        }
    };

    // ✅ Print broadcast info after initial build
    println!("Broadcasting live-reload at http://{}:{}/", host, port);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut current_state: HashMap<PathBuf, SystemTime> = HashMap::new();

        if !config.content_dir.is_empty() {
            for (path, time) in snapshot(Path::new(&config.content_dir)) {
                current_state.insert(path, time);
            }
        }

        for opt_file in [
            &config.favicon,
            &config.custom_css,
            &config.sidebar_css,
            &config.highlight_css,
        ] {
            if let Some(file) = opt_file {
                if let Ok(metadata) = std::fs::metadata(file) {
                    if let Ok(modified) = metadata.modified() {
                        current_state.insert(PathBuf::from(file), modified);
                    }
                }
            }
        }

        let mut changed = false;

        // Added files
        for (path, _) in &current_state {
            if !last_state.contains_key(path) {
                handle_change(&mut builder, path, "added");
                changed = true;
            }
        }

        // Modified files
        for (path, &modified) in &current_state {
            if let Some(&old_modified) = last_state.get(path) {
                if old_modified != modified {
                    handle_change(&mut builder, path, "modified");
                    changed = true;
                }
            }
        }

        // Deleted files
        for path in last_state.keys() {
            if !current_state.contains_key(path) {
                handle_change(&mut builder, path, "deleted");
                changed = true;
            }
        }

        if changed {
            let mut lock = clients.lock().unwrap();
            lock.retain(|mut client| {
                let msg = b"data: reload\n\n";
                client.write_all(msg).is_ok()
            });
        }

        last_state = current_state;
    }
}

// Handles additions, modifications, deletions
fn handle_change(builder: &mut Builder, path: &std::path::Path, action: &str) {
    match path.extension().and_then(|e| e.to_str()) {
        Some("md") => {
            if action == "added" {
                println!("Markdown added: {}", path.display());
                let _ = builder.add_markdown(path);
            } else if action == "modified" {
                println!("Markdown modified: {}", path.display());
                let _ = builder.rebuild_markdown(path);
            } else if action == "deleted" {
                println!("Markdown deleted: {}", path.display());
                let _ = builder.remove_page(path);
            }
        }
        Some("css") => {
            println!("CSS {}: {}", action, path.display());
            let _ = builder.rebuild_custom("css");
        }
        Some("js") => {
            println!("JS {}: {}", action, path.display());
            let _ = builder.rebuild_custom("js");
        }
        _ => {}
    }
}

// Recursively collect files in content_dir
fn snapshot(dir: &std::path::Path) -> Vec<(std::path::PathBuf, std::time::SystemTime)> {
    let mut files = Vec::new();
    collect_files(dir, &mut files);
    files
}
