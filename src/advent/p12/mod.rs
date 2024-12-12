use std::{collections::{HashMap, HashSet}, i32};

use garden::Garden;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 12: Garden Groups";

#[macro_use]
mod error;
mod input;
mod garden;

fn get_perimeter_count(area: &Area, x: i32, y: i32) -> usize {
	(if area.contains(&(x + 1, y)) { 0 } else { 1 }) +
	(if area.contains(&(x - 1, y)) { 0 } else { 1 }) +
	(if area.contains(&(x, y + 1)) { 0 } else { 1 }) +
	(if area.contains(&(x, y - 1)) { 0 } else { 1 })
}

type Area = HashSet<(i32, i32)>;

fn get_area_from(garden: &Garden, x: i32, y: i32, plot: char) -> Area {
	let mut queue = vec![(x, y)];
	let mut visited = HashSet::new();
	visited.insert((x, y));
	while let Some((x, y)) = queue.pop() {
		
		let new = (x + 1, y);
		if !visited.contains(&new) && garden.is_plot(new.0, new.1, plot) {
			visited.insert(new);
			queue.push(new);
		}
		let new = (x - 1, y);
		if !visited.contains(&new) && garden.is_plot(new.0, new.1, plot) {
			visited.insert(new);
			queue.push(new);
		}
		let new = (x, y + 1);
		if !visited.contains(&new) && garden.is_plot(new.0, new.1, plot) {
			visited.insert(new);
			queue.push(new);
		}
		let new = (x, y - 1);
		if !visited.contains(&new) && garden.is_plot(new.0, new.1, plot) {
			visited.insert(new);
			queue.push(new);
		}
	}
	visited
}

fn get_areas(garden: &Garden) -> Vec<Area> {
	let mut areas: Vec<Area> = Vec::new();
	for i in 0..garden.len() {
		let (x, y) = garden.index_xy(i);
		if areas.iter().find(|area| area.contains(&(x, y))).is_some() {
			continue;
		}
		let plot = garden.get(x, y).unwrap();
		areas.push(get_area_from(garden, x, y, *plot));
	}
	areas
}

pub fn solve_a() {
	let garden = unwrap!(input::get_input());
	let areas = get_areas(&garden);
	let mut cost = 0;
	for area in &areas {
		let mut perimeter = 0;
		for (x, y) in area {
			perimeter += get_perimeter_count(area, *x, *y);
		}
		let area = area.len();
		cost += perimeter * area;
	}
	println!("Cost {cost}");
}

fn get_start_position(area: &Area) -> (i32, i32) {
	let mut mx = i32::MAX;
	let mut my = i32::MAX;
	for (x, y) in area {
		if mx > *x { mx = *x; }
		if my > *y { my = *y; }
	}
	(mx, my)
}

fn get_end_position(area: &Area) -> (i32, i32) {
	let mut mx = 0;
	let mut my = 0;
	for (x, y) in area {
		if mx < *x { mx = *x; }
		if my < *y { my = *y; }
	}
	(mx + 2, my + 2)
}

fn get_sides_on_column(area: &Area, sx: i32, sy: i32, ex: i32, ey: i32, dx: i32, dy: i32) -> HashSet<(i32, bool)> {
	let mut set = HashSet::new();
	let mut x = sx;
	let mut y = sy;
	let mut inside = false;
	let mut i = 0;
	while x <= ex && y <= ey {
		let contains = area.contains(&(x, y));
		if inside != contains {
			set.insert((i, contains));
			inside = contains;
		}
		x += dx;
		y += dy;
		i += 1;
	}
	set
}

fn get_area_sides(area: &Area, dx: i32, dy: i32) -> usize {
	let (mut sx, mut sy) = get_start_position(area);
	let (ex, ey) = get_end_position(area);
	let mut sides = 0;
	let mut current = HashSet::new();
	while ex >= sx && ey >= sy {
		let new = get_sides_on_column(area, sx, sy, ex, ey, dy, dx); // Flipped dx and dy
		//println!("{new:?}");
		let delta = new.difference(&current);
		//println!("{new:?} => {delta:?}");
		sides += delta.count();
		current = new;
		sx += dx;
		sy += dy;
	}
	sides
}

pub fn solve_b() {

	let garden = unwrap!(input::get_input());
	let areas = get_areas(&garden);
	let mut cost = 0;
	for area in &areas {
		//println!("Area {area:?}");
		let h_sides = get_area_sides(area, 1, 0);
		let v_sides = get_area_sides(area, 0, 1);
		let (x, y) =  area.iter().next().unwrap();
		//println!("{:?} = {} * {}", garden.get(*x, *y), h_sides + v_sides, area.len());
		cost += (h_sides + v_sides) * area.len();
	}
	println!("Cost {cost}");
}
