use super::{error::AdventError, hike::{Map, Tile}};

pub fn get_input() -> Result<Map, AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/10.txt"),
        AdventError::FileReadError
    )?;
    let mut tiles = Vec::new();
    for line in content.lines() {
        for c in line.chars() {
            if c == '.' {
                tiles.push(11);
                continue;
            }
            let num = c.to_digit(10).ok_or(AdventError::ParseIntError(line.to_string()))?;
            tiles.push(num as Tile);
        }
    }
    let height = content.lines().count();
    let width = tiles.len() / height;
    Ok(Map::new(tiles, width, height))
}
