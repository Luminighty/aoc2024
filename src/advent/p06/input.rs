use super::{
    error::AdventError,
    guard::{Guard, Map, Tile},
};

pub fn get_input() -> Result<(Guard, Map), AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/06.txt"),
        AdventError::FileReadError
    )?;
    let mut guard = None;
    let mut tiles: Vec<Tile> = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for (y, line) in content.lines().enumerate() {
        height += 1;
        let mut t = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Ground,
                '#' => Tile::Wall,
                '^' if guard.is_none() => {
                    guard = Some(Guard::new(x as i32, y as i32, super::guard::Dir::Up));
                    Tile::Ground
                }
                '^' => {
                    return Err(AdventError::MultipleGuards);
                }
                _ => {
                    return Err(AdventError::UndefinedTile(c));
                }
            };
            t.push(tile);
        }
        width = t.len();
        tiles.append(&mut t);
    }
    let guard = guard.ok_or(AdventError::GuardNotFound)?;
    let map = Map::new(tiles, width, height);
    Ok((guard, map))
}
