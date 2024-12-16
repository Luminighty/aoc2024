use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

use super::maze::{Dir, Maze, Pos, Tile};

#[derive(Debug, Clone, Copy)]
pub struct State {
    cost: usize,
    x: i32,
    y: i32,
    facing: Dir,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.facing.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.facing == other.facing
    }
}
impl Eq for State {}

impl State {
    pub fn start(start: Pos, facing: Dir) -> Self {
        Self {
            cost: 0,
            x: start.0,
            y: start.1,
            facing,
        }
    }

    pub fn step(&self, maze: &Maze) -> Option<Self> {
        let (dx, dy) = self.facing.delta();
        let nx = self.x + dx;
        let ny = self.y + dy;
        if maze.is_wall(nx, ny) {
            return None;
        }
        Some(Self {
            cost: self.cost + 1,
            x: nx,
            y: ny,
            facing: self.facing,
        })
    }

    pub fn turn_left(&self, maze: &Maze) -> Option<Self> {
        let new = Self {
            x: self.x,
            y: self.y,
            cost: self.cost + 1000,
            facing: self.facing.turn_left(),
        };
        new.step(maze)
    }

    pub fn turn_right(&self, maze: &Maze) -> Option<Self> {
        let new = Self {
            x: self.x,
            y: self.y,
            cost: self.cost + 1000,
            facing: self.facing.turn_right(),
        };
        new.step(maze)
    }

    pub fn step_back(&self, visited: &HashSet<State>, queue: &mut Vec<State>) {
        let (dx, dy) = self.facing.delta();
        let nx = self.x - dx;
        let ny = self.y - dy;
        for v in visited {
            if v.x != nx || v.y != ny {
                continue;
            }
            if v.cost + 1 != self.cost {
                continue;
            }
            let mut new = self.clone();
            new.x = nx;
            new.y = ny;
            new.cost -= 1;
            queue.push(new);
        }
    }

    pub fn turn_left_back(&self, visited: &HashSet<State>, queue: &mut Vec<State>) {
        if self.cost < 1001 {
            return;
        }
        let mut new = self.clone();
        let (dx, dy) = self.facing.delta();
        new.x -= dx;
        new.y -= dy;
        new.facing = new.facing.turn_left();
        new.cost -= 1001;
        for v in visited {
            if v.x != new.x || v.y != new.y {
                continue;
            }
            if v.cost != new.cost {
                continue;
            }
            queue.push(new.clone());
        }
    }

    pub fn turn_right_back(&self, visited: &HashSet<State>, queue: &mut Vec<State>) {
        if self.cost < 1001 {
            return;
        }
        let mut new = self.clone();
        let (dx, dy) = self.facing.delta();
        new.x -= dx;
        new.y -= dy;
        new.facing = new.facing.turn_right();
        new.cost -= 1001;
        for v in visited {
            if v.x != new.x || v.y != new.y {
                continue;
            }
            if v.cost != new.cost {
                continue;
            }
            queue.push(new.clone());
        }
    }
}

fn is_state_better(old: &State, new: &State) -> bool {
    old.cost < new.cost
}

#[inline]
fn add_state(queue: &mut VecDeque<State>, visited: &HashSet<State>, state: Option<State>) {
    let state = if let Some(state) = state {
        state
    } else {
        return;
    };
    if visited.contains(&state) || queue.contains(&state) {
        return;
    }
    for i in 0..queue.len() {
        if !is_state_better(&queue[i], &state) {
            queue.insert(i, state);
            return;
        }
    }
    queue.push_back(state);
}

pub fn find_shortest_path(
    maze: Maze,
    start: Pos,
    end: Pos,
    facing: Dir,
) -> Option<(usize, HashSet<State>)> {
    let mut queue = VecDeque::new();
    queue.push_back(State::start(start, facing));
    let mut visited = HashSet::new();

    let mut max_cost = None;
    while let Some(state) = queue.pop_front() {
        if let Some(max_cost) = max_cost {
            if max_cost > state.cost {
                continue;
            }
        }
        visited.insert(state);
        if state.x == end.0 && state.y == end.1 && max_cost.is_none() {
            max_cost = Some(state.cost);
            continue;
        }

        add_state(&mut queue, &visited, state.step(&maze));
        add_state(&mut queue, &visited, state.turn_left(&maze));
        add_state(&mut queue, &visited, state.turn_right(&maze));
    }
    if let Some(max_cost) = max_cost {
        Some((max_cost, visited))
    } else {
        None
    }
}

pub fn find_places_to_sit(visited: &HashSet<State>, end: Pos, cost: usize) -> HashSet<Pos> {
    let mut seats = HashSet::new();
    let mut queue = Vec::new();
    for v in visited {
        if v.x != end.0 || v.y != end.1 || v.cost != cost {
            continue;
        }
        queue.push(v.clone());
    }
    while let Some(state) = queue.pop() {
        state.step_back(visited, &mut queue);
        state.turn_left_back(visited, &mut queue);
        state.turn_right_back(visited, &mut queue);
        seats.insert((state.x, state.y));
        // println!("{} {}", state.x, state.y);
    }
    seats
}

#[allow(dead_code)]
fn find_visited(visited: &HashSet<State>, x: i32, y: i32) -> Option<&State> {
    for v in visited {
        if v.x == x && v.y == y {
            return Some(v);
        }
    }
    None
}

#[allow(dead_code)]
fn print_visited(maze: &Maze, visited: &HashSet<State>) {
    for y in 0..maze.height {
        for x in 0..maze.width {
            if let Some(v) = find_visited(&visited, x as i32, y as i32) {
                print!("{}", v.facing);
                continue;
            }
            print!(
                "{}",
                match maze.get(x as i32, y as i32) {
                    Some(Tile::Wall) => "#",
                    Some(Tile::Floor) => ".",
                    _ => "?",
                }
            )
        }
        println!();
    }
}
