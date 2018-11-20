use std::io::{self, BufRead};
use itertools::Itertools;

fn get_score(num: u32) -> u32 {
	let mut score = 0;
	for i in 1..=num/2 {
		if num % i == 0 {
			score += i*10;
		}
	}
	score += num * 10;
	return score;
}

pub fn solve() {
	let stdin = io::stdin();
	let target = stdin.lock().lines().next().unwrap().unwrap().parse::<u32>().unwrap();

	let mut upper = 0;
	for i in 0.. {
		let two_to_i = 2u32.pow(i);
		let score = get_score(two_to_i);
		if score >= target {
			upper = two_to_i;
			break;
		}
	}

	let mut seive = vec![10; upper as usize];
	for i in 2..=upper {
		for k in 1.. {
			if i*k >= upper {
				break;
			}

			seive[(i*k) as usize] += i*10;
		}
	}

	let part1 = seive.iter().enumerate().find(|(_i, v)| *v >= &target);
	println!("Part 1: {} = {}", part1.unwrap().0, part1.unwrap().1);


	let mut seive2 = vec![0; upper as usize];
	for i in 1..=upper {
		for k in 1..=50 {
			if i*k >= upper {
				break;
			}

			seive2[(i*k) as usize] += i*11;
		}
	}
	let part2 = seive2.iter().enumerate().find(|(_i, v)| *v >= &target);
	println!("Part 2: {} = {}", part2.unwrap().0, part2.unwrap().1);

}