#[allow(unused_imports)]
use std::net::TcpListener;
use std::{
    io::{Read, Write},
    net::TcpStream,
};


fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];  
    let bytes_read =stream.read(&mut buffer).unwrap();

    let request_data = &buffer[..bytes_read];

    let parts: Vec<&[u8]> = request_data.split(|&c| c==b' ').collect();

    let response = match parts[1]{
        b"/" => "HTTP/1.1 200 OK \r\n\r\n",
        _ =>"HTTP/1.1 404 Not Found \r\n\r\n",
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}



fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
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

