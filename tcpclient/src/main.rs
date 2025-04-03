use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").expect("Connection to localhost:3000");
    stream
        .write("Hello world".as_bytes())
        .expect("Bytes written successfully");

    let mut buffer = [0; 5];
    stream.read(&mut buffer).expect("buffer read successfully");

    println!(
        "Got message from server {}",
        String::from_utf8(buffer.to_vec()).expect("Convert buffer to string")
    );
}
