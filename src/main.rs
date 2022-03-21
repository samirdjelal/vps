use std::{env, process};
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

mod command;
mod utils;

fn main() {
	utils::requirements();
	let commands = ["xampp", "swap", "version"];
	let index = Select::with_theme(&ColorfulTheme::default())
		.with_prompt("VPS Manager")
		.default(0)
		.items(&commands)
		.interact()
		.unwrap();
	
	match commands[index] {
		"xampp" => command::xampp(),
		"swap" => command::swap(),
		"version" => command::version(),
		_ => command::version(),
	}
}
