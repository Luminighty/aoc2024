#[derive(Debug)]
pub enum Tile {
	WALL, FLOOR, BOX,
}

#[derive(Debug)]
pub struct Warehouse {
	tiles: Vec<Tile>,
	pub height: usize,
	pub width: usize,
}

impl Warehouse {
	pub fn new(tiles: Vec<Tile>, height: usize, width: usize) -> Self {
		Self { tiles, height, width }
	}

	#[allow(dead_code)]
	pub fn print(&self, robot: &Robot) {
		for y in 0..self.height {
			for x in 0..self.width {
				if x == robot.x as usize && y == robot.y as usize {
					print!("@");
					continue;
				}
				match self.tiles[x + y * self.width] {
					Tile::BOX => print!("O"),
					Tile::FLOOR => print!("."),
					Tile::WALL => print!("#"),
				}
			}
			println!();
		}
	}

	pub fn get(&self, x: i32, y: i32) -> Option<&Tile> {
		if let Some(i) = self.xy(x, y) {
			self.tiles.get(i)
		} else {
			None
		}
	}

	pub fn set(&mut self, x: i32, y: i32, tile: Tile) {
		if let Some(i) = self.xy(x, y) {
			self.tiles[i] = tile;
		}
	}

	fn xy(&self, x: i32, y: i32) -> Option<usize> {
		if x < 0 || y < 0 { return None; }
		let x = x as usize;
		let y = y as usize;
		if x >= self.width || y >= self.height { return None; }
		Some(x + y * self.width)
	}

	pub fn gps(&self) -> usize {
		let mut sum = 0;
		for y in 0..self.height {
			for x in 0..self.width {
				match self.tiles[x + y * self.width] {
					Tile::BOX => { sum += y * 100 + x},
					_ => {},
				}
			}
		}
		sum
	}
}

#[derive(Debug)]
pub enum Action {
	Up, Down, Left, Right
}

impl Action {
	pub fn delta(&self) -> (i32, i32) {
		match self {
			Action::Up => (0, -1),
			Action::Left => (-1, 0),
			Action::Right => (1, 0),
			Action::Down => (0, 1),
		}
	}
}

impl std::fmt::Display for Action {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Action::Up => '^',
			Action::Right => '<',
			Action::Left => '>',
			Action::Down => 'v',
		})
	}
}

#[derive(Debug)]
pub struct Robot {
	pub x: i32,
	pub y: i32,
}

impl Robot {
	pub fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}
}

fn try_push(warehouse: &mut Warehouse, x: i32, y: i32, dx: i32, dy: i32) -> Option<(i32, i32)> {
	let x = x + dx;
	let y = y + dy;
	match warehouse.get(x, y) {
		Some(Tile::FLOOR) => Some((x, y)),
		Some(Tile::BOX) => try_push(warehouse, x, y, dx, dy),
		_ => None,
	}
}

pub fn step(warehouse: &mut Warehouse, robot: &mut Robot, action: Action) {
	let (dx, dy) = action.delta();
	let new_x = robot.x + dx;
	let new_y = robot.y + dy;
	match warehouse.get(new_x, new_y) {
		Some(Tile::FLOOR) => {
			robot.x = new_x;
			robot.y = new_y;
		},
		Some(Tile::BOX) => {
			if let Some((floor_x, floor_y)) = try_push(warehouse, new_x, new_y, dx, dy) {
				warehouse.set(floor_x, floor_y, Tile::BOX);
				warehouse.set(new_x, new_y, Tile::FLOOR);
				robot.x = new_x;
				robot.y = new_y;
			}
		}
		_ => {},
	}
}