use super::error::AdventError;

pub type Level = i32;
pub type Report = Vec<Level>;

fn parse_report(line: &str) -> Result<Report, AdventError> {
	line.split_whitespace()
		.map(str::parse)
		.map(|r| unwrap!(r, AdventError::ParseIntError(line.to_string())))
		.collect()
}

pub fn get_input() -> Result<Vec<Report>, AdventError> {
	let content = unwrap!(std::fs::read_to_string("inp/02.txt"), AdventError::FileReadError)?;

	content.lines()
		.map(parse_report)
		.collect()
}
