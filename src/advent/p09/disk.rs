pub type DiskMap = Vec<u8>;

pub struct Disk {
	diskmap: DiskMap,
	left: usize,
	loffset: u8,
	right: usize,
	roffset: u8,
}

impl Disk {
	pub fn new(diskmap: DiskMap) -> Self {
		let right = diskmap.len() - 1;
		let roffset = diskmap[right];
		Self { 
			diskmap, 
			left: 0, 
			loffset: 0, 
			right,
			roffset
		}
	}

	fn step_left(&mut self) {
		self.loffset += 1;
		while self.left < self.diskmap.len() && self.loffset >= self.diskmap[self.left] {
			self.left += 1;
			self.loffset = 0;
		}
	}

	fn step_right(&mut self) {
		self.roffset -= 1;
		while self.right != 0 && (self.roffset == 0 || self.is_free(self.right)) {
			self.right -= 1;
			self.roffset = self.diskmap[self.right];
		}
	}

	fn is_end(&self) -> bool {
		if self.left < self.right { return false }
		if self.left > self.right { return true }
		self.loffset >= self.roffset
	}

	fn is_free(&self, index: usize) -> bool {
		index % 2 == 1
	}
}

impl Iterator for Disk {
	type Item = usize;
	fn next(&mut self) -> Option<Self::Item> {
		if self.is_end() { return None }
		if self.is_free(self.left) {
			let block = self.right / 2;
			self.step_right();
			self.step_left();
			Some(block)
		} else {
			let block = self.left / 2;
			self.step_left();
			Some(block)
		}
	}
}
