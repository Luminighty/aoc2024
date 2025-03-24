use super::error::AdventError;

pub fn get_input() -> Result<(), AdventError> {
  let content = unwrap!(
    std::fs::read_to_string("inp/.txt"),
    AdventError::FileReadError
  )?;
  Ok(())
}
