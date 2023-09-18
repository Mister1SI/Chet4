use std::net::TcpStream;
use std::io::{Read, Write};
use std::io;

pub fn start(addr: &String, uname: &String) -> std::io::Result<()> {
	println!("{addr}{uname}\nClient!");
	println!("Type \"exit()\" to quit the program\nEnter messages to send:");
	let mut stream = TcpStream::connect(addr.as_str())?;
	loop {
		let mut msg = String::new();
		io::stdin().read_line(&mut msg).unwrap_or(0);
		if msg == "exit()" {
			break;
		}
		match stream.write_all(&msg.as_bytes()) {
			Ok(a) => (),
			Err(e) => println!("Error sending message: {}", e),
		}
	}
	Ok(())
}