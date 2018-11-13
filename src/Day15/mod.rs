use std::io::{self, BufRead};
use regex::Regex;
use itertools::Itertools;

struct Ingredient {
	_name: String,
	capacity: i32,
	durability: i32,
	flavor: i32,
	texture: i32,
	calories: i32,
}

fn get_score(ingredients: &[Ingredient], used_ingredients: &[i32], calorie_goal: &Option<i32>) -> i32 {
	let mut capacity: i32 = 0;
	let mut durability: i32 = 0;
	let mut flavor: i32 = 0;
	let mut texture: i32 = 0;
	let mut calories: i32 = 0;

	for (ingredient, amt) in ingredients.iter().zip(used_ingredients.iter()) {
		capacity += ingredient.capacity * amt;
		durability += ingredient.durability * amt;
		flavor += ingredient.flavor * amt;
		texture += ingredient.texture * amt;
		calories += ingredient.calories * amt;
	}

	if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
		return 0;
	} else if calorie_goal.is_some() && calories != calorie_goal.unwrap() {
		return 0;
	} else {
		return capacity * durability * flavor * texture;
	}
}

fn maximize(ingredients: &[Ingredient], used_ingredients: &[i32], teaspoons: u32, calorie_goal: &Option<i32>) -> i32 {
	if ingredients.len() == used_ingredients.len() {
		return get_score(ingredients, used_ingredients, calorie_goal);
	} else if teaspoons == 0 {
		let mut x = used_ingredients.to_vec();
		x.push(0);
		return maximize(ingredients, &x, teaspoons, calorie_goal);
	}

	let mut max_sub_score = 0;
	let mut used_ingredients = used_ingredients.to_vec();

	for t in 0..=teaspoons {
		used_ingredients.push(t as i32);
		let s = maximize(ingredients, &used_ingredients, teaspoons - t, calorie_goal);
		max_sub_score = std::cmp::max(max_sub_score, s);
		used_ingredients.pop();
	}
	return max_sub_score;
}

pub fn solve() {
	let stdin = io::stdin();
	let lines: Vec<String> = stdin.lock().lines()
		.filter_map(|line| line.ok())
		.collect_vec();

	let mut ingredients : Vec<Ingredient> = Vec::new();

	let re_state = Regex::new(r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
	for line in &lines {
		let vals = &re_state.captures(&line).unwrap();

		ingredients.push(Ingredient {
			_name: vals[1].to_string(),
			capacity: vals[2].parse::<i32>().unwrap(),
			durability: vals[3].parse::<i32>().unwrap(),
			flavor: vals[4].parse::<i32>().unwrap(),
			texture: vals[5].parse::<i32>().unwrap(),
			calories: vals[6].parse::<i32>().unwrap(),
			});
	}

	println!("Part 1: {}", maximize(&ingredients[..], &vec![], 100, &None));
	println!("Part 2: {}", maximize(&ingredients[..], &vec![], 100, &Some(500)));
}