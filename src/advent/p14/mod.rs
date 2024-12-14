use std::cmp::Ordering;

use robot::{Robot, HEIGHT, WIDTH};

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 14: Restroom Redoubt";

#[macro_use]
mod error;
mod input;
mod robot;
mod render;

#[allow(dead_code)]
fn robots_on_tile(robots: &Vec<Robot>, x: i32, y: i32) -> usize {
	robots.iter()
		.filter(|robot| robot.x == x && robot.y == y)
		.count()
}

#[allow(dead_code)]
fn print_map(robots: &Vec<Robot>) {
	for y in 0..HEIGHT {
		for x in 0..WIDTH {
			match robots_on_tile(robots, x, y) {
				0 => print!(" "),
				_ => print!("#"),
			}
		}
		println!();
	}
	println!();
}

fn get_quadrants(robots: &Vec<Robot>) -> (usize, usize, usize, usize) {
	let mut tl = 0;
	let mut tr = 0;
	let mut bl = 0;
	let mut br = 0;

	let cx = WIDTH / 2;
	let cy = HEIGHT / 2;
	for robot in robots {
		match (robot.x.cmp(&cx), robot.y.cmp(&cy)) {
		(Ordering::Equal, _) => {},
		(_, Ordering::Equal) => {},
    (Ordering::Less, Ordering::Less) => { tl += 1; },
    (Ordering::Less, Ordering::Greater) => { bl += 1; },
    (Ordering::Greater, Ordering::Less) => { tr += 1; },
    (Ordering::Greater, Ordering::Greater) => { br += 1; },
		}
	}

	(tl, tr, bl, br)
}

pub fn solve_a() {
	let mut robots = unwrap!(input::get_input());
	for robot in robots.iter_mut() {
		for _ in 0..100 {
			robot.step();
		}
	}
	let (tl, tr, bl, br) = get_quadrants(&robots);
	println!("Safety factor: {}", tl * tr * bl * br);
}

pub fn solve_b() {
	let robots = unwrap!(input::get_input());
	render::renderer(robots);
}
