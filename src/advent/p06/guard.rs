#[derive(Clone, Copy)]
pub enum Tile {
    Ground,
    Wall,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ground => write!(f, "."),
            Tile::Wall => write!(f, "#"),
        }
    }
}

pub struct Map {
    tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(tiles: Vec<Tile>, width: usize, height: usize) -> Self {
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&Tile> {
        match self.xy_index(x, y) {
            Some(idx) => self.tiles.get(idx),
            _ => None,
        }
    }

    pub fn xy_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(x + self.width * y)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            if i % self.width == 0 {
                println!();
            }
            print!("{tile}");
        }
    }

    pub fn add_wall(&self, position: usize) -> Self {
        let mut tiles = self.tiles.clone();
        tiles[position] = Tile::Wall;
        Self {
            width: self.width,
            height: self.height,
            tiles,
        }
    }
}
#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn turn_right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    pub fn delta(&self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::Up => "^",
                Dir::Down => "v",
                Dir::Left => "<",
                Dir::Right => ">",
            }
        )
    }
}

#[derive(Clone, Copy)]
pub struct Guard {
    pub dir: Dir,
    pub x: i32,
    pub y: i32,
}

impl Guard {
    pub fn new(x: i32, y: i32, dir: Dir) -> Self {
        Self { x, y, dir }
    }

    pub fn next_pos(&self) -> (i32, i32) {
        let (dx, dy) = self.dir.delta();
        (self.x + dx, self.y + dy)
    }

    pub fn step(&mut self) {
        let (x, y) = self.next_pos();
        self.x = x;
        self.y = y;
    }

    pub fn turn_right(&mut self) {
        self.dir = self.dir.turn_right();
    }
}
