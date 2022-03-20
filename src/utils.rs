#![allow(unused_imports)]
use std::fs::File;
use std::io::Read;
use std::process;

pub fn requirements() {
	if std::env::consts::OS != "linux" {
		eprintln!("Only ubuntu are supported for now.");
		process::exit(1);
	}
	#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
	if !nix::unistd::Uid::effective().is_root() {
		eprintln!("You must run this executable with root permissions");
		process::exit(1);
	}
}
