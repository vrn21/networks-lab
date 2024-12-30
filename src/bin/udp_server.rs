use std::net::UdpSocket;

fn main() {
	let socket = UdpSocket::bind("127.0.0.1:8080").unwrap();
	println!("server running on 8080");
	let mut buf = [0;512];
	loop{
		let (bytes_received,src) = socket.recv_from(&mut buf).unwrap();
		let msg = String::from_utf8_lossy(&buf [..bytes_received]);
		println!("received from {}: {}",src,msg);
	}

}
