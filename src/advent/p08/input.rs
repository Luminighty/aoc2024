use super::{antenna::Antenna, error::AdventError};


pub fn get_input() -> Result<(i32, i32, Vec<Antenna>), AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/08.txt"),
        AdventError::FileReadError
    )?;
    let mut antennas = Vec::new();
    let height = content.lines().count() as i32;
    let mut width = 0;
    for (y, line) in content.lines().enumerate() {
        width = line.len() as i32;
        for (x, c) in line.chars().enumerate() {
            if c == '.' { continue }
            antennas.push(Antenna::new(c, x as i32, y as i32));
        }
    }
    Ok((width, height, antennas))
}
