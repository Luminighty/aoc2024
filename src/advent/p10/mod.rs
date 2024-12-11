use std::collections::HashMap;

use hike::{Map, Tile};

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 10: Hoof It";

#[macro_use]
mod error;
mod input;
mod hike;

type TrailHead = Vec<HashMap<usize, usize>>;

#[allow(dead_code)]
fn print_trailhead(map: &Map, trailhead: &TrailHead) {
	for y in 0..map.height() {
		for x in 0..map.width() {
			print!("{:<3}", trailhead[map.xy_index(x, y).unwrap()].len());
		}
		println!();
	}
	println!();
}

fn try_add_tile(map: &Map, trailhead: &mut TrailHead, x: i32, y: i32, from_height: Tile, value: &HashMap<usize, usize>) {
	if let Some(&height) = map.get(x, y) {
		if height >= from_height || from_height - height != 1 { return; }
		let set = &mut trailhead[map.xy_index(x, y).unwrap()];
		for (key, value) in value {
			match set.get_mut(key) {
				Some(node) => {*node += value;},
				_ => { set.insert(*key, *value); }
			}
		}
	}
}

fn step_tile(map: &Map, trailhead: &mut TrailHead, x: i32, y: i32, for_height: Tile) {
	let height = match map.get(x, y) {
		Some(height) => *height,
		None => { return; }
	};
	if height != for_height {
		return;
	}
	let value = match map.xy_index(x, y).map(|i| trailhead.get(i)).flatten() {
		None => { return; }
		Some(x) => x,
	};
	if value.len() == 0 { return; }
	let value = value.clone();
	try_add_tile(map, trailhead, x - 1, y, height, &value);
	try_add_tile(map, trailhead, x + 1, y, height, &value);
	try_add_tile(map, trailhead, x, y - 1, height, &value);
	try_add_tile(map, trailhead, x, y + 1, height, &value);
}

fn step(map: &Map, trailhead: &mut TrailHead, for_height: Tile) {
	for x in 0..map.width() {
		for y in 0..map.height() {
			step_tile(map, trailhead, x, y, for_height);
		}
	}
}

fn build_trailhead(map: &Map) -> TrailHead {
	let mut trailhead = vec![HashMap::new(); map.len()];
	for i in 0..map.len() {
		match map.get_idx(i) {
			Some(9) => { trailhead[i].insert(i, 1); }
			_ => {}
		}
	}

	for height in (0..=9).rev() {
		step(map, &mut trailhead, height);
	}
	trailhead
}

pub fn solve_a() {
	let map = unwrap!(input::get_input());
	let trailhead = build_trailhead(&map);
	let mut sum = 0;
	for i in 0..map.len() {
		match map.get_idx(i) {
			Some(0) => { sum += trailhead[i].len(); }
			_ => {}
		}
	}
	println!("Trailhead score: {sum}");
}

pub fn solve_b() {
	let map = unwrap!(input::get_input());
	let trailhead = build_trailhead(&map);
	let mut sum = 0;
	for i in 0..map.len() {
		match map.get_idx(i) {
			Some(0) => {
				for v in trailhead[i].values() {
					sum += v;
				}
			}
			_ => {}
		}
	}
	println!("Trailhead score: {sum}");

}
