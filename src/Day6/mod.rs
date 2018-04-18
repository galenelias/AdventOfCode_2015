use std::io::{self, BufRead};
use itertools::Itertools;
use std::cmp;

fn part1(lines : &Vec<Vec<String>>) {
	let mut grid = vec![vec![false; 1000]; 1000];

	for line in lines {
		match line[0].as_ref() {
			"turn" => {
				let val = match line[1].as_ref() {
					"off" => false,
					"on" => true,
					_ => unreachable!(),
				};

				let start_coords = line[2].split(',').map(|w| w.parse::<usize>().unwrap()).collect_vec();
				let end_coords = line[4].split(',').map(|w| w.parse::<usize>().unwrap()).collect_vec();

				for r in start_coords[0]..end_coords[0]+1 {
					for c in start_coords[1]..end_coords[1]+1 {
						grid[r][c] = val;
					}
				}
			},
			"toggle" => {
				let start_coords = line[1].split(',').map(|w| w.parse::<usize>().unwrap()).collect_vec();
				let end_coords = line[3].split(',').map(|w| w.parse::<usize>().unwrap()).collect_vec();

				for r in start_coords[0]..end_coords[0]+1 {
					for c in start_coords[1]..end_coords[1]+1 {
						grid[r][c] = !grid[r][c];
					}
				}
			},
			_ => unreachable!(),
		}
	}

	let lights = grid.iter().map(|row| row.iter().filter(|&v| *v == true).count()).sum::<usize>();
	println!("Part 1: {}", lights);
}

fn part2(lines : &Vec<Vec<String>>) {
	let mut grid = vec![vec![0i32; 1000]; 1000];

	for line in lines {
		match line[0].as_ref() {
			"turn" => {
				let val = match line[1].as_ref() {
					"off" => -1,
					"on" => 1,
					_ => unreachable!(),
				};

				let start_coords = line[2].split(',').map(|w| w.parse::<usize>().unwrap()).collect_vec();
				let end_coords = line[4].split(',').map(|w| w.parse::<usize>().unwrap()).collect_vec();

				for r in start_coords[0]..end_coords[0]+1 {
					for c in start_coords[1]..end_coords[1]+1 {
						grid[r][c] = cmp::max(0, grid[r][c] + val);
					}
				}
			},
			"toggle" => {
				let start_coords = line[1].split(',').map(|w| w.parse::<usize>().unwrap()).collect_vec();
				let end_coords = line[3].split(',').map(|w| w.parse::<usize>().unwrap()).collect_vec();

				for r in start_coords[0]..end_coords[0]+1 {
					for c in start_coords[1]..end_coords[1]+1 {
						grid[r][c] += 2;
					}
				}
			},
			_ => unreachable!(),
		}
	}

	let lights = grid.iter().map(|row| row.iter().sum::<i32>()).sum::<i32>();
	println!("Part 2: {}", lights);
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<Vec<_>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split_whitespace().map(String::from).collect_vec())
		.collect::<Vec<_>>();

	part1(&lines);
	part2(&lines);
}