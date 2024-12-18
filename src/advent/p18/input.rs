use super::error::AdventError;

fn parse_byte(line: &str) -> Result<(i32, i32), AdventError> {
  let (x, y) = line.split_once(",").ok_or(AdventError::FileFormatError(line.to_string()))?;
  let x = unwrap!(x.parse(), AdventError::ParseIntError(x.to_string()))?;
  let y = unwrap!(y.parse(), AdventError::ParseIntError(y.to_string()))?;
  Ok((x, y))
}

pub fn get_input() -> Result<Vec<(i32, i32)>, AdventError> {
  let content = unwrap!(
    std::fs::read_to_string("inp/18.txt"),
    AdventError::FileReadError
  )?;
  content
    .lines()
    .map(parse_byte)
    .collect()
}
