use super::disk::DiskMap;

#[derive(Debug, Clone, Copy)]
pub struct Block {
	pub width: u8,
	pub file_id: Option<usize>
}

impl std::fmt::Display for Block {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let c = match self.file_id {
			Some(file) => file.to_string(),
			None => ".".to_string(),
		};
		for _ in 0..self.width {
			write!(f, "{c}")?;
		}
		Ok(())
	}
}

impl Block {
	fn file(file_id: usize, width: u8) -> Self {
		Self { width, file_id: Some(file_id) }
	}
	fn free(width: u8) -> Self {
		Self { width, file_id: None }
	}
	pub fn is_free(&self) -> bool {
		self.file_id.is_none()
	}
}

pub struct Defragment {
	blocks: Vec<Block>,
	i: usize,
}

impl Defragment {
	pub fn new(diskmap: DiskMap) -> Self {
		let blocks = diskmap
			.iter()
			.enumerate()
			.map(|(i, width)| if i % 2 == 0 { Block::file(i / 2, *width) } else { Block::free(*width)} )
			.collect();
		Self { blocks, i: 0 }
	}

	fn find_block_index(&self, width: u8) -> Option<usize> {
		for i in (self.i..self.blocks.len()).rev() {
			let block = &self.blocks[i];
			if !block.is_free() && block.width <= width {
				return Some(i);
			}
		}
		None
	}
}

impl Iterator for Defragment {
	type Item = Block;
	fn next(&mut self) -> Option<Self::Item> {
		if self.i >= self.blocks.len() { return None; }
		let current = &self.blocks[self.i];
		if !current.is_free() { 
			self.i += 1;
			return Some(current.clone())
		}
		let next = self.find_block_index(current.width).unwrap_or(self.i);
		if next > self.i {
			// println!("#{} {:?} <- #{next} {:?}", self.i, self.blocks[self.i], self.blocks[next]);
			self.blocks[self.i].width -= self.blocks[next].width;
			let removed = self.blocks[next].clone();
			self.blocks[next].file_id = None;
			Some(removed)
		} else {
			self.i += 1;
			Some(self.blocks[next].clone())
		}
	}
}