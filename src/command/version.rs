const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn version() {
	println!("VPS Manager v{}", VERSION);
}