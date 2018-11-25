use std::io::{self, BufRead};
use itertools::Itertools;

fn check_splits(vals: &Vec<u32>, pos: usize, target: u32) -> (bool, Option<(usize, u64)>) {
	let mut sum : u32 = 0;
	let mut first_index = None;

	for i in 0..pos {
		sum += vals[i];
		if sum == target {
			if first_index.is_none() {
				first_index = Some(i+1);
			}
			sum = 0;
		} else if sum > target {
			return (false, None);
		}
	}

	if let Some(index) = first_index {
		let product = vals.iter().take(index).fold(1u64, |acc, &x| acc * (x as u64));
		return (true, Some((index, product)));
	} else {
		return (true, None);
	}
}

fn should_backtrack(vals: &mut Vec<u32>, pos: usize, target: u32, best: Option<(usize,u64)>) -> bool {
	let (pass, first_group) = check_splits(vals, pos, target);
	if !pass || (best.is_some() && first_group.is_none() && pos > best.unwrap().0) || (best.is_some() && first_group.is_some() && first_group.unwrap() >= best.unwrap()) {
		return true;
	}

	return false;
}

fn get_score(vals: &mut Vec<u32>, target: u32) -> Option<(usize,u64)> {
	let result = check_splits(vals, vals.len(), target);
	if result.0 {
		println!("Solution: {:?} = {} ({})", vals, result.1.unwrap().0, result.1.unwrap().1,);
	}
	return result.1
}

fn optimize(vals: &mut Vec<u32>, pos: usize, target: u32, mut best: Option<(usize,u64)>) -> Option<(usize,u64)> {
	if pos == vals.len() {
		println!("Try: {:?} ({})", vals, best.is_some());
		return get_score(vals, target);
	} else if should_backtrack(vals, pos, target, best) {
		if best.is_some() {
			// println!("Backtracking: pos={}, vals={:?}", pos, vals);
		}
		return None;
	}

	for i in pos..vals.len() {
		vals.swap(pos, i);
		let outcome = optimize(vals, pos+1, target, best);
		if best.is_none() || (outcome.is_some() && outcome.unwrap() < best.unwrap()) {
			best = outcome;

			// Now that we have a solution, most solutions in this sub-tree will be just as good, so potentially bail out early
			if should_backtrack(vals, pos, target, best) {
				vals.swap(pos, i);
				return best;
			}

		}
		vals.swap(pos, i);
	}

	return best;
}


pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<u32> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.parse::<u32>().unwrap())
		.collect_vec();

	let mut lines = lines.into_iter().rev().collect_vec();
	let sum: u32 = lines.iter().sum();

	let part1 = optimize(&mut lines, 0, sum / 3, None);
	println!("Part 1: {} ({})", part1.unwrap().0, part1.unwrap().1);

	let part2 = optimize(&mut lines, 0, sum / 4, None);
	println!("Part 2: {} ({})", part2.unwrap().0, part2.unwrap().1);

}