use std::io::{self, BufRead};

pub fn solve() {
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap();
	let nums = input.chars().map(|ch| match ch { '(' => 1, ')' => -1, _ => unreachable!()}).collect::<Vec<_>>();

	println!("Part1: {}", nums.iter().sum::<i32>());

	let mut sum = 0;
	for (i, v) in nums.iter().enumerate() {
		sum += v;
		if sum < 0 {
			println!("Part 2: {}", i+1);
			break;
		}
	}
}