use std::io::{self, BufRead};

#[derive(Default)]
pub struct Globals {
	races: Vec<RaceInfo>
}


struct RaceInfo {
	time: i64,
	record: i64,
}

trait Runner {
	fn parse(&mut self);
	fn part1(&mut self);
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
	}

	fn part1(&mut self) {
		let mut ans = 1;
		for race in &self.races {
			let a = -1f64;
			let b = race.time as f64;
			let c =  (-race.record) as f64;

			let mut first = ((-b + (b * b - 4. * a * c).sqrt()) / (2. * a)).ceil();
			let mut second = ((-b - (b * b - 4. * a * c).sqrt()) / (2. * a)).floor();
		
			if (first * (b - first)) == race.record as f64 {
				first += 1.;
			}
			if (second * (b - second)) == race.record as f64 {
				second -= 1.;
			}
			ans *= (second - first) as i64 + 1;
		}
		println!("{}", ans);
	}
}

fn main() {
    let mut globals = Globals::default();
	Runner::parse(&mut globals);
	Runner::part1(&mut globals)
}
