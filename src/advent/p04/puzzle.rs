pub struct Puzzle {
    puzzle: Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl Puzzle {
    pub fn new(puzzle: Vec<String>) -> Self {
        let height = puzzle.len();
        let width = puzzle[0].len();
        let puzzle = puzzle.iter().flat_map(|s| s.chars()).collect();
        Self {
            height,
            width,
            puzzle,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            None
        } else {
            self.puzzle.get(y * self.width + x).copied()
        }
    }

    pub fn len(&self) -> usize {
        self.puzzle.len()
    }

    pub fn index_to_xy(&self, i: usize) -> (i32, i32) {
        let x = i % self.width;
        let y = i / self.width;
        (x as i32, y as i32)
    }
}
