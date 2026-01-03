use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;

fn handle_client(mut stream: TcpStream, root_dir: PathBuf) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    
    if reader.read_line(&mut request_line).is_ok() {
        // Parse the request line: "GET /path/file.html HTTP/1.1"
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        
        let response = if parts.len() >= 2 && parts[0] == "GET" {
            let mut path = parts[1].to_string();
            
            // If requesting root, serve index.html
            if path == "/" {
                path = "/index.html".to_string();
            }
            
            // Build the file path (strip leading slash)
            let file_path = root_dir.join(path.trim_start_matches('/'));
            
            // Security: prevent directory traversal attacks
            if !file_path.starts_with(&root_dir) {
                create_error_response(403, "Forbidden")
            } else if file_path.is_file() {
                // Serve the file
                match fs::read(&file_path) {
                    Ok(contents) => {
                        let content_type = get_content_type(&file_path);
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
                    Err(_) => create_error_response(500, "Internal Server Error")
                }
            } else {
                create_error_response(404, "Not Found")
            }
        } else {
            create_error_response(400, "Bad Request")
        };
        
        let _ = stream.write_all(response.as_bytes());
    }
}

fn create_error_response(code: u16, message: &str) -> String {
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

fn get_content_type(path: &Path) -> &str {
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
    let root_dir = PathBuf::from("./www"); // Your document root directory
    
    // Create www directory if it doesn't exist
    if !root_dir.exists() {
        fs::create_dir(&root_dir).expect("Failed to create www directory");
        println!("Created www directory. Place your HTML files there.");
    }
    
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .unwrap_or_else(|err| panic!("Failed to bind port {}: {}", port, err));
    
    println!("Server running on http://0.0.0.0:{}", port);
    println!("Serving files from: {:?}", root_dir.canonicalize().unwrap());
    
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let root = root_dir.clone();
            thread::spawn(move || handle_client(stream, root));
        }
    }
}
