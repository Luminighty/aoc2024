use std::collections::HashSet;

pub struct Memory {
  space: Vec<HashSet<(i32, i32)>>,
  bytes: Vec<(i32, i32)>,
  time: usize,
}

impl Memory {
  pub fn new(bytes: Vec<(i32, i32)>) -> Self {
    let mut space = vec![HashSet::new(); bytes.len() + 1];
    for (i, pos) in bytes.iter().enumerate() {
      for t in i..bytes.len() {
        space[t + 1].insert(pos.clone());
      }
    }
    let time = space.len();
    Self { space, time, bytes }
  }

  pub fn set_time(&mut self, time: usize) {
    self.time = time.min(self.space.len());
  }

  pub fn is_wall(&self, x: i32, y: i32) -> bool {
    self.space[self.time].contains(&(x, y))
  }

  pub fn len(&self) -> usize {
    self.space.len()
  }

  pub fn get_byte(&self, time: usize) -> Option<&(i32, i32)> {
    self.bytes.get(time)
  }
}
