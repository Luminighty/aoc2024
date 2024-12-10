use super::{disk::DiskMap, error::AdventError};

pub fn get_input() -> Result<DiskMap, AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/09.txt"),
        AdventError::FileReadError
    )?;
    content.chars()
        .map(|n| n.to_digit(10).ok_or(AdventError::ParseIntError(n.to_string())))
        .map(|res| res.map(|n| n as u8))
        .collect()
}
