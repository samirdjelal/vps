use std::{env, process};

mod command;
mod utils;

fn main() {
	utils::requirements();
	
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
