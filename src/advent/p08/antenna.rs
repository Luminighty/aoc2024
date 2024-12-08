#[derive(Debug)]
pub struct Antenna {
	pub freq: char,
	x: i32,
	y: i32,
}

impl Antenna {
	pub fn new(freq: char, x: i32, y: i32) -> Self {
		Self { freq, x, y }
	}

	pub fn antinode(&self, other: &Self) -> (i32, i32) {
		let (dx, dy) = self.delta(other);
		(other.x + dx, other.y + dy)
	}

	pub fn delta(&self, other: &Self) -> (i32, i32) {
		(other.x - self.x, other.y - self.y)
	}

	pub fn pos(&self) -> (i32, i32) {
		(self.x, self.y)
	}
}