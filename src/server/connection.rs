use std::io::{Read, Write};
use std::net::TcpStream;

use crate::http::request::{get_complete_request_len, parse_request};
use crate::http::encoding::{select_encoding, Encoding};
use crate::http::response::{
    add_connection_close_header,
    create_file_response,
    file_response,
   encoded_text_response,
    text_response,
};



pub fn handle_connection(mut stream: TcpStream, directory: Option<String>) {
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
                .is_some_and(|value: &String| value.eq_ignore_ascii_case("close"));

            let mut response = if request.path == "/" {
                b"HTTP/1.1 200 OK\r\n\r\n".to_vec()
            } else if request.path.starts_with("/echo/") {
                let body = &request.path[6..];
                if let Some(Encoding::Gzip) = select_encoding(&request){
                    encoded_text_response(body,Encoding::Gzip)

                }else{
                    text_response(body)

                }
           
            } else if request.path == "/user-agent" {
                if let Some(user_agent) = request.headers.get("user-agent") {
                    text_response(user_agent)
                } else {
                    b"HTTP/1.1 400 Bad Request\r\n\r\n".to_vec()
                }
            }else if let Some(filename) = request.path.strip_prefix("/files/") {
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

            if should_close {
                response = add_connection_close_header(response);
            }

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