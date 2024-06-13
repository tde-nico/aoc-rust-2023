use std::io::{self, BufRead};

fn process_line(line: String) -> u32 {
	let b: Vec<_> = line.chars().collect();
	let mut left: u32;
	let mut right: u32;

	left = 0;
	right = 0;

	for i in 0..line.len() {
		if b[i] as u8 >= 0x30 && b[i] as u8 <= 0x39 {
			left = (b[i] as u8 - 0x30) as u32;
			break;
		}
	}

	for j in (0..line.len()).rev() {
		if b[j] as u8 >= 0x30 && b[j] as u8 <= 0x39 {
			right = (b[j] as u8 - 0x30) as u32;
			break;
		}
	}

	return 10 * left + right;
}

fn process_lines() {
	let mut sum: u32 = 0;

	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		sum += process_line(line.unwrap());
	}
	println!("{}", sum);
}

fn main() {
	process_lines();
}
