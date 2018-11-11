use std::io::{self, BufRead};
use regex::Regex;
use itertools::Itertools;

struct Reindeer {
	name: String,
	speed: u32,
	duration: u32,
	rest: u32,
	position: u32,
	points: u32,
}

fn step_race(deer: &Reindeer, time: u32) -> u32 {
	let cycle_len = deer.duration + deer.rest;
	let remainder = time % cycle_len;

	if remainder < deer.duration {
		deer.speed
	} else {
		0
	}
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<String> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.collect_vec();

	let mut deers : Vec<Reindeer> = Vec::new();

	let re_state = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
	for line in &lines {
		let vals = &re_state.captures(&line).unwrap();
		let deer = vals[1].to_string();
		let mut speed = vals[2].parse::<u32>().unwrap();
		let mut dur = vals[3].parse::<u32>().unwrap();
		let mut rest = vals[4].parse::<u32>().unwrap();

		deers.push(Reindeer {name: deer, speed: speed, duration: dur, rest: rest, position: 0, points: 0});
	}
	
	for t in 0..2503 {
		for d in &mut deers {
			let dist = step_race(d, t);
			d.position += dist;
		}

		let furthest_pos = deers.iter_mut().max_by_key(|r| r.position).unwrap().position;
		for d in &mut deers {
			if d.position == furthest_pos {
				d.points += 1;
			}
		}
	}

	let part1 = deers.iter().max_by_key(|r| r.position).unwrap();
	println!("Part 1: {} ({})", part1.position, part1.name);

	let part2 = deers.iter().max_by_key(|r| r.points).unwrap();
	println!("Part 2: {} ({})", part2.points, part2.name);
}