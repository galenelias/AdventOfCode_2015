use std::io::{self, BufRead};
use itertools::Itertools;

// Perf optimization. Attempt to efficiently fix any occurrences of 'i,o,l'
// by immediately incrementing them to the next letter (and zero'ing out following
// letters)
fn fix_pwd(mut pwd: Vec<char>) -> Vec<char> {
	for i in 0..pwd.len() {
		if pwd[i] == 'i' || pwd[i] == 'o' || pwd[i] == 'l' {
			pwd[i] = ((pwd[i] as u8) + 1) as char;

			// Incrementing one digit resets the rest to 'a'
			for j in i+1..pwd.len() {
				pwd[j] = 'a';
			}
			break;
		}
	}
	pwd
}

fn increment_pwd(mut pwd: Vec<char>) -> Vec<char> {
	let mut i = pwd.len();
	while i > 0 {
		i -= 1;

		if pwd[i] != 'z' {
			pwd[i] = ((pwd[i] as u8) + 1) as char;
			break;
		} else {
			pwd[i] = 'a';
		}
	}
	pwd
}

fn is_valid_password(pwd: &Vec<char>) -> bool {
	!pwd.iter().any(|&c| c == 'i' || c == 'o' || c == 'l')
		&& pwd.windows(3).any(|w| w[1] as i32 - w[0] as i32 == 1 && w[2] as i32 - w[0] as i32 == 2)
		&& pwd.windows(2).filter(|&w| w[0] == w[1]).unique().count() >= 2
}

fn find_next_valid_password(mut pwd: Vec<char>) -> Vec<char> {
	while !is_valid_password(&pwd) {
		pwd = fix_pwd(increment_pwd(pwd));
	}
	pwd
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<Vec<char>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.chars().clone().collect_vec())
		.collect_vec();

	for line in lines {
		let pwd = find_next_valid_password(line.clone());
		println!("Part 1: {} -> {}", line.iter().collect::<String>(), pwd.iter().collect::<String>());
	}
}