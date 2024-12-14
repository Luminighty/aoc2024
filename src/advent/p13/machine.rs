use std::ops::Sub;

#[derive(Debug)]
pub struct Pos {
	x: isize,
	y: isize,
}

#[derive(Debug)]
pub struct Machine {
	btn_a: Pos,
	btn_b: Pos,
	prize: Pos,
}

impl Pos {
	pub fn new(x: isize, y: isize) -> Self {
		Self { x, y }
	}
}

impl Machine {
	pub fn new(btn_a: Pos, btn_b: Pos, prize: Pos) -> Self {
		Self { btn_a, btn_b, prize }
	}

	pub fn is_winning(&self, press_a: isize, press_b: isize) -> bool {
		self.btn_a.x * press_a + self.btn_b.x * press_b == self.prize.x &&
		self.btn_a.y * press_a + self.btn_b.y * press_b == self.prize.y
	}

	pub fn fix_prize(&mut self) {
		self.prize.x += 10000000000000;
		self.prize.y += 10000000000000;
	}

	pub fn cram_a(&self) -> Option<isize> {
		let quotent = (self.prize.x * self.btn_b.y) - (self.prize.y * self.btn_b.x);
		let dividend = (self.btn_a.x * self.btn_b.y) - (self.btn_a.y * self.btn_b.x);
		// println!("({} * {}) - ({} * {}) = {quotent}", self.prize.x, self.btn_b.y, self.prize.y, self.btn_b.x);
		// println!("({} * {}) - ({} * {}) = {dividend}", self.btn_a.x, self.btn_b.y, self.btn_a.y, self.btn_b.x);
		if quotent % dividend != 0 {
			None
		} else {
			Some(quotent / dividend)
		}
	}

	pub fn get_b(&self, a: isize) -> Option<Press> {
		let rem = &self.prize - self.btn_a.press_a(a);
		let b = rem.x / self.btn_b.x;
		Some(Press{ a, b })
	}
}

pub struct WinIter {
	machine: Machine,
	a_press: isize,
	
}

#[derive(Debug)]
pub struct Press {
	pub a: isize, pub b: isize
}

impl Press {
	pub fn cost(&self) -> isize {
		self.a * 3 + self.b
	}
}

impl Pos {
	pub fn press_a(&self, amount: isize) -> Self {
		Self {
			x: self.x * amount,
			y: self.y * amount,
		}
	}
}

impl Sub<Pos> for &Pos {
	type Output = Pos;
	fn sub(self, rhs: Pos) -> Self::Output {
		Pos {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}

impl IntoIterator for Machine {
	type IntoIter = WinIter;
	type Item = Press;

	fn into_iter(self) -> Self::IntoIter {
		let a_press = self.prize.x / self.btn_a.x + 1;
		WinIter { a_press, machine: self }
	}
}

impl Iterator for WinIter {
  type Item = Press;
  fn next(&mut self) -> Option<Self::Item> {
		loop {
			if self.a_press < 0 {
				return None;
			}

			let a_press = self.a_press;
			let remaining = &self.machine.prize - self.machine.btn_a.press_a(self.a_press);
			let b_press = remaining.x / self.machine.btn_b.x;
			self.a_press -= 1;
			if self.machine.is_winning(a_press, b_press) {
				return Some(Press { a: a_press, b: b_press });
			}
		};
  }
}