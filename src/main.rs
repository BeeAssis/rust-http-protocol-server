#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    let request_data = &buffer[..bytes_read];
    let parts: Vec<&[u8]> = request_data.split(|&c| c == b' ').collect();

    let path = parts[1];

    let response = if path == b"/" {
        "HTTP/1.1 200 OK\r\n\r\n".to_string()
    } else if path.starts_with(b"/echo/") {
        let body = &path[6..];
        let content_length = body.len();

        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            content_length,
            std::str::from_utf8(body).unwrap(),
        )
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}