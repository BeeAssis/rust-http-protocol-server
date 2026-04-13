use std::fs;
use crate::http::encoding::{Encoding, gzip_compress};

pub fn text_response(body: &str) -> Vec<u8> {
    let headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n",
        body.len()
    );

    let mut response = headers.into_bytes();
    response.extend_from_slice(body.as_bytes());
    response
}

pub fn create_file_response(path: &str, body: &[u8]) -> Vec<u8> {
    match fs::write(path, body) {
        Ok(_) => b"HTTP/1.1 201 Created\r\n\r\n".to_vec(),
        Err(_) => b"HTTP/1.1 500 Internal Server Error\r\n\r\n".to_vec(),
    }
}

pub fn file_response(path: &str) -> Vec<u8> {
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

pub fn add_connection_close_header(mut response: Vec<u8>) -> Vec<u8> {
    if let Some(pos) = response.windows(4).position(|w| w == b"\r\n\r\n") {
        let mut new_response = Vec::with_capacity(response.len() + "Connection: close\r\n".len());
        new_response.extend_from_slice(&response[..pos]);
        new_response.extend_from_slice(b"\r\nConnection: close");
        new_response.extend_from_slice(&response[pos..]);
        response = new_response;
    }

    response
}


pub fn encoded_text_response(body: &str, encoding: Encoding) -> Vec<u8> {
    
    let encoding_str = match encoding {
        Encoding::Gzip => "gzip",
        Encoding::Brotli => "br",
        Encoding::Deflate => "deflate",
        Encoding::Zstd => "zstd",
    };

    let compressed_body = gzip_compress(body.as_bytes());

  let headers = format!(
      "HTTP/1.1 200 OK\r\n\
  Content-Encoding: {}\r\n\
  Content-Type: text/plain\r\n\
  Content-Length: {}\r\n\
  \r\n",
        encoding_str,
        compressed_body.len()
   );

    let mut response = headers.into_bytes();
    response.extend_from_slice(&compressed_body);
    response
}
