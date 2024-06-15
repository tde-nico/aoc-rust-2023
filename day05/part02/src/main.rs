use std::{io::{self, BufRead}, ops::Range};

#[derive(Default)]
pub struct Globals {
	seeds: Vec<i64>,
	mapping: Vec<Mapping>,
}

#[derive(Debug, Default)]
struct SingleMap {
	range: Range<i64>,
	delta: i64,
}

#[derive(Debug, Default)]
struct Mapping {
	map: Vec<SingleMap>,
}

impl Mapping {
	fn add_mapping(&mut self, dest: i64, src: i64, len: i64) {
		self.map.push(SingleMap {
			range: Range { start: src, end: src + len },
			delta: dest - src,
		})
	}

	fn reverse_lookup(&self, val: i64) -> i64 {
		for map in &self.map {
			let rev = val - map.delta;
			if map.range.contains(&rev) {
				return rev;
			}
		}
		val
	}

	fn apply_map(&self, val: i64) -> i64 {
		for map in &self.map {
			if map.range.contains(&val) {
				return val + map.delta
			}
		}
		val
	}
}

trait Runner {
	fn parse(&mut self);
	fn part1(&mut self);
	fn part2(&mut self);
	fn part2_slow(&mut self);
}

impl Runner for Globals {
	fn parse(&mut self) {
		let stdin = io::stdin();
		let  mut lines = Vec::<String>::new();
		for line in stdin.lock().lines() {
			lines.push(line.unwrap());
		}

		let seeds = lines[0].split_once(": ").unwrap().1;
		self.seeds = seeds
			.split(' ')
			.map(|seed| seed.parse().unwrap())
			.collect();

		let mut curmap = Mapping::default();
		for line in lines[2..].iter() {
			if line.len() == 0 {
				continue;
			}
			if line.contains(":") {
				self.mapping.push(curmap);
				curmap = Mapping::default();
				continue;
			}
			let nums: Vec<i64> = line
				.split(' ')
				.map(|num| num.parse().unwrap())
				.collect();
			curmap.add_mapping(nums[0], nums[1], nums[2])
		}
		if !curmap.map.is_empty() {
			self.mapping.push(curmap)
		}
	}

	fn part1(&mut self) {
		let mut min = i64::MAX;
		for seed in &self.seeds {
			let mut cur = *seed;
			for map in &self.mapping {
				cur = map.apply_map(cur);
			}
			min = min.min(cur);
		}
		println!("{}", min);
	}
	
	fn part2(&mut self) {
		let seed_ranges = self.seeds
		.chunks(2)
		.map(|vec| Range { start: vec[0], end: vec[0]+vec[1] })
		.collect::<Vec<_>>();
		let mut location = 1_i64;
		loop {
			let mut cur = location;
			for map in self.mapping.iter().rev() {
				cur = map.reverse_lookup(cur);
			}
			for sr in &seed_ranges {
				if sr.contains(&cur) {
					println!("{}", location);
					return
				}
			}
			location += 1;
		}
	}

	fn part2_slow(&mut self) {
		let mut min = i64::MAX;
		for seed_range in self.seeds.chunks(2) {
			for seed in seed_range[0]..seed_range[0]+seed_range[1] {
				let mut cur = seed;
				for map in &self.mapping {
					cur = map.apply_map(cur);
				}
				min = min.min(cur);
			}
		}
		println!("{}", min);
	}
}

fn main() {
    let mut globals = Globals::default();
	Runner::parse(&mut globals);
	Runner::part2(&mut globals)
}
