use std::net::TcpListener;

fn main() {
    let socket = TcpListener::bind("127.0.0.1:9001").unwrap();

    for stream in socket.incoming() {
        let _stream = stream.unwrap();

        println!("Connection recieved");
    }
}
