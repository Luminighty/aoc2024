pub type Plot = char;

#[derive(Debug)]
pub struct Garden {
	plots: Vec<Plot>,
	width: usize,
	height: usize,
}

impl Garden {
	pub fn new(plots: Vec<Plot>, width: usize, height: usize) -> Self {
		Self { plots, width, height }
	}

	pub fn len(&self) -> usize {
		self.plots.len()
	}

	pub fn index_xy(&self, index: usize) -> (i32, i32) {
		let x = (index % self.width) as i32;
		let y = (index / self.width) as i32;
		(x, y)
	}

	pub fn get(&self, x: i32, y: i32) -> Option<&Plot> {
		match self.xy_index(x, y) {
			Some(idx) => self.plots.get(idx),
			_ => None
		}
	}

	pub fn xy_index(&self, x: i32, y: i32) -> Option<usize> {
		if x < 0 || y < 0 { return None; }
		let x = x as usize;
		let y = y as usize;
		if x >= self.width || y >= self.height { return None; }
		Some(x + y * self.width)
	}

	pub fn is_plot(&self, x: i32, y: i32, plot: Plot) -> bool {
		match self.get(x, y) {
			Some(&p) => p == plot,
			_ => false,
		}
	}

}


#[cfg(test)]
mod tests {
  use super::*;
	#[test]
	pub fn test_get() {
		let garden = Garden::new(vec!['A', 'B', 'C', 'D'], 2, 2);
		assert_eq!(garden.get(0, 0), Some(&'A'));
		assert_eq!(garden.get(1, 0), Some(&'B'));
		assert_eq!(garden.get(0, 1), Some(&'C'));
		assert_eq!(garden.get(1, 1), Some(&'D'));
		assert_eq!(garden.get(-1, -1), None);
		assert_eq!(garden.get(2, 0), None);
		assert_eq!(garden.get(0, 2), None);
	}

	#[test]
	pub fn test_is_plot() {
		let garden = Garden::new(vec!['A', 'B', 'C', 'D'], 2, 2);
		assert!(garden.is_plot(0, 0, 'A'));
		assert!(garden.is_plot(1, 0, 'B'));
		assert!(garden.is_plot(0, 1, 'C'));
		assert!(garden.is_plot(1, 1, 'D'));
		assert!(!garden.is_plot(2, 0, 'A'));
		assert!(!garden.is_plot(0, 2, 'A'));
		assert!(!garden.is_plot(0, -1, 'A'));
		assert!(!garden.is_plot(-1, 0, 'A'));
	}
}