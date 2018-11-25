use std::io::{self, BufRead};
use serde_json::{Value};

fn sum_json(v: &Value) -> i64 {
	match v {
		Value::Number(num) => num.as_i64().unwrap(),
		Value::Array(arr) => arr.iter().map(|v| sum_json(v)).sum::<i64>(),
		Value::Object(vals) => vals.values().map(|v| sum_json(v)).sum::<i64>(),
		Value::String(_) => 0,
		_ => 0,
	}
}

fn sum_json_2(v: &Value) -> i64 {
	match v {
		Value::Number(num) => num.as_i64().unwrap(),
		Value::Array(arr) => arr.iter().map(|v| sum_json_2(v)).sum::<i64>(),
		Value::Object(obj) => if !obj.values().any(|k| k == "red") { obj.values().map(|v| sum_json_2(v)).sum::<i64>() } else { 0 },
		Value::String(_) => 0,
		_ => unreachable!("{:?}", v),
	}
}

pub fn solve() {
	let stdin = io::stdin();
	let input = stdin.lock().lines().next().unwrap().unwrap();

	let v: Value = serde_json::from_str(&input).unwrap();

	println!("Part 1: {}", sum_json(&v));
	println!("Part 2: {}", sum_json_2(&v));
}