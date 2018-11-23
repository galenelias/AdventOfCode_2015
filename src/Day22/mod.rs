use std::io::{self, BufRead};
use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq)]
enum EffectType {
	MagicMissle,
	Drain,
	Shield,
	Poison,
	Recharge,
}

#[derive(Clone, Debug)]
struct Effect {
	effect_type: EffectType,
	cost: u32,
	duration: u32,
	damage: i32,
	alt_value: i32,
}

#[derive(Clone, Debug)]
struct AppliedEffect {
	effect: &'static Effect,
	duration: u32,
}

static SPELLS : &'static [Effect] = &[
	Effect { effect_type: EffectType::MagicMissle, cost: 53, duration: 0, damage: 4, alt_value: 0},
	Effect { effect_type: EffectType::Drain, cost: 73, duration: 0, damage: 2, alt_value: 2},
	Effect { effect_type: EffectType::Shield, cost: 113, duration: 6, damage: 0, alt_value: 7},
	Effect { effect_type: EffectType::Poison , cost: 173, duration: 6, damage: 3, alt_value: 0},
	Effect { effect_type: EffectType::Recharge, cost: 229, duration: 5, damage: 0, alt_value: 101},
];

fn optimize_battle(mana_spent: u32, mut mana: u32, mut player_hp: i32, mut boss_hp: i32, boss_dmg: i32, mut effects: Vec<AppliedEffect>, hard_mode: bool, best: &Option<u32>) -> Option<u32> {
	if hard_mode {
		player_hp -= 1;
		if player_hp == 0 {
			return None;
		}
	}

	// Apply player's turn effects
	for effect in &mut effects {
		if effect.duration > 0 {
			match effect.effect.effect_type {
				EffectType::Shield => (),
				EffectType::Poison => boss_hp -= 3,
				EffectType::Recharge => mana += 101,
				_ => unreachable!(),
			}
			effect.duration -= 1;
		}
	}

	if boss_hp <= 0 {
		return Some(mana_spent);
	}

	// Try out various spells
	let mut best_outcome: Option<u32> = *best;
	for spell in SPELLS.iter()
		.filter(|spell| mana >= spell.cost && !effects.iter().any(|e| e.effect.effect_type == spell.effect_type && e.duration > 0) ) {

		let new_mana_spent = mana_spent + spell.cost;

		if best.is_some() && new_mana_spent >= best.unwrap() {
			continue;
		}

		let mut new_effects = effects.iter().filter(|e| e.duration > 0 ).cloned().collect_vec();
		let mut new_player_hp = player_hp;
		let mut new_boss_hp = boss_hp;
		let mut new_mana = mana - spell.cost;

		// Apply damage/life gain
		match spell.effect_type {
			EffectType::MagicMissle => new_boss_hp -= spell.damage,
			EffectType::Drain => {new_boss_hp -= spell.damage; new_player_hp += spell.alt_value},
			EffectType::Shield => new_effects.push(AppliedEffect { effect: spell, duration: spell.duration}),
			EffectType::Poison => new_effects.push(AppliedEffect { effect: spell, duration: spell.duration}),
			EffectType::Recharge => new_effects.push(AppliedEffect { effect: spell, duration: spell.duration}),
		};

		let armor = new_effects.iter().find(|e| e.effect.effect_type == EffectType::Shield && e.duration > 0).map_or(0, |e| e.effect.alt_value);

		// Boss turn, apply effects
		for effect in &mut new_effects {
			if effect.duration > 0 {
				match effect.effect.effect_type {
					EffectType::Shield => (),
					EffectType::Poison => new_boss_hp -= 3,
					EffectType::Recharge => new_mana += 101,
					_ => unreachable!(),
				}
				effect.duration -= 1;
			}
		}

		if new_boss_hp <= 0 {
			if best_outcome.is_none() || new_mana_spent < best_outcome.unwrap() {
				best_outcome = Some(new_mana_spent);
			}
			continue;
		}

		new_player_hp -= std::cmp::max(1, boss_dmg - armor);
		if new_player_hp <= 0 {
			continue;
		}

		// Game goes on, simulate future turns, checking for the best outcome
		let outcome = optimize_battle(
			new_mana_spent,
			new_mana,
			new_player_hp,
			new_boss_hp,
			boss_dmg,
			new_effects,
			hard_mode,
			&best_outcome);

		if outcome.is_some() && (best_outcome.is_none() || outcome.unwrap() < best_outcome.unwrap()) {
			best_outcome = outcome;
		}
	}

	return best_outcome;
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<i32> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split(": ").skip(1).next().unwrap().parse::<i32>().unwrap())
		.collect_vec();

	let boss_hp = lines[0];
	let boss_dmg = lines[1];
	let player_hp = 50;
	let player_mana = 500;

	let part1 = optimize_battle(0 /*mana_spent*/, player_mana, player_hp, boss_hp, boss_dmg, vec![], false /*hard*/, &None);
	println!("Part 1: {}", part1.unwrap_or(0));

	let part2 = optimize_battle(0 /*mana_spent*/, player_mana, player_hp, boss_hp, boss_dmg, vec![], true /*hard*/, &None);
	println!("Part 2: {}", part2.unwrap_or(0));
}