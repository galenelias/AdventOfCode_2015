use std::io::{self, BufRead};
use itertools::Itertools;

fn count_bools(grid_in: &Vec<Vec<bool>>, r: i32, c: i32) -> u32 {
	let mut count = 0;
	for i in std::cmp::max(0, r - 1)..std::cmp::min(grid_in.len() as i32, r + 2) {
		for j in std::cmp::max(0, c - 1)..std::cmp::min(grid_in[i as usize].len() as i32, c + 2) {
			if grid_in[i as usize][j as usize] && (i != r || j != c) {
				count += 1;
			}
		}
	}
	return count;
}

fn step_grid(grid_in: &Vec<Vec<bool>>, part2: bool) -> Vec<Vec<bool>> {
	let mut grid_out = grid_in.clone();
	let rows = grid_in.len();
	let cols = grid_in[0].len();

	for i in 0..rows {
		for j in 0..cols {
			let count = count_bools(grid_in, i as i32, j as i32);
			if grid_in[i][j] && !(count == 2 || count == 3) {
				grid_out[i][j] = false;
			} else if !grid_in[i][j] && count == 3 {
				grid_out[i][j] = true;
			}
		}
	}
	if part2 {
		grid_out[0][0] = true;
		grid_out[0][cols-1] = true;
		grid_out[rows-1][0] = true;
		grid_out[rows-1][cols-1] = true;
	}
	return grid_out;
}

fn print_grid(grid: &Vec<Vec<bool>>) {
	for row in grid {
		println!("{}", row.iter().map(|b| if *b { '#' } else { '.'}).collect::<String>());
	}
}

fn count_grid(grid: &Vec<Vec<bool>>) -> usize {
	grid.iter().map(|row| row.iter().filter(|b| **b).count()).sum::<usize>()
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<Vec<bool>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.chars().map(|c| c == '#').collect_vec())
		.collect_vec();

	let mut part1 = lines.clone();
	for _ in 0..100 {
		part1 = step_grid(&part1, false /*part2*/);
	}
	println!("Part 1: {}", count_grid(&part1));

	let rows = lines.len();
	let cols = lines[0].len();
	let mut part2 = lines.clone();
	part2[0][0] = true;
	part2[0][cols-1] = true;
	part2[rows-1][0] = true;
	part2[rows-1][cols-1] = true;

	for _ in 0..100 {
		part2 = step_grid(&part2, true /*part2*/);
	}
	println!("Part 2: {}", count_grid(&part2));
}
