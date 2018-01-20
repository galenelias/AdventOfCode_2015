use std::io::{self, BufRead};
use md5;

fn find_bitcoin(salt : &str, zero_count : usize) -> usize {
	for i in 0.. {
		let hash = format!("{:x}", md5::compute(format!("{}{}", salt, i)));
		if hash.chars().take(zero_count).all(|ch| ch == '0') {
			return i;
		}
	}
	unreachable!();
}

pub fn solve() {
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap();

	println!("Part 1: {}", find_bitcoin(&input, 5));
	println!("Part 2: {}", find_bitcoin(&input, 6));
}