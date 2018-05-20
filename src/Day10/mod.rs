use std::io::{self, BufRead};
use itertools::Itertools;

fn look_and_say(input: &str) -> String {
	let mut result = String::new();

	let chars = input.chars().collect_vec();

	let mut i = 0;

	while i < chars.len() {
		let mut j = i + 1;
		while j < chars.len() {
			if chars[i] != chars[j] {
				break;
			}
			j += 1;
		}
		result.push_str(&(j - i).to_string());
		result.push(chars[i]);

		i = j;
	}
	return result;
}

pub fn solve() {
	let stdin = io::stdin();
	let mut input = stdin.lock().lines().next().unwrap().unwrap();

	println!("Input: {}", input);
	println!("Output: {}", look_and_say(&input));

	for _i in 0..40 {
		input = look_and_say(&input);
	}

	println!("Part 1: {}", input.len());

	for _i in 0..10 {
		input = look_and_say(&input);
	}

	println!("Part 2: {}", input.len());
}