use super::{error::AdventError, garden::{Garden, Plot}};

pub fn get_input() -> Result<Garden, AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/12.txt"),
        AdventError::FileReadError
    )?;
    let plots: Vec<Plot> = content.lines().flat_map(|line| line.chars()).collect();
    let height = content.lines().count();
    let width = plots.len() / height;
    Ok(Garden::new(plots, width, height))
}
