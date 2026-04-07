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

fn parse_request(request: &[u8]) -> Option<Request> {

    if request.len() < 4 {
        return None;
    }
    

    let mut header_end = None;

    for i in 0..request.len()-3{
        if request[i] == b'\r'
        && request[i + 1] == b'\n'
        && request[i + 2] == b'\r'
        && request[i + 3] == b'\n'
    {
        header_end = Some(i);
        break;
    }

 }
  let header_end = match header_end {
    Some(i) => i,
    None => return None,
    };


    let headers_bytes = &request[..header_end];
    let body_bytes = &request[header_end+4..];

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

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    let request_data = &buffer[..bytes_read];

    let request = match parse_request(request_data){
        Some(req) => req,
        None =>{
            stream.write_all(BAD_REQUEST).unwrap();
            stream.flush().unwrap();
            return;
        }
    };


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
            let file_path = format!("{}{}", dir, filename);

            if request.method == "GET"{
                file_response(&file_path)
            }else if request.method == "POST"{
                 create_file_response(&file_path,&request.body)
            }else{
                 b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec()
            }    
        } else {
            b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec()
        }
    } else {
        b"HTTP/1.1 404 Not Found\r\n\r\n".to_vec()
    };

    stream.write_all(&response).unwrap();
    stream.flush().unwrap();
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