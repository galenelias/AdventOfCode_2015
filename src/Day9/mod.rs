use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use permutohedron::LexicalPermutation;

fn calc_route_distance(cities: &[String], distances: &HashMap<(String,String), u32>) -> u32 {
	let mut dist: u32 = 0;
	for i in 0..cities.len()-1 {
		// TODO: More efficient lookups. These clones will be terrible.
		//  HashMap<&str,&str>?
		dist += distances.get(&(cities[i].clone(), cities[i+1].clone())).unwrap();
	}
	return dist;
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<Vec<String>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split_whitespace().map(String::from).collect_vec())
		.collect_vec();

	let mut distances = HashMap::new();
	let mut cities = HashSet::new();

	for line in &lines {
		distances.insert((line[0].clone(), line[2].clone()), line[4].parse::<u32>().unwrap());
		distances.insert((line[2].clone(), line[0].clone()), line[4].parse::<u32>().unwrap());
		cities.insert(line[0].clone());
		cities.insert(line[2].clone());
	}

	let mut cities_vec = cities.iter().cloned().collect_vec();
	let mut min_distance: Option<u32> = None;

	loop {
		let dist = calc_route_distance(&cities_vec[..], &distances);
		if min_distance.is_none() || dist < min_distance.unwrap() {
			min_distance = Some(dist);
		}

		if !cities_vec.next_permutation() {
			break;
		}
	}

	println!("Part 1: {}", min_distance.unwrap_or_default());
}