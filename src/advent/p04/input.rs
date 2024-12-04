use super::error::AdventError;

pub fn get_input() -> Result<Vec<String>, AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/04.txt"),
        AdventError::FileReadError
    )?;
    Ok(content.lines().map(String::from).collect())
}
