use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};

fn main() {
    let socket = TcpListener::bind("127.0.0.1:9001").unwrap();

    for stream in socket.incoming() {
        let stream = stream.unwrap();

        println!("Connection recieved");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buffer = BufReader::new(&stream);
    let http_request: Vec<_> = buffer
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();

    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("first.html").unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
