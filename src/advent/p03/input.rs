use super::error::AdventError;

pub fn get_input() -> Result<String, AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/03.txt"),
        AdventError::FileReadError
    )?;
    Ok(content)
}
