use std::io::{self, BufRead};
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};

pub fn solve() {
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap();
	let nums = input.chars().map(|ch| match ch { '(' => 1, ')' => -1, _ => unreachable!()}).collect::<Vec<_>>();

	println!("Part1: {}", nums.iter().sum::<i32>());

	let mut part2_iter = nums.iter();
	part2_iter.fold_while(0, |acc, x| if acc + x >= 0 { Continue(acc + x) } else { Done(acc) }).into_inner();

	println!("Part 2: {}", nums.len() - part2_iter.len());
}