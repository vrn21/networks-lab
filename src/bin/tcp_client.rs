use std::net::TcpStream;
use std::io::Write;

fn main(){
	let mut stream = TcpStream::connect("localhost:8080");
	let message = "Hello server";
	stream.unwrap().write_all(message.as_bytes());
	println!("Message sent: {}",message);
}
