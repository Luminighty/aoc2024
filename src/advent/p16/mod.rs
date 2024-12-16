pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 16: Reindeer Maze";

#[macro_use]
mod error;
mod input;
mod maze;
mod state;

pub fn solve_a() {
    let (maze, start, end) = unwrap!(input::get_input());
    let facing = maze::Dir::Right;

    if let Some((len, _visited)) = state::find_shortest_path(maze, start, end, facing) {
        println!("Score: {len:?}");
    } else {
        println!("Path not found!");
    }
}

pub fn solve_b() {
    let (maze, start, end) = unwrap!(input::get_input());
    let facing = maze::Dir::Right;

    if let Some((len, visited)) = state::find_shortest_path(maze, start, end, facing) {
        let seats = state::find_places_to_sit(&visited, end, len);
        println!("Seats: {}", seats.len());
    } else {
        println!("Path not found!");
    }
}
