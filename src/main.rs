use std::{
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

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
    // println!("Request recieved: {http_request:#?}");
}
