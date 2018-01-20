use std::io::{self, BufRead};
use itertools::Itertools;
use std::collections::HashMap;

fn part1 (lines : &[Vec<char>]) -> usize {
	lines.iter().filter(|line| {
		let unique_vowels = line.iter().filter(|ch| match **ch { 'a'|'e'|'i'|'o'|'u' => true, _ => false }).count();
		let any_repeat = line.windows(2).any(|w| w[0] == w[1]);
		let any_naughty = line.windows(2).any(|w| w == ['a','b'] || w == ['c','d'] || w == ['p','q'] || w == ['x','y']);

		unique_vowels >= 3 && any_repeat && !any_naughty
	}).count()
}

fn part2 (lines : &[Vec<char>]) -> usize {
	lines.iter().filter(|line| {
		let windows = line.windows(2).collect_vec();
		let mut pairs = HashMap::new();
		
		for w in &windows {
			*pairs.entry(w).or_insert(0) += 1;
		}

		let any_dupe_pair = pairs.iter().filter(|&(_k, v)| *v > 1).any(|(k,_v)| {
			let poss = windows.iter().positions(|w| &w == k).collect_vec();
			poss.iter().max().unwrap() > &(poss.iter().min().unwrap() + 1)
		});
		let has_xyx = line.windows(3).any(|w| w[0] == w[2]);

		any_dupe_pair && has_xyx
	}).count()
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<Vec<char>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.chars().clone().collect_vec())
		.collect::<Vec<_>>();

		println!("Part 1: {}", part1(&lines));
		println!("Part 2: {}", part2(&lines));
}