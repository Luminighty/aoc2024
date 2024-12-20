#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Stripe {
	White,
	Blue,
	Black,
	Red,
	Green,
}

#[derive(Hash, PartialEq, Eq)]
pub struct Pattern {
	stripes: Vec<Stripe>,
}

impl Pattern {
	pub fn new(stripes: Vec<Stripe>) -> Self {
		Self { stripes }
	}

	pub fn concat(&self, other: &Self) -> Self {
		let mut stripes = Vec::with_capacity(self.len() + other.len());
		for s in &self.stripes { stripes.push(s.clone()); }
		for s in &other.stripes { stripes.push(s.clone()); }
		Self { stripes }
	}

	pub fn slice(&self, start: usize) -> Self {
		let mut stripes = Vec::with_capacity(self.len() - start);
		for s in &self.stripes[start..] {
			stripes.push(s.clone());
		}
		Self { stripes }
	}

	pub fn matches(&self, other: &Self, offset: usize) -> bool {
		if other.stripes.len() > self.stripes.len() - offset { return false; }
		for (i, stripe) in other.stripes.iter().enumerate() {
			if stripe != &self.stripes[i + offset] { return false; }
		}
		return true;
	}

	pub fn len(&self) -> usize {
		self.stripes.len()
	}
}

impl std::fmt::Display for Pattern {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for s in self.stripes.iter() {
			write!(f, "{}", s)?;
		}
		Ok(())
	}
}

impl std::fmt::Display for Stripe {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			Stripe::White => "w",
			Stripe::Blue => "u",
			Stripe::Black => "b",
			Stripe::Red => "r",
			Stripe::Green => "g",
		})
	}
}
