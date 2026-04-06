use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{spawn, sleep};
use std::time::Duration;
struct Request {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
}

fn parse_request(request_str: &str) -> Option<Request> {
    let lines: Vec<&str> = request_str.split("\r\n").collect();

    let request_line = lines.first()?;
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() != 3 {
        return None;
    }

    let method = parts[0].to_string();
    let path = parts[1].to_string();
    let version = parts[2].to_string();

    let mut headers = HashMap::new();

    for line in lines.iter().skip(1) {
        if line.is_empty() {
            break;
        }

        if let Some((name, value)) = line.split_once(": ") {
            headers.insert(name.to_ascii_lowercase(), value.to_string());
        }
    }

    Some(Request {
        method,
        path,
        version,
        headers,
    })
}

fn text_response(body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    )
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    let request_data = &buffer[..bytes_read];
    let request_str = std::str::from_utf8(request_data).unwrap();

    let request = match parse_request(request_str) {
        Some(req) => req,
        None => {
            let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }
    };

    let response = if request.path == "/" {
        "HTTP/1.1 200 OK\r\n\r\n".to_string()
    } else if request.path.starts_with("/echo/") {
        let body = &request.path[6..];
        text_response(body)
    } else if request.path == "/user-agent" {
        if let Some(user_agent) = request.headers.get("user-agent") {
            text_response(user_agent)
        } else {
            "HTTP/1.1 400 Bad Request\r\n\r\n".to_string()
        }
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
            Ok(stream) =>{
                spawn(move||handle_connection(stream));

            },
            Err(e) => println!("error: {}", e),
        }
    }
}