use super::{
    error::AdventError,
    maze::{Maze, Pos, Tile},
};

pub fn get_input() -> Result<(Maze, Pos, Pos), AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/16.txt"),
        AdventError::FileReadError
    )?;
    let mut tiles = Vec::new();
    let height = content.lines().count();
    let mut start = None;
    let mut end = None;
    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    tiles.push(Tile::Floor);
                    start = Some((x as i32, y as i32))
                }
                'E' => {
                    tiles.push(Tile::Floor);
                    end = Some((x as i32, y as i32))
                }
                '#' => {
                    tiles.push(Tile::Wall);
                }
                '.' => {
                    tiles.push(Tile::Floor);
                }
                _ => {
                    return Err(AdventError::UnknownTile(c));
                }
            }
        }
    }
    let width = tiles.len() / height;
    let start = start.ok_or(AdventError::MissingStart)?;
    let end = end.ok_or(AdventError::MissingEnd)?;
    Ok((Maze::new(tiles, width, height), start, end))
}
