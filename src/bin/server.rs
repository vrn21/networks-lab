use std::net::{TcpListener,TcpStream};
use std::io::Read;


fn handle_connection(stream: TcpStream){
	let mut stream = stream;
	println!("New connection:{:?}",stream.peer_addr());
	let mut buf = [0;512];
	let mut bytes_read = stream.read(&mut buf).unwrap();
	if bytes_read > 0{
		let msg = String::from_utf8_lossy(&buf[..bytes_read]);
		println!("Received:{}",msg);
	
	}
}

fn main(){
	let listener = TcpListener::bind("localhost:8080").unwrap();
	println!("server listening at 8080");
	for stream in listener.incoming(){
		match stream {
			Ok(stream) => {
				handle_connection(stream);
			}
			Err(_)=>{println!("Some error")}
		}	
	}
}
