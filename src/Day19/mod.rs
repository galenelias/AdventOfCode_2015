use std::io::{self, BufRead};
use itertools::Itertools;
use std::collections::{HashSet, HashMap};

fn do_replacements(seed: &str, replacements: &HashMap<String, Vec<String>>, max_key_len: usize) -> HashSet<String> {
	let mut outcomes = HashSet::new();

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

					outcomes.insert(new_string);
				}
			}
		}
	}

	return outcomes;
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<String> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.collect_vec();

	let mut replacements : HashMap<String, Vec<String>> = HashMap::new();
	let mut max_key_len = 0;

	for line in &lines {
		let parts = line.split(" => ").map(|s| s.to_string()).collect_vec();
		// println!("{:?}", parts);
		if parts.len() > 1 {
			max_key_len = std::cmp::max(max_key_len, parts[0].len());
			replacements.entry(parts[0].clone()).or_insert(Vec::new()).push(parts[1].clone());
		}
	}


	// println!("{}", max_key_len);
	// println!("{}", lines.last().unwrap());
	let medicine_molecule = lines.last().unwrap();
	let part1 = do_replacements(medicine_molecule, &replacements, max_key_len);
	println!("Part 1: {}", part1.len());


	let mut steps = 0;
	let mut molecules = HashSet::new();
	// molecules.insert("e".to_string());

	let mut queue = Vec::new();
	queue.push("e".to_string());

	loop {
		steps += 1;
		let mut new_molecules = Vec::new();

		for mol in queue {
			let mut outputs = do_replacements(&mol, &replacements, max_key_len);
			for out in outputs {
				if out.len() <= medicine_molecule.len() && molecules.insert(out.clone()) {
					if new_molecules.is_empty() {
						println!("{}", out);
					}
					new_molecules.push(out);
				}
			}
		}

		println!("Step {}, molecules={}, queue={}", steps, molecules.len(), new_molecules.len());
		if molecules.contains(medicine_molecule) {
			break;
		}
		queue = new_molecules;
	}

	println!("Part 2: {}", steps);

}