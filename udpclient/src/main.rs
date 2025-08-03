use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind socket");
    let msg = b"Hello, UDP server!";
    socket
        .send_to(msg, "127.0.0.1:34254")
        .expect("Failed to send data");
}
