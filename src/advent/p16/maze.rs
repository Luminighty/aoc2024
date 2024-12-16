use std::fmt::Display;

pub type Pos = (i32, i32);

pub enum Tile {
    Wall,
    Floor,
}

pub struct Maze {
    tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Maze {
    pub fn new(tiles: Vec<Tile>, width: usize, height: usize) -> Self {
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        match self.get(x, y) {
            Some(Tile::Floor) => false,
            _ => true,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&Tile> {
        if let Some(i) = self.xy(x, y) {
            self.tiles.get(i)
        } else {
            None
        }
    }

    pub fn xy(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(x + y * self.width)
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self.get(x as i32, y as i32) {
                        Some(Tile::Wall) => "#",
                        Some(Tile::Floor) => ".",
                        _ => "?",
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Dir {
    Right,
    Left,
    Up,
    Down,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Dir::Down => "v",
                Dir::Right => ">",
                Dir::Left => "<",
                Dir::Up => "^",
            }
        )
    }
}

impl Dir {
    pub fn delta(&self) -> Pos {
        match self {
            Dir::Right => (1, 0),
            Dir::Left => (-1, 0),
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Left => Dir::Up,
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Down,
        }
    }
}
