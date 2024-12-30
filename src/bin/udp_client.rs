use std::net::UdpSocket;

fn main() {
	let socket = UdpSocket::bind("127.0.0.1:8081").unwrap();
	println!("Client socket bound to {:?}",socket.local_addr());
	let server_addr = "127.0.0.1:8080";
	let msg = "hello server";
	socket.send_to(msg.as_bytes(),server_addr).unwrap();
	println!("messsage sent to {} and {}",server_addr,msg);	
}
