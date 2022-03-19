use dialoguer::{theme::ColorfulTheme, Select, Confirm};
use execute::{Execute, command};

pub fn swap() {
	let memory_size = [1024, 2048, 3072, 4096, 5120, 6144];
	let memory = Select::with_theme(&ColorfulTheme::default())
		.with_prompt("Swap memory size")
		.default(0)
		.items(&["1GB", "2GB", "3GB", "4GB", "5GB", "6GB"])
		.interact()
		.unwrap();
	let memory = memory_size[memory];
	
	let mount = Select::with_theme(&ColorfulTheme::default())
		.with_prompt("Mount a swap partition?")
		.default(0)
		.items(&["false", "true"])
		.interact()
		.unwrap();
	let mount = mount == 1;
	
	let confirm = Confirm::with_theme(&ColorfulTheme::default())
		.with_prompt("Do you really want to continue?")
		.default(true)
		.show_default(false)
		.wait_for_newline(true)
		.interact()
		.unwrap();
	
	if confirm {
		println!("Creating swapfile of {:?} MB!", memory);
		let _output = command("swapoff -a").execute();
		let _output = command(format!("dd if=/dev/zero of=/swapfile bs=1M count={}", memory)).execute();
		let _output = command("chmod 600 /swapfile").execute();
		let _output = command("mkswap /swapfile").execute();
		let _output = command("swapon /swapfile").execute();
		let _output = command("grep SwapTotal /proc/meminfo").execute();
		if mount {
			println!("Mounting swapfile using fstab!");
			//todo: check if mount already exists
			let _output = command(r#"echo "/swapfile none swap defaults 0 0" >> /etc/fstab"#).execute();
		}
	} else {
		println!("Nevermind then :(");
	}
}