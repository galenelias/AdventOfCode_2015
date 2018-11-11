use std::io::{self, BufRead};
use regex::Regex;
use itertools::Itertools;
use std::collections::{HashSet, HashMap};
use permutohedron::LexicalPermutation;

fn calc_table_preference_val(people: &Vec<&String>, preferences: &HashMap<(String, String), i32>) -> i32 {
	let mut val: i32 = 0;
	for i in 0..people.len()-1 {
		val += preferences.get(&(people[i].clone(), people[i+1].clone())).unwrap();
		val += preferences.get(&(people[i+1].clone(), people[i].clone())).unwrap();
	}

	val += preferences.get(&(people[0].clone(), people[people.len()-1].clone())).unwrap();
	val += preferences.get(&(people[people.len()-1 ].clone(), people[0].clone())).unwrap();
	return val;
}

pub fn solve_day_13(lines: &Vec<String>, part2: bool) -> i32 {
	let mut people = HashSet::new();
	let mut preferences = HashMap::new();

	let re_state = Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).").unwrap();
	for line in lines {
		let vals = &re_state.captures(&line).unwrap();
		let person1 = vals[1].to_string();
		let person2 = vals[4].to_string();
		let mut value = vals[3].parse::<i32>().unwrap();
		if vals[2].to_string() == "lose" {
			value *= -1;
		}

		people.insert(person1.clone());
		people.insert(person2.clone());

		if part2 {
			preferences.insert((person1.clone(), "Me".to_string()), 0);
			preferences.insert(("Me".to_string(), person1.clone()), 0);
		}

		preferences.insert((person1, person2), value);
	}

	if part2 { 
		people.insert("Me".to_string());
	}

	let mut people_vec = people.iter().collect_vec();
	let mut max_distance: Option<i32> = None;

	loop {
		let dist = calc_table_preference_val(&people_vec, &preferences);

		if max_distance.is_none() || dist > max_distance.unwrap() {
			max_distance = Some(dist);
		}

		if !people_vec.next_permutation() {
			break;
		}
	}

	return max_distance.unwrap();
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<String> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.collect_vec();

	println!("Part 1: {}", solve_day_13(&lines, false));
	println!("Part 2: {}", solve_day_13(&lines, true));
}