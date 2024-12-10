pub type Tile = u8;

pub struct Map {
	tiles: Vec<Tile>,
	width: usize,
	height: usize,
}

impl Map {
	pub fn new(tiles: Vec<Tile>, width: usize, height: usize) -> Self {
		Self { tiles, width, height }
	}

	pub fn get(&self, x: i32, y: i32) -> Option<&Tile> {
		match self.xy_index(x, y) {
			Some(idx) => self.tiles.get(idx),
			_ => None,
		}
	}

	pub fn get_idx(&self, idx: usize) -> Option<&Tile> {
		self.tiles.get(idx)
	}

	pub fn xy_index(&self, x: i32, y: i32) -> Option<usize> {
		if x < 0 || y < 0 { 
			return None
		}
		let x = x as usize;
		let y = y as usize;
		if x >= self.width || y >= self.height { 
			return None
		}
		Some(x + y * self.width)
	}

	pub fn len(&self) -> usize {
		self.tiles.len()
	}

	pub fn width(&self) -> i32 {
		self.width as i32
	}
	pub fn height(&self) -> i32 {
		self.width as i32
	}
}