use std::io::{self, BufRead};
use itertools::Itertools;

struct PersonData {
	children: u32,
	cats: u32,
	samoyeds: u32,
	pomeranians: u32,
	akitas: u32,
	vizslas: u32,
	goldfish: u32,
	trees: u32,
	cars: u32,
	perfumes: u32,
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<String> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.collect_vec();

	let evidence = PersonData {
		children: 3,
		cats: 7,
		samoyeds: 2,
		pomeranians: 3,
		akitas: 0,
		vizslas: 0,
		goldfish: 5,
		trees: 3,
		cars: 2,
		perfumes: 1,
	};

	for line in &lines {
		let mut is_match_part1 = true;
		let mut is_match_part2 = true;

		for s in line[(line.find(':').unwrap()+2)..].split(", ") {
			let parts = s.split(": ").collect_vec();
			let amount = parts[1].parse::<u32>().unwrap();

			is_match_part1 = is_match_part1 && amount == match parts[0] {
				"children" => evidence.children,
				"cats" => evidence.cats,
				"samoyeds" => evidence.samoyeds,
				"pomeranians" => evidence.pomeranians,
				"akitas" => evidence.akitas,
				"vizslas" => evidence.vizslas,
				"goldfish" => evidence.goldfish,
				"trees" => evidence.trees,
				"cars" => evidence.cars,
				"perfumes" => evidence.perfumes,
				_ => unreachable!("Unexpected {}", parts[0]),
			};

			is_match_part2 = is_match_part2 && match parts[0] {
				"pomeranians" => amount < evidence.pomeranians,
				"goldfish" => amount < evidence.goldfish,
				"children" => amount == evidence.children,
				"samoyeds" => amount == evidence.samoyeds,
				"akitas" => amount == evidence.akitas,
				"vizslas" => amount == evidence.vizslas,
				"cars" => amount == evidence.cars,
				"perfumes" => amount == evidence.perfumes,
				"cats" => amount > evidence.cats,
				"trees" => amount >= evidence.trees,
				_ => unreachable!("Unexpected {}", parts[0]),
			};

		}
		if is_match_part1 {
			println!("Part 1: {}", line);
		}
		if is_match_part2 {
			println!("Part 2: {}", line);
		}
	}
}