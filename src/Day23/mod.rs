use std::io::{self, BufRead};
use itertools::Itertools;
use std::collections::HashMap;

fn simulate(mut regs : HashMap<char, u64>, instructions: &Vec<Vec<String>>) -> HashMap<char, u64> {
	let mut ip : i32= 0;
	while ip >= 0 && (ip as usize) < instructions.len() {
		let instr = &instructions[ip as usize];
		ip += 1;
		match instr[0].as_str() {
			"hlf" => *(regs.entry(instr[1].chars().next().unwrap()).or_insert(0)) /= 2,
			"tpl" => *(regs.entry(instr[1].chars().next().unwrap()).or_insert(0)) *= 3,
			"inc" => *(regs.entry(instr[1].chars().next().unwrap()).or_insert(0)) += 1,
			"jmp" => ip += instr[1].parse::<i32>().unwrap() - 1,
			"jie" => if regs.get(&instr[1].chars().next().unwrap()).unwrap_or(&0) % 2 == 0 { ip += instr[2].parse::<i32>().unwrap() - 1; },
			"jio" => if regs.get(&instr[1].chars().next().unwrap()).unwrap_or(&0) == &1 { ip += instr[2].parse::<i32>().unwrap() - 1; },
			_ => unreachable!(),
		}
	}
	return regs;
}

pub fn solve() {
	let stdin = io::stdin();
	let instructions: Vec<Vec<String>> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split(" ").map(|s| String::from(s)).collect_vec())
		.collect_vec();

	let regs_part1 = simulate(HashMap::new(), &instructions);
	println!("Part 1: {}", regs_part1.get(&'b').unwrap());

	let mut regs_part2 : HashMap<char, u64> = HashMap::new();
	regs_part2.insert('a', 1);
	let regs_part2 = simulate(regs_part2, &instructions);
	println!("Part 2: {}", regs_part2.get(&'b').unwrap());
}