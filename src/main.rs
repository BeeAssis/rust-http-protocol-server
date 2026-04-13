use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;

struct Request {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

fn get_complete_request_len(buffer: &[u8]) -> Option<usize> {
    let header_end = find_header_end(buffer)?;

    let headers_bytes = &buffer[..header_end];
    let body_bytes = &buffer[header_end + 4..];

    let headers_str = std::str::from_utf8(headers_bytes).ok()?;
    let lines: Vec<&str> = headers_str.split("\r\n").collect();

    let mut content_length = 0usize;

    for line in lines.iter().skip(1) {
        if let Some((name, value)) = line.split_once(": ") {
            if name.eq_ignore_ascii_case("Content-Length") {
                content_length = value.parse::<usize>().ok()?;
            }
        }
    }

    if body_bytes.len() < content_length {
        return None;
    }

    Some(header_end + 4 + content_length)
}
fn find_header_end(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < 4 {
        return None;
    }

    for i in 0..bytes.len() - 3 {
        if bytes[i] == b'\r'
            && bytes[i + 1] == b'\n'
            && bytes[i + 2] == b'\r'
            && bytes[i + 3] == b'\n'
        {
            return Some(i);
        }
    }

    None
}

fn parse_request(request: &[u8]) -> Option<Request> {

    let header_end = find_header_end(request)?;

    
    let headers_bytes = &request[..header_end];
    let body_bytes = &request[header_end + 4..];


    let headers_str = std::str::from_utf8(headers_bytes).ok()?;
    let lines: Vec<&str> = headers_str.split("\r\n").collect();


    let request_line = lines.first()?;
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() !=3{
       return None;
    }

    let method = parts[0].to_string();
    let path = parts[1].to_string();
    let version = parts[2].to_string();

    let mut headers_map = HashMap::new();

    for line in lines.iter().skip(1) {
        if let Some((name,value)) = line.split_once(": "){
            headers_map.insert(name.to_ascii_lowercase(),value.to_string());
        }
    }

     Some(Request {
        method,
        path,
        version,
        headers: headers_map,
        body: body_bytes.to_vec(),
    })




}

fn text_response(body: &str) -> Vec<u8> {
    let headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n",
        body.len()
    );

    let mut response = headers.into_bytes();
    response.extend_from_slice(body.as_bytes());
    response
}

fn create_file_response(path:&str, body:&[u8]) ->Vec<u8>{
    match fs::write(path, body){
        Ok(_) => b"HTTP/1.1 201 Created\r\n\r\n".to_vec(),
        Err(_) => b"HTTP/1.1 500 Internal Server Error\r\n\r\n".to_vec(),
    }

}

fn file_response(path: &str) -> Vec<u8> {
    match fs::read(path) {
        Ok(bytes) => {
            let headers = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                bytes.len()
            );
            let mut response = headers.into_bytes();
            response.extend(bytes);
            response
        }
        Err(_) => b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec(),
    }
}



fn handle_connection(mut stream: TcpStream, directory: Option<String>) {
    const BAD_REQUEST: &[u8] = b"HTTP/1.1 400 Bad Request\r\n\r\n";
    let mut incoming_bytes: Vec<u8> = Vec::new();

    loop {
        let mut temp_buffer = [0; 1024];

        let num_bytes_read = match stream.read(&mut temp_buffer) {
            Ok(0) => return,
            Ok(n) => n,
            Err(_) => return,
        };

        incoming_bytes.extend_from_slice(&temp_buffer[..num_bytes_read]);

        loop {
            let request_len = match get_complete_request_len(&incoming_bytes) {
                Some(len) => len,
                None => break,
            };

            let request_data = &incoming_bytes[..request_len];

            let request = match parse_request(request_data) {
                Some(req) => req,
                None => {
                    let _ = stream.write_all(BAD_REQUEST);
                    let _ = stream.flush();
                    return;
                }
            };

            let should_close = request
                .headers
                .get("connection")
                .is_some_and(|value| value.eq_ignore_ascii_case("close"));

            let response = if request.path == "/" {
                b"HTTP/1.1 200 OK\r\n\r\n".to_vec()
            } else if request.path.starts_with("/echo/") {
                let body = &request.path[6..];
                text_response(body)
            } else if request.path == "/user-agent" {
                if let Some(user_agent) = request.headers.get("user-agent") {
                    text_response(user_agent)
                } else {
                    b"HTTP/1.1 400 Bad Request\r\n\r\n".to_vec()
                }
            } else if let Some(filename) = request.path.strip_prefix("/files/") {
                if let Some(dir) = directory.as_ref() {
                    let file_path = format!("{}/{}", dir.trim_end_matches('/'), filename);

                    if request.method == "GET" {
                        file_response(&file_path)
                    } else if request.method == "POST" {
                        create_file_response(&file_path, &request.body)
                    } else {
                        b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec()
                    }
                } else {
                    b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec()
                }
            } else {
                b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec()
            };

            if stream.write_all(&response).is_err() {
                return;
            }

            if stream.flush().is_err() {
                return;
            }

            incoming_bytes.drain(..request_len);

            if should_close {
                return;
            }
        }
    }
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    let args: Vec<String> = std::env::args().collect();
    let directory = if args.len() > 2 && args[1] == "--directory" {
        Some(args[2].clone())
    } else {
        None
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory = directory.clone();
                spawn(move || handle_connection(stream, directory));
            }
            Err(e) => println!("error: {}", e),
        }
    }
}