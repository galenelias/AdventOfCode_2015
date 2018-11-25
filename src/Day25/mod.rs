use std::io::{self, BufRead};
use regex::Regex;

struct Generator {
	value: u64,
	r: usize,
	c: usize,
}

impl Iterator for Generator {
	type Item = (u64, (usize, usize));

	fn next(&mut self) -> Option<Self::Item> {
		let result = (self.value, (self.r, self.c));
		self.value = (self.value * 252533) % 33554393;
		if self.r > 0 {
			self.r -= 1;
			self.c += 1;
		} else {
			self.r = self.c + 1;
			self.c = 0;
		}
		Some(result)
	}
}

pub fn solve() {
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap();

	let regex = Regex::new(r"To continue, please consult the code grid in the manual.  Enter the code at row (\d+), column (\d+).").unwrap();
	let vals = &regex.captures(&input).unwrap();
	let target_row = vals[1].parse::<usize>().unwrap() - 1;
	let target_col = vals[2].parse::<usize>().unwrap() - 1;

	let mut gen = Generator{ r: 0, c: 0, value: 20151125 };
	let (v, _) = gen.find(|&(_, (r, c))| r == target_row && c == target_col).unwrap();
	println!("Part 1: {}", v);
}