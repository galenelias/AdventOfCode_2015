use std::io::{self, BufRead};
use std::collections::HashMap;
use itertools::Itertools;

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split(" -> ").map(String::from).collect_vec())
		.collect::<Vec<_>>();

	let mut wires = HashMap::new();

	for line in lines {
		if let Ok(val) = line[0].parse::<u16>() {
			let wire = line[1].clone();
			wires.insert(wire, val);
		}
		else if line[0].contains("AND") {
			let parts = line[0].split(" AND ").collect_vec();
			let val1 = wires.get(parts[0]).unwrap().clone();
			let val2 = wires.get(parts[1]).unwrap().clone();
			wires.insert(line[1].clone(), val1 & val2);
		}
		else if line[0].contains("OR") {
			let parts = line[0].split(" OR ").collect_vec();
			let val1 = wires.get(parts[0]).unwrap().clone();
			let val2 = wires.get(parts[1]).unwrap().clone();
			wires.insert(line[1].clone(), val1 | val2);
		}
		else if line[0].contains("LSHIFT") {
			let parts = line[0].split(" LSHIFT ").collect_vec();
			let val1 = wires.get(parts[0]).unwrap().clone();
			let val2 = parts[1].parse::<u16>().unwrap();
			wires.insert(line[1].clone(), val1 << val2);
		}
		else if line[0].contains("RSHIFT") {
			let parts = line[0].split(" RSHIFT ").collect_vec();
			let val1 = wires.get(parts[0]).unwrap().clone();
			let val2 = parts[1].parse::<u16>().unwrap();
			wires.insert(line[1].clone(), val1 >> val2);
		}
		else if line[0].contains("NOT") {
			let parts = line[0].split("NOT ").collect_vec();
			let val1 = wires.get(parts[1]).unwrap().clone();
			wires.insert(line[1].clone(), !val1);
		}
		else {
			println!("Unprocessed line: {}", line[0]);
		}
	}

	for (k,v) in wires {
		println!("{} -> {}", k, v);
	}
}