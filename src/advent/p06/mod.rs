use std::collections::HashSet;

use guard::{Guard, Map, Tile};

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 6: Guard Gallivant";

#[macro_use]
mod error;
mod guard;
mod input;

#[allow(dead_code)]
fn print_state(map: &Map, visited: &HashSet<usize>, guard: &Guard) {
    for y in 0..map.height {
        for x in 0..map.width {
            let x = x as i32;
            let y = y as i32;
            if guard.x == x && guard.y == y {
                print!("{}", guard.dir);
                continue;
            }
            if visited.contains(&map.xy_index(x, y).unwrap()) {
                print!("X");
                continue;
            }
            print!("{}", map.get(x, y).unwrap());
        }
        println!();
    }
    println!();
}

fn get_visited_tiles(guard: &mut Guard, map: &Map) -> HashSet<usize> {
    let mut visited = HashSet::new();
    loop {
        // print_state(&map, &visited, &guard);
        visited.insert(map.xy_index(guard.x, guard.y).unwrap());
        let (new_x, new_y) = guard.next_pos();
        match map.get(new_x, new_y) {
            Some(Tile::Wall) => guard.turn_right(),
            Some(Tile::Ground) => guard.step(),
            None => break,
        };
    }

    visited
}

pub fn solve_a() {
    let (mut guard, map) = unwrap!(input::get_input());
    let visited = get_visited_tiles(&mut guard, &map);
    println!("Visited places: {}", visited.len());
}

fn is_loop(mut guard: Guard, map: &Map) -> bool {
    let mut visited = HashSet::new();
    loop {
        let key = (guard.dir, map.xy_index(guard.x, guard.y).unwrap());
        if visited.contains(&key) {
            return true;
        }
        visited.insert(key);
        let (new_x, new_y) = guard.next_pos();
        match map.get(new_x, new_y) {
            Some(Tile::Wall) => guard.turn_right(),
            Some(Tile::Ground) => guard.step(),
            None => return false,
        };
    }
}

fn try_add(visited: &mut HashSet<usize>, map: &Map, x: i32, y: i32) {
    if let Some(idx) = map.xy_index(x, y) {
        visited.insert(idx);
    }
}

pub fn solve_b() {
    let (guard, map) = unwrap!(input::get_input());
    let mut visited = get_visited_tiles(&mut guard.clone(), &map);
    visited.remove(&map.xy_index(guard.x, guard.y).unwrap());
    try_add(&mut visited, &map, guard.x + 1, guard.y);
    try_add(&mut visited, &map, guard.x - 1, guard.y);
    try_add(&mut visited, &map, guard.x, guard.y + 1);
    try_add(&mut visited, &map, guard.x, guard.y - 1);

    let mut count = 0;
    for tile in visited {
        if is_loop(guard.clone(), &map.add_wall(tile)) {
            count += 1;
        }
    }
    println!("Possible walls: {count}");
}
