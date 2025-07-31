use std::net::TcpListener;
use std::{io::Write, net::TcpStream};

fn handle_client(mut stream: TcpStream) {
    println!("accepted new connection");
    let status = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(status.as_bytes()).unwrap();
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {e}");
            }
        }
    }
}
