use std::net::{TcpListener, TcpStream};

pub fn start(addr: &String) {
    println!("{addr}\nServer!");
	let listener = match TcpListener::bind(addr) {
    	Ok(a) => a,
		Err(e) => {
			eprintln!("Error creating the listening server: {e}");
			e
		}
	};
	for stream in listener.incoming() {
		match stream {
			Ok(s) => {
				handle_connection(s);
			}
			Err(e) => {
				
			}
		}
	}

	
}

fn handle_connection(s: TcpStream) {
	
}