#[derive(PartialEq, Eq)]
pub enum Tile { 
	Wall,
	Floor,
}

pub struct Track {
	width: usize,
	height: usize,
	tiles: Vec<Tile>,
}


impl Track {
	pub fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Self {
		Self { tiles, width, height }
	}

	pub fn len(&self) -> usize {
		self.width * self.height
	}

	pub fn get(&self, x: i32, y: i32) -> Option<&Tile> {
		self.xy_index(x, y).map(|i| self.tiles.get(i)).flatten()
	}

	pub fn xy_index(&self, x: i32, y: i32) -> Option<usize> {
		if x < 0 || y < 0 { return None; }
		let x = x as usize;
		let y = y as usize;
		if x >= self.width || y > self.height { return None; }
		Some(x + y * self.width)
	}

	pub fn is_wall(&self, x: i32, y: i32) -> bool {
		match  self.get(x, y) {
			Some(Tile::Floor) => false,
			_ => true,
		}
	}
}


#[derive(Debug, Clone, Copy)]
pub enum PathNode {
	Wall,
	Visited(u32),
}

pub struct Path {
	tiles: Vec<PathNode>,
	width: usize,
	height: usize,
}

fn try_add_tile(track: &Track, queue: &mut Vec<(i32, i32, u32)>, x: i32, y: i32, d: u32) {
	if track.is_wall(x, y) { return; }
	queue.push((x, y, d));
}

impl Path {
	pub fn from_track(track: &Track, sx: i32, sy: i32) -> Self {
		let mut queue = vec![(sx, sy, 0)];
		let mut tiles = vec![None; track.len()];
		for i in 0..track.len() {
			if track.tiles[i] == Tile::Wall {
				tiles[i] = Some(PathNode::Wall)
			}
		}

		while let Some((x, y, d)) = queue.pop() {
			try_add_tile(track, &mut queue, x + 1, y, d + 1);
			try_add_tile(track, &mut queue, x - 1, y, d + 1);
			try_add_tile(track, &mut queue, x, y + 1, d + 1);
			try_add_tile(track, &mut queue, x, y - 1, d + 1);
			if let Some(index) = track.xy_index(x, y) {
				tiles[index] = Some(PathNode::Visited(d))
			}
		}
		let tiles = tiles.into_iter().collect::<Option<Vec<PathNode>>>().unwrap();
		Self { tiles, height: track.height, width: track.width }
	}
}