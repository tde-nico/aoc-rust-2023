use std::io::{self, BufRead};
use std::collections::HashMap;

fn memncmp<T: PartialEq>(s1: &[T], s2: &[T], n: usize) -> bool {
	s1.iter().zip(s2.iter()).take(n).all(|(c1, c2)| c1 == c2)
}

fn process_line(line: String) -> u32 {
	let b: Vec<_> = line.chars().collect();
	let mut left: u32;
	let mut right: u32;
	let mut flag: bool;

	let mut digits: HashMap<Vec<_>, u32> = HashMap::new();
	digits.insert("one".chars().collect(), 1);
    digits.insert("two".chars().collect(), 2);
    digits.insert("three".chars().collect(), 3);
    digits.insert("four".chars().collect(), 4);
    digits.insert("five".chars().collect(), 5);
    digits.insert("six".chars().collect(), 6);
    digits.insert("seven".chars().collect(), 7);
    digits.insert("eight".chars().collect(), 8);
    digits.insert("nine".chars().collect(), 9);

	left = 0;
	right = 0;

	for i in 0..line.len() {
		flag = false;
		for (digit, value) in &digits {
			let len = std::cmp::min(line.len() - 1, i + digit.len());
			if memncmp(&b[i..], &digit, len) {
				flag = true;
				left = *value;
				break;
			}
		}
		
		if flag {
			break ;
		}
		
		if b[i] as u8 >= 0x30 && b[i] as u8 <= 0x39 {
			left = (b[i] as u8 - 0x30) as u32;
			break;
		}
	}

	for j in (0..line.len()).rev() {

		flag = false;
		for (digit, value) in &digits {
			if line.len() < j + digit.len() {
				continue;
			}
			if memncmp(&b[j..], &digit, j + digit.len()) {
				flag = true;
				right = *value;
				break;
			}
		}
		
		if flag {
			break ;
		}

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
