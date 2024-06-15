use std::{io::{self, BufRead}, collections::HashSet};

#[derive(Default)]
pub struct Globals {
	cards: Vec<Card>,
}

struct Card {
	winning: HashSet<i64>,
	chosen: HashSet<i64>,
}

impl Card {
	fn count(&self) -> usize {
		self.winning
			.intersection(&self.chosen)
			.count()
	}

	fn score(&self) -> i64 {
		let cards = self.count();
		if cards > 0 {
			return 1 << (cards - 1);
		}
		return 0;
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
			let (_, nums) = data.split_once(": ").unwrap();
			let (win, chose) = nums.split_once(" | ").unwrap();
			let winning = win
				.split_whitespace()
				.map(|num| num.parse::<i64>().unwrap())
				.collect::<HashSet<_>>();
			let chosen = chose
				.split_whitespace()
				.map(|num| num.parse::<i64>().unwrap())
				.collect::<HashSet<_>>();
			self.cards.push(Card { winning, chosen });
		}
	}

	fn part1(&mut self) {
		let total = self.cards
			.iter()
			.map(Card::score)
			.sum::<i64>();
		println!("{}", total);
	}

	fn part2(&mut self) {
		let mut mul = vec![1usize; self.cards.len()];
		for (i, card) in self.cards.iter().enumerate() {
			let count = card.count();
			for j in i+1..i+count+1 {
				mul[j] += mul[i];
			}
		}
		println!("{}", mul.iter().sum::<usize>());
	}
}

fn main() {
    let mut globals = Globals::default();
	Runner::parse(&mut globals);
	Runner::part2(&mut globals)
}
