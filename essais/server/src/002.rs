use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;

fn handle_client(mut stream: TcpStream, root_dir: PathBuf) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();

    if reader.read_line(&mut request_line).is_ok() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        let response = if parts.len() >= 2 && parts[0] == "GET" {
            let path = parts[1];

            if path == "/query" {
                handle_query_endpoint(&root_dir)
            } else {
                serve_static(path, &root_dir)
            }
        } else {
            create_error_response(400, "Bad Request")
        };

        let _ = stream.write_all(response.as_bytes());
    }
}

fn handle_query_endpoint(root_dir: &PathBuf) -> String {
    let docs_dir = root_dir.join("docs");
    let mut items = Vec::new();

    if let Ok(entries) = fs::read_dir(&docs_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("html") {
                let rel_path = format!(
                    "/docs/{}",
                    entry.file_name().to_string_lossy()
                );

                // Read file to extract <title>
                let content = fs::read_to_string(&path).unwrap_or_default();
                let title = extract_title(&content)
                    .unwrap_or_else(|| entry.file_name().to_string_lossy().into_owned());

                items.push(format!(
                    r#"{{"path":"{}","title":"{}"}}"#,
                    rel_path, escape_json(&title)
                ));
            }
        }
    }

    let json = format!("[{}]", items.join(","));

    format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: application/json; charset=UTF-8\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        json.len(),
        json
    )
}

fn serve_static(path: &str, root_dir: &PathBuf) -> String {
    let mut file_path = path.to_string();
    if file_path == "/" {
        file_path = "/content.html".to_string(); // use content.html as main UI
    }

    let full_path = root_dir.join(file_path.trim_start_matches('/'));

    if !full_path.starts_with(root_dir) {
        return create_error_response(403, "Forbidden");
    }

    if full_path.is_file() {
        match fs::read(&full_path) {
            Ok(contents) => {
                let content_type = get_content_type(&full_path);
                let mut header = format!(
                    "HTTP/1.1 200 OK\r\n\
                     Content-Type: {}\r\n\
                     Content-Length: {}\r\n\
                     Connection: close\r\n\
                     \r\n",
                    content_type,
                    contents.len()
                );
                header.push_str(&String::from_utf8_lossy(&contents));
                header
            }
            Err(_) => create_error_response(500, "Internal Server Error"),
        }
    } else {
        create_error_response(404, "Not Found")
    }
}

fn extract_title(content: &str) -> Option<String> {
    let lower = content.to_lowercase();
    let start = lower.find("<title>")?;
    let end = lower.find("</title>")?;
    if end <= start {
        return None;
    }
    let start_idx = start + "<title>".len();
    Some(content[start_idx..end].trim().to_string())
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

// create_error_response, get_content_type, main() as before,
// just make sure main uses content.html as entry and keeps root_dir = "./www".
