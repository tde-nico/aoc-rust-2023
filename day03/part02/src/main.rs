use std::{io::{self, BufRead}, collections::HashSet};

#[derive(Default)]
pub struct Globals {
	nums: Vec<PartNumber>,
	syms: HashSet<(i64, i64)>,
	gears: HashSet<(i64, i64)>,
}

struct PartNumber {
	value: i64,
	points: HashSet<(i64, i64)>,
}

impl PartNumber {
	fn new(row: i64, col: i64, ch: char) -> Self {
		let points = HashSet::from([
			(row - 1, col - 1), (row - 1, col), (row - 1, col + 1),
			(row, col - 1), (row, col + 1),
			(row + 1, col - 1), (row + 1, col), (row + 1, col + 1),
		]);
		Self {
			value: (ch as u8 - b'0') as i64,
			points: points,
		}
	}

	fn add_digit(&mut self, row: i64, col: i64, ch: char) {
		self.value = self.value * 10 + (ch as u8 - b'0') as i64;
		self.points.extend([
			(row - 1, col + 1),
			(row, col + 1),
			(row + 1, col + 1)
		]);
	}
}

trait Runner {
	fn parse(&mut self);
	fn part1(&mut self);
	fn part2(&mut self);
}

impl Runner for Globals {
	fn parse(&mut self) {
		let stdin = io::stdin();

		let mut cur_num: Option<PartNumber> = None;
		for (row, line) in stdin.lock().lines().enumerate() {
			for (col, ch) in line.unwrap().chars().enumerate() {
				if ch.is_ascii_digit() {
					if let Some(ref mut num) = cur_num {
						num.add_digit(row as i64, col as i64, ch);
					} else {
						cur_num = Some(PartNumber::new(row as i64, col as i64, ch));
					}
				} else {
					if let Some(num) = cur_num.take() {
						self.nums.push(num);
					}
					if ch != '.' {
						self.syms.insert((row as i64, col as i64));
						if ch == '*' {
							self.gears.insert((row as i64, col as i64));
						}
					}
				}
			}
		}
	}

	fn part1(&mut self) {
		let total = self
			.nums
			.iter()
			.filter(|num| num.points
				.intersection(&self.syms)
				.next()
				.is_some()
			)
			.map(|num| num.value)
			.sum::<i64>();
		println!("{}", total);
	}

	fn part2(&mut self) {
		let mut total = 0;

		'next_gear: for gear in &self.gears {
			let mut matches = Vec::new();
			for num in &self.nums {
				if num.points.contains(gear) {
					if matches.len() == 2 {
						continue 'next_gear;
					}
					matches.push(num.value);
				}
			}
			if matches.len() == 2 {
				total += matches[0] * matches[1]
			}
		}

		println!("{}", total);
	}
}

fn main() {
    let mut globals = Globals::default();
	Runner::parse(&mut globals);
	Runner::part2(&mut globals)
}
