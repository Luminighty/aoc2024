use super::{error::AdventError, track::{Tile, Track}};

pub fn get_input() -> Result<(Track, (i32, i32), (i32, i32)), AdventError> {
  let content = unwrap!(
    std::fs::read_to_string("inp/20.txt"),
    AdventError::FileReadError
  )?;
  let mut start = None;
  let mut end = None;
  let mut track = Vec::new();
  let height = content.lines().count();
  for (y, line) in content.lines().enumerate() {
    for (x, c) in line.chars().enumerate() {
      match c {
        '#' => track.push(Tile::Wall),
        '.' => track.push(Tile::Floor),
        'S' => {
          track.push(Tile::Floor);
          start = Some((x as i32, y as i32))
        },
        'E' => {
          track.push(Tile::Floor);
          end = Some((x as i32, y as i32))
        }
        x => { return Err(AdventError::UndefinedChar(x)); }
      }
    }
  }
  let width = track.len() / height;
  let start = start.ok_or(AdventError::StartNotFound)?;
  let end = end.ok_or(AdventError::EndNotFound)?;
  let track = Track::new(width, height, track);
  Ok((track, start, end))
}
