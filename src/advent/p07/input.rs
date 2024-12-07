use std::num::ParseIntError;

use super::{error::AdventError, Equation};

fn parse_equation(line: &str) -> Result<Equation, AdventError> {
    let (target, nums) = line
        .split_once(": ")
        .ok_or(AdventError::FileFormat(line.to_owned()))?;
    let target = unwrap!(target.parse(), AdventError::ParseInt(line.to_owned()))?;
    let nums: Result<Vec<i64>, ParseIntError> =
        nums.split_whitespace().map(|n| n.parse()).collect();
    let nums = unwrap!(nums, AdventError::ParseInt(line.to_owned()))?;

    Ok((target, nums))
}

pub fn get_input() -> Result<Vec<Equation>, AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/07.txt"),
        AdventError::FileReadError
    )?;

    content.lines().map(parse_equation).collect()
}
