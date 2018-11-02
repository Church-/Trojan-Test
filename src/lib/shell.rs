use std::iter::{Peek,take_while};

fn shell() -> Vec<String> {
	let v = Vec::new();
	let cmd_vec: Vec<&str> = "cat /etc/passwd | grep root | cut -d 1".split("|").collect();
	
}