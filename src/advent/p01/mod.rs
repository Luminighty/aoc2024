use input::LocationId;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 1: Historian Hysteria";

#[macro_use]
mod error;
mod input;


pub fn solve_a() {
	let pairs = unwrap!(input::get_input());
	let mut left: Vec<LocationId> = pairs.iter().map(|pair| pair.0).collect();
	let mut right: Vec<LocationId> = pairs.iter().map(|pair| pair.1).collect();
	left.sort();
	right.sort();
	let distance = left.iter().zip(right).map(|(left, right)| (left - right).abs()).reduce(|acc, d| acc + d);
	println!("distance: {:?}", distance);
}

fn similarity(right: &Vec<LocationId>, location: LocationId) -> LocationId {
	right.iter()
		.map(|&right| if right == location { location } else { 0 })
		.reduce(|acc, v| acc + v)
		.unwrap_or(0)
}

pub fn solve_b() {
	let pairs = unwrap!(input::get_input());
	let left: Vec<LocationId> = pairs.iter().map(|pair| pair.0).collect();
	let right: Vec<LocationId> = pairs.iter().map(|pair| pair.1).collect();

	let score = left
		.iter()
		.map(|left| similarity(&right, *left))
		.reduce(|acc, v| acc + v);
	println!("Similarity {:?}", score)
}
