use std::io::{self, BufRead};
use itertools::Itertools;

fn check_fit(containers: &[u32], target: u32, mask: u32) -> bool {
	let mut total = 0;
	for i in 0..containers.len() {
		if mask & (1 << i) != 0 {
			total += containers[i];
		}
	}
	return total == target;
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<u32> = stdin.lock().lines()
		.filter_map(|line| line.ok()).filter_map(|l| l.parse::<u32>().ok())
		.collect_vec();

	let mut variations = 0;
	let mut min_containers = lines.len() as u32;
	let mut min_container_count = 0;

	for i in 0..=2u32.pow(lines.len() as u32) {
		if check_fit(&lines, 150, i) {
			let ones = i.count_ones();
			if ones < min_containers {
				min_containers = ones;
				min_container_count = 1;
			} else if ones == min_containers {
				min_container_count += 1;
			}
			variations += 1;
		}
	}

	println!("Part 1: {}", variations);
	println!("Part 2: {} (container count = {})", min_container_count, min_containers);
}