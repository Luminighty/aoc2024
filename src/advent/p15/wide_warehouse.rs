use std::collections::VecDeque;

use super::warehouse::{Action, Robot, Tile, Warehouse};

#[derive(Debug, Clone, Copy)]
pub enum WTile {
	FLOOR, WALL,
	LBOX, RBOX
}

pub struct WideWarehouse {
	tiles: Vec<WTile>,
	width: usize,
	height: usize,
}

impl WideWarehouse {
	#[allow(dead_code)]
	pub fn print(&self, robot: &Robot) {
		for y in 0..self.height {
			for x in 0..self.width {
				if x == robot.x as usize && y == robot.y as usize {
					print!("@");
					continue;
				}
				match self.tiles[x + y * self.width] {
					WTile::LBOX => print!("["),
					WTile::RBOX => print!("]"),
					WTile::FLOOR => print!("."),
					WTile::WALL => print!("#"),
				}
			}
			println!();
		}
	}

	pub fn get(&self, x: i32, y: i32) -> Option<&WTile> {
		if let Some(i) = self.xy(x, y) {
			self.tiles.get(i)
		} else {
			None
		}
	}

	pub fn set(&mut self, x: i32, y: i32, tile: WTile) {
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
					WTile::LBOX => {
						let y_val = y; //.min(self.height - y);
						let x_val = x; //.min(self.width - x - 1);
						sum += x_val + y_val * 100;
					},
					_ => {},
				}
			}
		}
		sum
	}
}

impl From<Warehouse> for WideWarehouse {
	fn from(warehouse: Warehouse) -> Self {
		let mut tiles = Vec::with_capacity(warehouse.width * warehouse.height * 2);
		for y in 0..warehouse.height {
			for x in 0..warehouse.width {
				match warehouse.get(x as i32, y as i32) {
					Some(Tile::BOX) => {
						tiles.push(WTile::LBOX);
						tiles.push(WTile::RBOX);
					},
					Some(Tile::FLOOR) => {
						tiles.push(WTile::FLOOR);
						tiles.push(WTile::FLOOR);
					},
					Some(Tile::WALL) => {
						tiles.push(WTile::WALL);
						tiles.push(WTile::WALL);
					}
					_ => {},
				}
			}
		}
		WideWarehouse { tiles, height: warehouse.height, width: warehouse.width * 2 }
	}
}

fn try_add_box(boxes: &Vec<(i32, i32)>, queue: &mut VecDeque<(i32, i32)>, x: i32, y: i32) {
	if boxes.contains(&(x, y)) { return; }
	if queue.contains(&(x, y)) { return; }
	queue.push_back((x, y));
}

fn try_push_vert(warehouse: &mut WideWarehouse, x: i32, y: i32, dy: i32) -> Option<Vec<(i32, i32)>> {
	let mut boxes = Vec::new();
	let mut queue = VecDeque::new();
	queue.push_back((x, y));
	match warehouse.get(x, y) {
		Some(WTile::LBOX) => { queue.push_back((x + 1, y)); },
		Some(WTile::RBOX) => { queue.push_back((x - 1, y)); },
		_ => {},
	}

	while let Some((x, y)) = queue.pop_front() {
		match warehouse.get(x, y + dy) {
			Some(WTile::LBOX) => {
				try_add_box(&boxes, &mut queue, x, y + dy);
				try_add_box(&boxes, &mut queue, x + 1, y + dy);
			},
			Some(WTile::RBOX) => {
				try_add_box(&boxes, &mut queue, x, y + dy);
				try_add_box(&boxes, &mut queue, x - 1, y + dy);
			},
			Some(WTile::FLOOR) => {},
			_ => { return None; }
		}
		boxes.push((x, y));
	}
	Some(boxes)
}

fn try_push_horiz(warehouse: &mut WideWarehouse, x: i32, y: i32, dx: i32) -> Option<(i32, i32)> {
	let x = x + dx;
	match warehouse.get(x, y) {
		Some(WTile::FLOOR) => Some((x, y)),
		Some(WTile::LBOX) | Some(WTile::RBOX) => try_push_horiz(warehouse, x, y, dx),
		_ => None,
	}
}

pub fn step(warehouse: &mut WideWarehouse, robot: &mut Robot, action: Action) {
	let (dx, dy) = action.delta();
	let new_x = robot.x + dx;
	let new_y = robot.y + dy;
	match warehouse.get(new_x, new_y) {
		Some(WTile::FLOOR) => {
			robot.x = new_x;
			robot.y = new_y;
		},
		Some(WTile::LBOX) | Some(WTile::RBOX) if dx != 0 => {
			if let Some((floor_x, floor_y)) = try_push_horiz(warehouse, new_x, new_y, dx) {
				let mut x = floor_x;
				while x != robot.x {
					warehouse.set(x, floor_y, warehouse.get(x - dx, floor_y).unwrap().clone());
					x -= dx;
				}
				robot.x = new_x;
				robot.y = new_y;
			}
		},
		Some(WTile::LBOX) | Some(WTile::RBOX) if dy != 0 => {
			if let Some(boxes) = try_push_vert(warehouse, new_x, new_y, dy) {
				for (bx, by) in boxes.into_iter().rev() {
					warehouse.set(bx, by + dy, warehouse.get(bx, by).unwrap().clone());
					warehouse.set(bx, by, WTile::FLOOR);
				}
				robot.x = new_x;
				robot.y = new_y;
			}
		}
		_ => {},
	}
}