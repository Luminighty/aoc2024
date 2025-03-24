use std::collections::HashMap;

use towel::Pattern;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 19: Linen Layout";

#[macro_use]
mod error;
mod input;
mod towel;

fn is_possible_pattern(desired: &Pattern, patterns: &Vec<Pattern>, offset: usize) -> bool {
	if offset == desired.len() { return true; }
	for pattern in patterns {
		if !desired.matches(pattern, offset) { continue; }
		if is_possible_pattern(desired, patterns, offset + pattern.len()) {
			return true;
		}
	}
	false
}

pub fn solve_a() {
	let (patterns, desired) = unwrap!(input::get_input());
	let mut c = 0;
	for desired in &desired {
		let possible = is_possible_pattern(desired, &patterns, 0);
		if possible {
			c += 1;
		}
	}
	println!("Possible designs: {c}");
}

fn add_possible_patterns(patterns: &mut LenMap, length: usize) {
	let mut new_patterns = HashMap::new();

	for i in 1..length {
		for (left, left_c) in patterns.get_len(i).iter() {
			for (right, right_c) in patterns.get_len(length - i).iter() {
				let pattern = left.concat(right);
				
				if let Some(p) = new_patterns.get_mut(&pattern) {
					*p += left_c * right_c;
				} else {
					new_patterns.insert(pattern, left_c * right_c);
				}
				let pattern = right.concat(left);
				if let Some(p) = new_patterns.get_mut(&pattern) {
					*p += left_c * right_c;
				} else {
					new_patterns.insert(pattern, left_c * right_c);
				}
			}
		}
	}
	
	for (pattern, count) in new_patterns.into_iter() {
		patterns.add(pattern, count);
	}
}

pub fn solve_b() {
	let (patterns, desired) = unwrap!(input::get_input());
	let max = desired.iter().map(|p| p.len()).max().unwrap_or_default();
	let mut cache = LenMap::new(max, patterns);
	for l in 2..=max {
		println!("{l}/{max}");
		add_possible_patterns(&mut cache, l);
	}
	for d in desired {
		println!("{:?}", cache.get(&d));
	}
}

struct LenMap {
	maps: Vec<HashMap<Pattern, usize>>
}

impl LenMap {
	pub fn new(max_length: usize, patterns: Vec<Pattern>) -> Self {
		let mut maps = Vec::with_capacity(max_length + 1);
		for _ in 0..(max_length + 1) { maps.push(HashMap::new()); }
		for pattern in patterns {
			maps[pattern.len()].insert(pattern, 1);
		}
		Self { maps }
	}

	pub fn get_len(&self, length: usize) -> &HashMap<Pattern, usize> {
		&self.maps[length]
	}

	pub fn get(&self, pattern: &Pattern) -> Option<&usize> {
		let map = &self.maps[pattern.len()];
		map.get(pattern)
	}

	pub fn add(&mut self, pattern: Pattern, count: usize) {
		if let Some(c) = self.maps[pattern.len()].get_mut(&pattern) {
			*c += count;
		} else {
			self.maps[pattern.len()].insert(pattern, count);
		}
	}
}