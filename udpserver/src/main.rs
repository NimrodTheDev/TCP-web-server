use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("Could not bind socket");
    let mut buf = [0; 1024];
    loop {
        let (len, src) = socket.recv_from(&mut buf).expect("Failed to receive data");
        println!("Received {} bytes from {}: {:?}", len, src, &buf[..len]);
    }
}
