pub const WIDTH: i32 = 101;
pub const HEIGHT: i32 = 103;

#[derive(Debug, Copy, Clone)]
pub struct Robot {
	pub x: i32,
	pub y: i32,
	vx: i32,
	vy: i32,
}

impl Robot {
	pub fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
		Self { x, y, vx, vy }
	}

	pub fn step(&mut self) {
		self.x = (self.x + self.vx + WIDTH) % WIDTH;
		self.y = (self.y + self.vy + HEIGHT) % HEIGHT;
	}

	pub fn back(&mut self) {
		self.x = (self.x - self.vx + WIDTH) % WIDTH;
		self.y = (self.y - self.vy + HEIGHT) % HEIGHT;
	}
}