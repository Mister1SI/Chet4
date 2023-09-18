use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub fn start(addr: &String) {
    println!("{}\nServer!", addr);
	let listener = TcpListener::bind(addr.as_str())
		.expect("Error creating the listening server.");
	
	for stream in listener.incoming() {
		match stream {
			Ok(mut s) => {
				thread::spawn(move || {
    				if let Err(e) = handle_connection(s) {
        				eprintln!("Error when handling client connection:\n{}", e);
    				}
				});
			}
			Err(e) => {
				eprintln!("Error when handling client connection:\n{}", e);
			}
		}
	}

	
}

fn handle_connection(mut s: TcpStream) -> Result<(), std::io::Error> {
	let mut buffer = [0; 1024];
	let bytes_read = s.read(&mut buffer)?;
	let msg = String::from_utf8(buffer[..bytes_read]
								.to_vec()).unwrap_or(String::from(""));
	
	println!("{}", msg);
	Ok(())
}