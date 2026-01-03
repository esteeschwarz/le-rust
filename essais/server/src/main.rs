use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;

mod endpoint002;          // declares module, loads src/endpoint002.rs
fn handle_client_dep(mut stream: TcpStream, root_dir: PathBuf) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    
    if reader.read_line(&mut request_line).is_ok() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        
        let response = if parts.len() >= 2 && parts[0] == "GET" {
            let path = parts[1].to_string();
            
            // Handle /query endpoint
            if path == "/query" {
                endpoint002::handle_query_endpoint(&root_dir)
            } else {
                // Original file serving logic
                let mut file_path = path.clone();
                if file_path == "/" {
                    file_path = "/index.html".to_string();
                }
                
                let full_path = root_dir.join(file_path.trim_start_matches('/'));
                
                if !full_path.starts_with(&root_dir) {
                    endpoint002::create_error_response(403, "Forbidden")
                } else if full_path.is_file() {
                    match fs::read(&full_path) {
                        Ok(contents) => {
                            let content_type = endpoint002::get_content_type(&full_path);
                            format!(
                                "HTTP/1.1 200 OK\r\n\
                                Content-Type: {}\r\n\
                                Content-Length: {}\r\n\
                                Connection: close\r\n\
                                \r\n",
                                content_type,
                                contents.len()
                            ) + &String::from_utf8_lossy(&contents)
                        }
                        Err(_) => endpoint002::create_error_response(500, "Internal Server Error")
                    }
                } else {
                    endpoint002::create_error_response(404, "Not Found")
                }
            }
        } else {
            endpoint002::create_error_response(400, "Bad Request")
        };
        
        let _ = stream.write_all(response.as_bytes());
    }
}

fn handle_query_endpoint_dep(root_dir: &PathBuf) -> String {
    // Read the template file (content.html)
    let template_path = root_dir.join("content.html");
    let template = match fs::read_to_string(&template_path) {
        Ok(content) => content,
        Err(_) => {
            return endpoint002::create_error_response(404, "content.html not found");
        }
    };
    
    // Read files from www/docs directory
    let docs_dir = root_dir.join("docs");
    let file_list = match fs::read_dir(&docs_dir) {
        Ok(entries) => {
            let mut items = String::new();
            items.push_str("<ul>\n");
            
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    
                    // Only include HTML files
                    if file_name_str.ends_with(".html") {
                        items.push_str(&format!(
                            "  <li><a href=\"/docs/{}\">{}</a></li>\n",
                            file_name_str, file_name_str
                        ));
                    }
                }
            }
            
            items.push_str("</ul>");
            items
        }
        Err(_) => "<p>Error reading docs directory</p>".to_string(),
    };
    
    // Replace the placeholder in the template
    let final_html = template.replace(
        r#"<div id="content"></div>"#,
        &format!(r#"<div id="content">{}</div>"#, file_list)
    );
    
    format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/html; charset=UTF-8\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\
        \r\n\
        {}",
        final_html.len(),
        final_html
    )
}

fn create_error_response_dep(code: u16, message: &str) -> String {
    let body = format!(
        "<html><body><h1>{} {}</h1></body></html>",
        code, message
    );
    format!(
        "HTTP/1.1 {} {}\r\n\
        Content-Type: text/html; charset=UTF-8\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\
        \r\n\
        {}",
        code, message, body.len(), body
    )
}

fn get_content_type_dep(path: &Path) -> &str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("html") | Some("htm") => "text/html; charset=UTF-8",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("pdf") => "application/pdf",
        Some("txt") => "text/plain",
        _ => "application/octet-stream",
    }
}

fn main() {
    let port: &str = "1025";
    let root_dir = PathBuf::from("./www");
    
    if !root_dir.exists() {
        fs::create_dir(&root_dir).expect("Failed to create www directory");
        println!("Created www directory. Place your HTML files there.");
    }
    
    let docs_dir = root_dir.join("docs");
    if !docs_dir.exists() {
        fs::create_dir(&docs_dir).expect("Failed to create docs directory");
        println!("Created www/docs directory.");
    }
    
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .unwrap_or_else(|err| panic!("Failed to bind port {}: {}", port, err));
    
    println!("Server running on http://0.0.0.0:{}", port);
    println!("Serving files from: {:?}", root_dir.canonicalize().unwrap());
    println!("Visit http://localhost:{}/query to see the docs list", port);
    
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let root = root_dir.clone();
            thread::spawn(move || endpoint002::handle_client(stream, root));
        }
    }
}
