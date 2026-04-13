use std::collections::HashMap;


pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub fn get_complete_request_len(buffer: &[u8]) -> Option<usize> {
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


pub fn find_header_end(bytes: &[u8]) -> Option<usize> {
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



pub fn parse_request(request: &[u8]) -> Option<Request> {
    let header_end = find_header_end(request)?;

    let headers_bytes = &request[..header_end];
    let body_bytes = &request[header_end + 4..];

    let headers_str = std::str::from_utf8(headers_bytes).ok()?;
    let lines: Vec<&str> = headers_str.split("\r\n").collect();

    let request_line = lines.first()?;
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() != 3 {
        return None;
    }

    let method = parts[0].to_string();
    let path = parts[1].to_string();
    let version = parts[2].to_string();

    let mut headers_map = HashMap::new();

    for line in lines.iter().skip(1) {
        if let Some((name, value)) = line.split_once(": ") {
            headers_map.insert(name.to_ascii_lowercase(), value.to_string());
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
