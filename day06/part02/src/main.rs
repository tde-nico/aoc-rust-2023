use std::io::{self, BufRead};

#[derive(Default)]
pub struct Globals {
	races: Vec<RaceInfo>,
	part2race: RaceInfo,
}

#[derive(Default)]
struct RaceInfo {
	time: i64,
	record: i64,
}

impl RaceInfo {
	fn wins(&self) -> i64 {
		let a = -1f64;
		let b = self.time as f64;
		let c =  (-self.record) as f64;
	
		let mut first = ((-b + (b * b - 4. * a * c).sqrt()) / (2. * a)).ceil();
		let mut second = ((-b - (b * b - 4. * a * c).sqrt()) / (2. * a)).floor();
	
		if (first * (b - first)) == self.record as f64 {
			first += 1.;
		}
		if (second * (b - second)) == self.record as f64 {
			second -= 1.;
		}
		return (second - first) as i64 + 1;
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
		let  mut lines = Vec::<String>::new();
		for line in stdin.lock().lines() {
			lines.push(line.unwrap());
		}

		let times = lines[0]
			.split_once(":")
			.unwrap()
			.1
			.split_whitespace()
			.map(|val| val.parse().unwrap())
			.collect::<Vec<i64>>();
		let records = lines[1]
			.split_once(":")
			.unwrap()
			.1
			.split_whitespace()
			.map(|val| val.parse().unwrap())
			.collect::<Vec<i64>>();

		for (&time, &record) in times.iter().zip(records.iter()) {
			self.races.push(RaceInfo { time, record });
		}

		let time = lines[0]
			.split_once(":")
			.unwrap()
			.1
			.chars()
			.filter(|ch| !ch.is_whitespace())
			.collect::<String>()
			.parse::<i64>()
			.unwrap();
		let record = lines[1]
			.split_once(":")
			.unwrap()
			.1
			.chars()
			.filter(|ch| !ch.is_whitespace())
			.collect::<String>()
			.parse::<i64>()
			.unwrap();

		self.part2race = RaceInfo{time, record};
	}

	fn part1(&mut self) {
		println!("{}", self.races.iter().fold(1, |acc, race| acc * race.wins()));
	}

	fn part2(&mut self) {
		println!("{}", self.part2race.wins());
	}
}

fn main() {
    let mut globals = Globals::default();
	Runner::parse(&mut globals);
	Runner::part2(&mut globals)
}
