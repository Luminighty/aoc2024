use regex::Regex;

use super::{error::AdventError, robot::Robot};


fn parse_robot(line: &str) -> Result<Robot, AdventError> {
    let robot_re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")?;
    let robot = robot_re.captures(line).ok_or(AdventError::FileFormat(line.to_owned()))?;
    let x = &robot.get(1).ok_or(AdventError::FileFormat(line.to_owned()))?;
    let y = &robot.get(2).ok_or(AdventError::FileFormat(line.to_owned()))?;
    let vx = &robot.get(3).ok_or(AdventError::FileFormat(line.to_owned()))?;
    let vy = &robot.get(4).ok_or(AdventError::FileFormat(line.to_owned()))?;
    let x = unwrap!(str::parse(x.as_str()), AdventError::ParseIntError(line.to_owned()))?;
    let y = unwrap!(str::parse(y.as_str()), AdventError::ParseIntError(line.to_owned()))?;
    let vx = unwrap!(str::parse(vx.as_str()), AdventError::ParseIntError(line.to_owned()))?;
    let vy = unwrap!(str::parse(vy.as_str()), AdventError::ParseIntError(line.to_owned()))?;
    Ok(Robot::new(x, y, vx, vy))
}

pub fn get_input() -> Result<Vec<Robot>, AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/14.txt"),
        AdventError::FileReadError
    )?;
    content
        .lines()
        .map(parse_robot)
        .collect()
}
