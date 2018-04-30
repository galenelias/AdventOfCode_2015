use std::io::{self, BufRead};
use std::collections::HashMap;
use itertools::Itertools;

fn get_wire_value(name: &str, rules: &HashMap<String, String>, memo: &mut HashMap<String, u16>) -> u16 {
	// Might be a literal value rather than a wire name, or maybe we remember the result
	if let Ok(val) = name.parse::<u16>() {
		return val;
	} else if let Some(val) = memo.get(name) {
		return *val;
	}

	let rule = rules.get(name).unwrap();
	let result: u16;
	if let Ok(val) = rule.parse::<u16>() {
		return val;
	} else if rule.contains("AND") {
		let parts = rule.split(" AND ").collect_vec();
		let val1 = get_wire_value(parts[0], rules, memo);
		let val2 = get_wire_value(parts[1], rules, memo);
		result = val1 & val2;
	} else if rule.contains("OR") {
		let parts = rule.split(" OR ").collect_vec();
		let val1 = get_wire_value(parts[0], rules, memo);
		let val2 = get_wire_value(parts[1], rules, memo);
		result = val1 | val2;
	} else if rule.contains("LSHIFT") {
		let parts = rule.split(" LSHIFT ").collect_vec();
		let val1 = get_wire_value(parts[0], rules, memo);
		let val2 = parts[1].parse::<u16>().unwrap();
		result = val1 << val2;
	} else if rule.contains("RSHIFT") {
		let parts = rule.split(" RSHIFT ").collect_vec();
		let val1 = get_wire_value(parts[0], rules, memo);
		let val2 = parts[1].parse::<u16>().unwrap();
		result = val1 >> val2;
	} else if rule.contains("NOT") {
		let parts = rule.split("NOT ").collect_vec();
		let val1 = get_wire_value(parts[1], rules, memo);
		result = !val1;
	} else {
		result = get_wire_value(rule, rules, memo);
	}

	memo.insert(name.to_string(), result);
	result
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split(" -> ").map(String::from).collect_vec())
		.collect::<Vec<_>>();

	let mut rules = HashMap::new();
	for line in lines {
		rules.insert(line[1].clone(), line[0].clone());
	}

	let mut memo_p1 = HashMap::new();
	let wire_a_p1 = get_wire_value("a", &rules, &mut memo_p1);
	println!("Part 1: {}", wire_a_p1);

	let mut memo_p2 = HashMap::new();
	memo_p2.insert("b".to_string(), wire_a_p1);
	let wire_a_p2 = get_wire_value("a", &rules, &mut memo_p2);
	println!("Part 2: {}", wire_a_p2);
}