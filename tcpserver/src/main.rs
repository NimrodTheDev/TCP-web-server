use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").expect("Server failed to start");
    println!("Connected to port 3000");
    for stream in listener.incoming() {
        let mut stream = stream.expect("Error in incoming error");
        println!("connection established");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer);
        stream.write(&buffer);
    }
}
