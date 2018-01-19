use std::io::{self, BufRead};
use std::collections::HashMap;
use itertools::Itertools;

fn deliver_presents<'a, I>(directions: I, grid: &mut HashMap<(i32,i32),usize>)
where
    I: Iterator<Item = &'a char>,
{
	grid.insert((0,0), 1);

	let mut pos = (0,0);
	for ch in directions {
		pos = match *ch {
			'<' => (pos.0 - 1, pos.1),
			'>' => (pos.0 + 1, pos.1),
			'^' => (pos.0, pos.1 - 1),
			'v' => (pos.0, pos.1 + 1),
			_ => unreachable!(),
		};
		*grid.entry(pos).or_insert(0) += 1;		
	}
}

fn part1(input : &[char]) {
	let mut grid1 = HashMap::new();
	deliver_presents(input.iter(), &mut grid1);
	println!("Part 1: {}", grid1.iter().filter(|&(_k,&v)| v > 0).count());
}

fn part2(input : &[char]) {
	let mut grid = HashMap::new();
	deliver_presents(input.iter().step(2), &mut grid);
	deliver_presents(input.iter().skip(1).step(2), &mut grid);
	println!("Part 2: {}", grid.iter().filter(|&(_k,&v)| v > 0).count());
}


pub fn solve() {
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap().chars().collect::<Vec<_>>();

	part1(&input);
	part2(&input);
}