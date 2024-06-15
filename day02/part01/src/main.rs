use std::io::{self, BufRead};

#[derive(Default)]
pub struct Globals {
	games: Vec<Vec<Turn>>,
}

#[derive(Debug, Default)]
struct Turn {
	red: usize,
	green: usize,
	blue: usize,
}

impl Turn {
	fn is_valid(&self) -> bool {
		self.red <= 12 && self.green <= 13 && self.blue <= 14
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
		for line in stdin.lock().lines() {
			let data = line.unwrap();
			let (_, turns) = data.split_once(": ").unwrap();
			let mut turn_list = Vec::new();

			let turns = turns.split("; ").collect::<Vec<_>>();
			for t in turns {
				let mut turn = Turn::default();

				let cubes = t.split(", ").collect::<Vec<_>>();
				for c in cubes {
					let (amount, color) = c.split_once(' ').unwrap();
					let amount: usize = amount.parse().unwrap();

					match &color[0..1] {
						"r" => turn.red = amount,
						"g" => turn.green = amount,
						"b" => turn.blue = amount,
						_ => panic!("Color Error\n"),
					}
				}
				turn_list.push(turn);
			}
			self.games.push(turn_list);
		}
	}

	fn part1(&mut self) {
		let mut valid_games = 0;
		'next_game: for (i, game) in self.games.iter().enumerate() {
			for turn in game {
				if !turn.is_valid() {
					continue 'next_game;
				}
			}
			valid_games += i + 1;
		}
		println!("{valid_games}");
	}
	
	fn part2(&mut self) {
		println!("Part 2 Not Implemented");
	}
}

fn main() {
	let mut globals = Globals::default();
	Runner::parse(&mut globals);
	Runner::part1(&mut globals);
}
