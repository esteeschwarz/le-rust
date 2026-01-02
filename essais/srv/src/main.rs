use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).is_ok() {
        let response = if request_line.starts_with("GET / ") {
            "HTTP/1.1 200 OK\r\n\
             Content-Type: text/html; charset=UTF-8\r\n\
             Content-Length: 128\r\n\
             Connection: close\r\n\
             \r\n\
             <!DOCTYPE html>
             <html><head><title>Rust Test Server</title></head>
             <body><h1>Hello from Rust in UTM VM!</h1>
             <p>Port 80 is working.</p></body></html>\r\n"
        } else {
            "HTTP/1.1 404 Not Found\r\n\r\n"
        };
        let _ = stream.write_all(response.as_bytes());
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:89").expect("Failed to bind port 80");
    println!("Server running on http://0.0.0.0:89 (find IP with 'ip addr')");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(|| handle_client(stream));
        }
    }
}
