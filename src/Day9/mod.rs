use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use permutohedron::LexicalPermutation;

fn calc_route_distance(cities: &[usize], distances: &Vec<Vec<u32>>) -> u32 {
	let mut dist: u32 = 0;
	for i in 0..cities.len()-1 {
		dist += distances[cities[i]][cities[i+1]];
	}
	return dist;
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<Vec<String>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split_whitespace().map(String::from).collect_vec())
		.collect_vec();

	let mut cities = HashSet::new();
	for line in &lines {
		cities.insert(line[0].clone());
		cities.insert(line[2].clone());
	}

	let cities_vec = cities.iter().collect_vec();
	let mut cities_idx_map = HashMap::new();
	for (i, val) in cities_vec.iter().enumerate() {
		cities_idx_map.insert(val.clone(), i);
	}

	let mut distances = vec![vec![0; cities_vec.len()]; cities_vec.len()];

	for line in &lines {
		let city1_ord = cities_idx_map.get(&line[0]).unwrap();
		let city2_ord = cities_idx_map.get(&line[2]).unwrap();
		let dist = line[4].parse::<u32>().unwrap();
		distances[*city1_ord][*city2_ord] = dist;
		distances[*city2_ord][*city1_ord] = dist;
	}

	let mut cities_vec_nums = (0..cities_vec.len()).collect_vec();
	let mut min_distance: Option<u32> = None;
	let mut max_distance: Option<u32> = None;

	loop {
		let dist = calc_route_distance(&cities_vec_nums[..], &distances);
		if min_distance.is_none() || dist < min_distance.unwrap() {
			min_distance = Some(dist);
		}

		if max_distance.is_none() || dist > max_distance.unwrap() {
			max_distance = Some(dist);
		}

		if !cities_vec_nums.next_permutation() {
			break;
		}
	}

	println!("Part 1: {}", min_distance.unwrap_or_default());
	println!("Part 2: {}", max_distance.unwrap_or_default());
}