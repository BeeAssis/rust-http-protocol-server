use std::net::TcpListener;
use std::thread::spawn;
use crate::server::connection::handle_connection;

mod server;
mod http;



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