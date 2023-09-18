use std::env;
use std::io;

mod client;
mod server;


fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		let mode = args[1].as_str();
		match mode {
			"-h" => help(),
			"-c" => {
				let data = get_data(false);
				client::start(&data[0], &data[1]);
			}
			"-s" => {
				let data = get_data(true);
				server::start(&data[0]);
			}
			_ => {
				println!("Unknown mode {}.", mode);
				help();
			}
		}
	} else {help();}
}

fn get_data(server_mode: bool) -> Vec<String> {
	let mut addr = String::new();
	let mut uname = String::new();
	println!("Enter the address & port (address:port):");
	io::stdin().read_line(&mut addr).unwrap();
	if !server_mode {
		println!("Enter a username:");
		io::stdin().read_line(&mut uname).unwrap();
		return vec![addr, uname];
	} else {
		return vec![addr];
	}
}




fn help() {
	println!(r#"
			 ==========
			 CHET 4
			 ==========

			 HELP MENU
			 ---------------
			 Usage: chet4 <mode> [options]
			 
			 Modes:
			 -h: Display the help menu.
			 -c: Run in client mode.
			 -s: Run in server mode.
 "#)
}