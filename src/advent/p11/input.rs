use super::{error::AdventError, Stone};

pub fn get_input() -> Result<Vec<Stone>, AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/11.txt"),
        AdventError::FileReadError
    )?;
    content
        .split_whitespace()
        .map(|word| unwrap!(word.parse(), AdventError::ParseIntError(word.to_string())))
        .collect()
}
