use std::io::{self, BufRead};
use itertools::Itertools;

fn decode(input: &str) -> String {
	let mut it = input.chars().skip(1);
	let mut decoded = String::new();

	while let Some(ch) = it.next() {
		if ch == '\"' {
			break;
		} else if ch == '\\' {
			let next_ch = it.next().unwrap();
			if next_ch == 'x' {
				let _digit1 = it.next().unwrap();
				let _digit2 = it.next().unwrap();
				decoded.push('X'); // TODO, hex decode...
			} else {
				decoded.push(next_ch);
			}
		} else {
			decoded.push(ch);
		}
	}

	return decoded;
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<String> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.collect_vec();

	for line in &lines {
		println!("{}", line)
	}

	for line in lines.iter().map(|l| decode(l)) {
		println!("{}", line)
	}

	let original = lines.iter().map(|l| l.len()).sum::<usize>();
	let decoded = lines.iter().map(|l| decode(l)).map(|l| l.len()).sum::<usize>();
	let part1 = original - decoded;
	println!("Part 1: {} ({} - {})", part1, original, decoded);
}