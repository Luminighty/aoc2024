use std::collections::{HashMap, HashSet};

use antenna::Antenna;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 8: Resonant Collinearity";

#[macro_use]
mod error;
mod input;
mod antenna;

fn group_antennas(antennas: Vec<Antenna>) -> HashMap<char, Vec<Antenna>> {
	let mut groups = HashMap::new();
	for antenna in antennas {
		match groups.get_mut(&antenna.freq) {
			None => { groups.insert(antenna.freq, vec![antenna]); }
			Some(v) => { v.push(antenna); }
		};
	}
	groups
}

fn get_antinodes(antennas: &Vec<Antenna>, antinodes: &mut HashSet<(i32, i32)>) {
	for left in 0..(antennas.len() - 1) {
		for right in (left + 1)..antennas.len() {
			antinodes.insert(antennas[left].antinode(&antennas[right]));
			antinodes.insert(antennas[right].antinode(&antennas[left]));
		}
	}
}

fn in_world(x: i32, y: i32, world: &(i32, i32)) -> bool {
	x >= 0 && x < world.0 && y >= 0 && y < world.1
}

fn add_harmonics(antinodes: &mut HashSet<(i32, i32)>, start: (i32, i32), delta: (i32, i32), world: &(i32, i32)) {
	let mut x = start.0 + delta.0;
	let mut y = start.1 + delta.1;
	while in_world(x, y, world) {
		antinodes.insert((x, y));
		x += delta.0;
		y += delta.1;
	}
}

fn get_harmonics(antennas: &Vec<Antenna>, antinodes: &mut HashSet<(i32, i32)>, world: &(i32, i32)) {
	for left in 0..(antennas.len() - 1) {
		for right in (left + 1)..antennas.len() {
			let delta = antennas[right].delta(&antennas[left]);
			add_harmonics(antinodes, antennas[right].pos(), delta, world);
			let delta = antennas[left].delta(&antennas[right]);
			add_harmonics(antinodes, antennas[left].pos(), delta, world);
		}
	}
}

pub fn solve_a() {
	let (width, height, antennas) = unwrap!(input::get_input());
	let world = (width, height);
	let groups = group_antennas(antennas);
	let mut antinodes = HashSet::new();
	for group in groups.values() {
		get_antinodes(group, &mut antinodes);
	}
	let antinodes = antinodes.iter().filter(|(x, y)| in_world(*x, *y, &world)).count();
	println!("Antinodes: {antinodes}");
}

pub fn solve_b() {
	let (width, height, antennas) = unwrap!(input::get_input());
	let world = (width, height);
	let groups = group_antennas(antennas);
	let mut antinodes = HashSet::new();
	for group in groups.values() {
		get_harmonics(group, &mut antinodes, &world);
	}
	println!("Antinodes: {}", antinodes.len());
}
