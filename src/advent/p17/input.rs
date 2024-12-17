use super::{
    device::{Device, Num, Op, Program},
    error::AdventError,
};

fn parse_register(register: &str) -> Result<Num, AdventError> {
    let (_, value) = register
        .trim()
        .split_once(": ")
        .ok_or(AdventError::FileFormatError(register.to_string()))?;
    let value = unwrap!(value.parse(), AdventError::ParseIntError(value.to_string()))?;
    Ok(value)
}

fn parse_registers(registers: &str) -> Result<(Num, Num, Num), AdventError> {
    let mut regs = registers.lines();
    let a = regs
        .next()
        .ok_or(AdventError::FileFormatError(registers.to_string()))?;
    let b = regs
        .next()
        .ok_or(AdventError::FileFormatError(registers.to_string()))?;
    let c = regs
        .next()
        .ok_or(AdventError::FileFormatError(registers.to_string()))?;
    let a = parse_register(a)?;
    let b = parse_register(b)?;
    let c = parse_register(c)?;
    Ok((a, b, c))
}

fn parse_program(program: &str) -> Result<Vec<Op>, AdventError> {
    let (_, program) = program
        .trim()
        .split_once(": ")
        .ok_or(AdventError::FileFormatError(program.to_string()))?;
    program
        .split(",")
        .map(|op| unwrap!(op.parse(), AdventError::ParseIntError(op.to_owned())))
        .collect()
}

pub fn get_input() -> Result<(Device, Program), AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/17.txt"),
        AdventError::FileReadError
    )?;
    let (registers, program) = content
        .split_once("\n\n")
        .ok_or(AdventError::FileFormatError(content.to_string()))?;
    let (a, b, c) = parse_registers(registers)?;
    let program = parse_program(program)?;
    Ok((Device::new(a, b, c), program))
}
