use std::io::{self, BufRead};
use regex::Regex;
use itertools::Itertools;

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<String> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.collect_vec();

	let re = Regex::new("-?[[:digit:]]+").unwrap();
	for line in &lines {
		let sum = re.find_iter(&line).filter_map(|m| m.as_str().parse::<i32>().ok()).sum::<i32>();
		println!("Part 1: {}", sum);
	}
}