pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 4: Ceres Search";

#[macro_use]
mod error;
mod input;
mod puzzle;

fn contains_xmas(puzzle: &puzzle::Puzzle, start_x: i32, start_y: i32, dx: i32, dy: i32) -> bool {
    const WORD: &'static str = "XMAS";
    let mut x = start_x;
    let mut y = start_y;
    for w in WORD.chars() {
        match puzzle.get(x, y) {
            Some(c) if w == c => {}
            _ => {
                return false;
            }
        };
        x += dx;
        y += dy;
    }
    true
}

fn find_solutions(puzzle: &puzzle::Puzzle) -> u32 {
    let mut c = 0;
    let dirs = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    for i in 0..puzzle.len() {
        let (sx, sy) = puzzle.index_to_xy(i);
        for (dx, dy) in &dirs {
            if contains_xmas(puzzle, sx, sy, *dx, *dy) {
                c += 1;
            }
        }
    }
    c
}

pub fn solve_a() {
    let lines = unwrap!(input::get_input());
    let puzzle = puzzle::Puzzle::new(lines);
    println!("Solutions {}", find_solutions(&puzzle));
}

fn find_x_mas(puzzle: &puzzle::Puzzle) -> u32 {
    let mut c = 0;
    for i in 0..puzzle.len() {
        let (sx, sy) = puzzle.index_to_xy(i);
        if is_x_mas(puzzle, sx, sy) {
            c += 1;
        }
    }
    c
}

fn is_x_mas(puzzle: &puzzle::Puzzle, x: i32, y: i32) -> bool {
    match puzzle.get(x, y) {
        Some('A') => {}
        _ => return false,
    }

    match (puzzle.get(x - 1, y - 1), puzzle.get(x + 1, y + 1)) {
        (Some('M'), Some('S')) => {}
        (Some('S'), Some('M')) => {}
        _ => return false,
    }

    match (puzzle.get(x - 1, y + 1), puzzle.get(x + 1, y - 1)) {
        (Some('M'), Some('S')) => {}
        (Some('S'), Some('M')) => {}
        _ => return false,
    }
    true
}

pub fn solve_b() {
    let lines = unwrap!(input::get_input());
    let puzzle = puzzle::Puzzle::new(lines);
    println!("X-MAS Solutions {}", find_x_mas(&puzzle));
}
