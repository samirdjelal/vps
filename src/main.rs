use std::{env, process};

mod command;

fn main() {
	requirements();
	
	let mut args = env::args();
	if args.len() > 1 {
		if let Some(cmd) = args.nth(1) {
			let cmd: &str = cmd.as_str();
			match cmd {
				"swap" => command::swap(),
				"help" => command::help(),
				_ => {}
			}
			process::exit(0);
		}
	}
	command::help();
}


fn requirements() {
	if std::env::consts::OS != "linux" {
		eprintln!("Only ubuntu are supported for now.");
		// process::exit(1);
	}
}