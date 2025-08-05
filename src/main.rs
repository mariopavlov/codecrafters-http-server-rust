use std::io::{BufRead, BufReader};
use std::net::TcpListener;
use std::{io::Write, net::TcpStream};

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("[DEBUG] Request: {http_request:#?}");

    let req_line = http_request.first().unwrap();

    println!("accepted new connection");

    if req_line == "GET / HTTP/1.1" {
        let status = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(status.as_bytes()).unwrap();
    } else {
        let status = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(status.as_bytes()).unwrap();
    }
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
