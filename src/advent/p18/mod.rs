use std::collections::{HashSet, VecDeque};

use memory::Memory;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 18: RAM Run";

#[macro_use]
mod error;
mod input;
mod memory;

const SIZE: i32 = 70;

fn add_to_queue(memory: &Memory, visited: &HashSet<(i32, i32)>, queue: &mut VecDeque<(i32, i32, usize)>, x: i32, y: i32, d: usize) {
	if x < 0 || y < 0 || x > SIZE || y > SIZE { return; }
	if memory.is_wall(x, y) { return; }
	let item = (x, y, d);
	if visited.contains(&(x, y)) { return; }

	let mut position = None;
	for (i, other) in queue.iter().enumerate() {
		if other.0 == x && other.1 == y { return; }
		if other.2 >= d { position = Some(i); }
	}

	if let Some(position) = position {
		queue.insert(position, item);
	} else {
		queue.push_back(item);
	}
}

fn find_path(memory: &Memory, sx: i32, sy: i32) -> Option<usize> {
	let mut queue = VecDeque::new();
	queue.push_front((sx, sy, 0));
	let mut visited = HashSet::new();

	while let Some((x, y, d)) = queue.pop_front() {
		if x == SIZE && y == SIZE { return Some(d) }

		add_to_queue(memory, &visited, &mut queue, x + 1, y, d + 1);
		add_to_queue(memory, &visited, &mut queue, x - 1, y, d + 1);
		add_to_queue(memory, &visited, &mut queue, x, y + 1, d + 1);
		add_to_queue(memory, &visited, &mut queue, x, y - 1, d + 1);

		visited.insert((x, y));
	}

	None
}

fn add_to_queue_2(memory: &Memory, queue: &mut Vec<(i32, i32)>, visited: &HashSet<(i32, i32)>, x: i32, y: i32) {
	if x < 0 || y < 0 || x > SIZE || y > SIZE { return; }
	if memory.is_wall(x, y) { return; }
	if visited.contains(&(x, y)) { return; }
	if queue.contains(&(x, y)) { return; }
	queue.push((x, y));
}

fn has_path(memory: &Memory) -> bool {
	let mut queue = vec![(0, 0)];
	let mut visited = HashSet::new();
	
	while let Some((x, y)) = queue.pop() {
		if x == SIZE && y == SIZE { return true }
	
		add_to_queue_2(memory, &mut queue, &visited, x + 1, y);
		add_to_queue_2(memory, &mut queue, &visited, x - 1, y);
		add_to_queue_2(memory, &mut queue, &visited, x, y + 1);
		add_to_queue_2(memory, &mut queue, &visited, x, y - 1);

		visited.insert((x, y));
	}
	false
}

#[allow(dead_code)]
fn print_memory(memory: &Memory) {
	for y in 0..=SIZE {
		for x in 0..=SIZE {
			print!("{}", if memory.is_wall(x, y) { '#' } else { '.' })
		}
		println!()
	}
}


pub fn solve_a() {
	let bytes = unwrap!(input::get_input());
	let mut mem = memory::Memory::new(bytes);
	mem.set_time(1024);
	//print_memory(&mem);
	println!("Shortest path {:?}", find_path(&mem, 0, 0));
}


fn find_latest_second(memory: &mut Memory, from: usize, to: usize) -> usize {
	let time = (from + to) / 2;
	memory.set_time(time);
	let path = has_path(memory);
	println!("find_latest_second({from}, {to}) = {:?}", path);
	if to - from <= 1 {
		return if path { time } else { time - 1 }
	}
	if has_path(memory) {
		find_latest_second(memory, time, to)
	} else {
		find_latest_second(memory, from, time)
	}
}

pub fn solve_b() {
	let bytes = unwrap!(input::get_input());
	let mut mem = memory::Memory::new(bytes);
	let to = mem.len();
	let second = find_latest_second(&mut mem, 0, to);
	if let Some((x, y)) = mem.get_byte(second) {
		println!("Latest byte pos: {x},{y} @ {second}s")
	} else {
		println!("Not found.")
	}
}
