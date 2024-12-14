use super::{error::AdventError, machine::{Machine, Pos}};

fn parse_button(button: &str) -> Result<Pos, AdventError> {
    let (left, right) = button.split_once(",").ok_or(AdventError::FileFormatError(button.to_owned()))?;
    let (_, x) = left.split_once("+").ok_or(AdventError::FileFormatError(button.to_owned()))?;
    let (_, y) = right.split_once("+").ok_or(AdventError::FileFormatError(button.to_owned()))?;
    let x = unwrap!(x.parse(), AdventError::ParseIntError(x.to_string()))?;
    let y = unwrap!(y.parse(), AdventError::ParseIntError(y.to_string()))?;
    Ok(Pos::new(x, y))
}

fn parse_prize(prize: &str) -> Result<Pos, AdventError> {
    let (left, right) = prize.split_once(",").ok_or(AdventError::FileFormatError(prize.to_owned()))?;
    let (_, x) = left.split_once("=").ok_or(AdventError::FileFormatError(prize.to_owned()))?;
    let (_, y) = right.split_once("=").ok_or(AdventError::FileFormatError(prize.to_owned()))?;
    let x = unwrap!(x.parse(), AdventError::ParseIntError(x.to_string()))?;
    let y = unwrap!(y.parse(), AdventError::ParseIntError(y.to_string()))?;
    Ok(Pos::new(x, y))
}

fn parse_machine(machine: &str) -> Result<Machine, AdventError> {
    let mut parts = machine.lines();
    let btn_a = parts.next().ok_or(AdventError::FileFormatError(machine.to_owned()))?;
    let btn_b = parts.next().ok_or(AdventError::FileFormatError(machine.to_owned()))?;
    let prize = parts.next().ok_or(AdventError::FileFormatError(machine.to_owned()))?;

    let btn_a = parse_button(btn_a)?;
    let btn_b = parse_button(btn_b)?;
    let prize = parse_prize(prize)?;

    Ok(Machine::new(btn_a, btn_b, prize))
}

pub fn get_input() -> Result<Vec<Machine>, AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/13.txt"),
        AdventError::FileReadError
    )?;
    content.split("\n\n").map(parse_machine).collect()
}
