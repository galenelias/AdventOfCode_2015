use std::io::{self, BufRead};
use itertools::Itertools;
use std::collections::{HashSet, HashMap};

fn do_replacements(seed: &str, replacements: &HashMap<String, Vec<String>>, max_key_len: usize, output: &mut HashSet<String>)  {
	for i in 0..seed.len() {
		for j in 1..=max_key_len {
			if i + j > seed.len() {
				continue;
			}

			let x = seed[i..i+j].to_string();
			if let Some(vals) = replacements.get(&x) {
				for val in vals {
					let mut new_string = seed[..i].to_string();
					new_string.push_str(&val);
					new_string.push_str(&seed[i+j..]);

					output.insert(new_string);
				}
			}
		}
	}
}

// Find the largest replacement, and do that
fn replace_greedy(seed: &str, replacements: &HashMap<String, Vec<String>>, max_key_len: usize) -> String  {
	let mut result = String::new();
	let mut longest_replacement = 0;

	for i in 0..seed.len() {
		for j in longest_replacement+1..=max_key_len {
			if i + j > seed.len() {
				continue;
			}

			let x = seed[i..i+j].to_string();
			if let Some(vals) = replacements.get(&x) {
				for val in vals {
					let mut new_string = seed[..i].to_string();
					new_string.push_str(&val);
					new_string.push_str(&seed[i+j..]);

					result = new_string;
					longest_replacement = j;
				}
			}
		}
	}
	return result;
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<String> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.collect_vec();

	let mut replacements : HashMap<String, Vec<String>> = HashMap::new();
	let mut inv_replacements : HashMap<String, Vec<String>> = HashMap::new();
	let mut max_key_len1 = 0;
	let mut max_key_len2 = 0;

	for line in &lines {
		let parts = line.split(" => ").map(|s| s.to_string()).collect_vec();
		if parts.len() > 1 {
			max_key_len1 = std::cmp::max(max_key_len1, parts[0].len());
			max_key_len2 = std::cmp::max(max_key_len2, parts[1].len());
			replacements.entry(parts[0].clone()).or_insert(Vec::new()).push(parts[1].clone());
			inv_replacements.entry(parts[1].clone()).or_insert(Vec::new()).push(parts[0].clone());
		}
	}

	let medicine_molecule = lines.last().unwrap();
	let mut part1 = HashSet::new();
	do_replacements(medicine_molecule, &replacements, max_key_len1, &mut part1);
	println!("Part 1: {}", part1.len());

	let mut steps = 0;
	let mut m = medicine_molecule.clone();
	loop {
		steps += 1;
		m = replace_greedy(&m, &inv_replacements, max_key_len2);

		if m == "e" {
			break;
		}
	}

	println!("Part 2: {}", steps);
}