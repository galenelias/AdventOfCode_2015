use std::io::{self, BufRead};
use itertools::Itertools;

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<Vec<u32>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split('x').map(|tok| tok.parse::<u32>().unwrap()).sorted())
		.collect::<Vec<_>>();

	let sides = lines.iter().map(|dims| vec![dims[0]*dims[1], dims[1]*dims[2], dims[0]*dims[2]]).collect::<Vec<_>>();

	let sum = sides.iter().map(|row| row.iter().sum::<u32>()*2 + row.iter().nth(0).unwrap()).sum::<u32>();
	println!("Part 1: {}", sum);

	let part2 = lines.iter().map(|row| row[0]*2 + row[1]*2 + row[0]*row[1]*row[2]).sum::<u32>();
	println!("Part 2: {}", part2);
}