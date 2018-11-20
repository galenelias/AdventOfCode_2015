use std::io::{self, BufRead};
use itertools::Itertools;

struct Item {
	_name: String,
	cost: i32,
	damage: i32,
	armor: i32,
}

#[derive(Copy, Debug)]
struct Character {
	hitpoints: i32,
	damage: i32,
	armor: i32,
	money: i32,
}

impl Character {
	fn equip(&mut self, item: &Item) {
		self.damage += item.damage;
		self.armor += item.armor;
		self.money -= item.cost;
	}
}

impl Clone for Character {
    fn clone(&self) -> Character { *self }
}

fn fight(player: &Character, boss: &Character) -> bool {
	let player_damage = std::cmp::max(1, player.damage - boss.armor);
	let boss_damage = std::cmp::max(1, boss.damage - player.armor);
	let mut player_hp = player.hitpoints;
	let mut boss_hp = boss.hitpoints;
	loop {
		boss_hp -= player_damage;
		if boss_hp <= 0 {
			return true;
		}

		player_hp -= boss_damage;
		if player_hp <= 0 {
			return false;
		}
	}
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<i32> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split(": ").skip(1).next().unwrap().parse::<i32>().unwrap())
		.collect_vec();

	let weapons = vec![
		Item { _name: String::from("Dagger"),    cost:  8, damage: 4, armor: 0},
		Item { _name: String::from("Shortsord"), cost: 10, damage: 5, armor: 0},
		Item { _name: String::from("Warhammer"), cost: 25, damage: 6, armor: 0},
		Item { _name: String::from("Longsword"), cost: 40, damage: 7, armor: 0},
		Item { _name: String::from("Greataxe"),  cost: 74, damage: 8, armor: 0},
	];

	let armors = vec![
		Item { _name: String::from("None"),       cost:  0, damage: 0, armor: 0},
		Item { _name: String::from("Leather"),    cost: 13, damage: 0, armor: 1},
		Item { _name: String::from("Chainmail"),  cost: 31, damage: 0, armor: 2},
		Item { _name: String::from("Splintmail"), cost: 53, damage: 0, armor: 3},
		Item { _name: String::from("Bandedmail"), cost: 75, damage: 0, armor: 4},
		Item { _name: String::from("Platemail"),  cost:102, damage: 0, armor: 5},
	];

	let rings = vec![
		Item { _name: String::from("NilRing1"), cost:   0, damage: 0, armor: 0},
		Item { _name: String::from("NilRing2"), cost:   0, damage: 0, armor: 0},
		Item { _name: String::from("Damage1"),  cost:  25, damage: 1, armor: 0},
		Item { _name: String::from("Damage2"),  cost:  50, damage: 2, armor: 0},
		Item { _name: String::from("Damage3"),  cost: 100, damage: 3, armor: 0},
		Item { _name: String::from("Defense1"), cost:  20, damage: 0, armor: 1},
		Item { _name: String::from("Defense1"), cost:  40, damage: 0, armor: 2},
		Item { _name: String::from("Defense1"), cost:  80, damage: 0, armor: 3},
	];

	let boss = Character {
		hitpoints: lines[0],
		damage: lines[1],
		armor: lines[2],
		money: 0,
	};

	let player = Character {
		hitpoints: 100,
		damage: 0,
		armor: 0,
		money: 0,
	};

	let mut best_cost = None;
	let mut worst_cost = None;

	// Iterate through all possible player loadouts
	for weapon in &weapons {
		for armor in &armors {
			for iring1 in 0..rings.len() {
				for iring2 in iring1+1..rings.len() {
					let mut equipped_player = player.clone();
					equipped_player.equip(weapon);
					equipped_player.equip(armor);
					equipped_player.equip(&rings[iring1]);
					equipped_player.equip(&rings[iring2]);

					if fight(&equipped_player, &boss) && (best_cost.is_none() || -equipped_player.money < best_cost.unwrap()) {
						best_cost = Some(-equipped_player.money);
					}
					if !fight(&equipped_player, &boss) && (worst_cost.is_none() || -equipped_player.money > worst_cost.unwrap()) {
						worst_cost = Some(-equipped_player.money);
					}
				}
			}
		}
	}

	println!("Part 1: {}", best_cost.unwrap());
	println!("Part 2: {}", worst_cost.unwrap());
}