use super::error::AdventError;

pub type LocationId = i32;


fn parse_pair(line: &str) -> Result<(LocationId, LocationId), AdventError> {
	let mut split = line.split_whitespace();
	let first = split.next().ok_or(AdventError::InvalidLineError(line.to_string()))?;
	let second = split.next().ok_or(AdventError::InvalidLineError(line.to_string()))?;
	let first = unwrap!(first.parse(), AdventError::ParseIntError(first.to_string()))?;
	let second = unwrap!(second.parse(), AdventError::ParseIntError(second.to_string()))?;
	Ok((first, second))
}

pub fn get_input() -> Result<Vec<(LocationId, LocationId)>, AdventError> {
	let content = unwrap!(std::fs::read_to_string("inp/01.txt"), AdventError::FileReadError)?;

	content.lines()
		.map(parse_pair)
		.collect()
}
