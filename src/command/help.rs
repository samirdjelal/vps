const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn help() {
	println!("VPS v{}", VERSION);
	println!(" ");
	println!("COMMANDS:");
	println!(" - swap  : create swap partition");
	println!(" - help  : show help menu");
	println!(" ");
}