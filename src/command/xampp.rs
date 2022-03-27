use dialoguer::{theme::ColorfulTheme, Select};
use execute::{Execute, command};

pub fn xampp() {
	let commands = ["start", "restart", "stop", "install", "uninstall"];
	let index = Select::with_theme(&ColorfulTheme::default())
		.with_prompt("XAMPP Server")
		.default(0)
		.items(&commands)
		.interact()
		.unwrap();
	
	let xampp = Xampp::new("/opt/lampp/xampp".to_string(), false);
	
	match commands[index] {
		"start" => xampp.start(),
		"restart" => xampp.restart(),
		"stop" => xampp.stop(),
		"install" => xampp.install(),
		"uninstall" => xampp.uninstall(),
		_ => {}
	}
}

struct Xampp {
	path: String,
	status: bool,
}

impl Xampp {
	fn new(path: String, status: bool) -> Xampp {
		//todo: check if xampp is installed and it's status
		Xampp {
			path,
			status,
		}
	}
	fn start(&self) {
		if self.status {
			println!("XAMPP is already running");
		} else {
			let cmd = format!("{} start", self.path);
			command(cmd).execute();
			println!("XAMPP is now running");
		}
	}
	fn restart(&self) {
		if self.status {
			let cmd = format!("{} restart", self.path);
			command(cmd).execute();
			println!("XAMPP is now running");
		} else {
			println!("XAMPP is not running");
		}
	}
	fn stop(&self) {
		if self.status {
			let cmd = format!("{} stop", self.path);
			command(cmd).execute();
			println!("XAMPP is now stopped");
		} else {
			println!("XAMPP is not running");
		}
	}
	fn install(&self) {
		// let command = format!("{} start", self.path);
		// command(command);
		// println!("XAMPP is now installed");
	}
	fn uninstall(&self) {
		// let command = format!("{} start", self.path);
		// command(command);
		// println!("XAMPP is now uninstalled");
	}
}
