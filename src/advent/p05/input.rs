use super::{
    error::AdventError,
    print::{Rule, Update},
};

fn parse_rule(line: &str) -> Result<Rule, AdventError> {
    let mut iter = line.splitn(2, "|");
    let first = iter.next().ok_or(AdventError::RuleError(line.to_owned()))?;
    let second = iter.next().ok_or(AdventError::RuleError(line.to_owned()))?;
    let first = unwrap!(first.parse(), AdventError::ParseIntError(first.to_owned()))?;
    let second = unwrap!(
        second.parse(),
        AdventError::ParseIntError(second.to_owned())
    )?;
    Ok(Rule(first, second))
}

fn parse_update(line: &str) -> Result<Update, AdventError> {
    line.split(",")
        .map(|num| num.parse())
        .map(|res| unwrap!(res, AdventError::ParseIntError(line.to_owned())))
        .collect()
}

pub fn get_input() -> Result<(Vec<Rule>, Vec<Update>), AdventError> {
    let content = unwrap!(
        std::fs::read_to_string("inp/05.txt"),
        AdventError::FileReadError
    )?;

    let (rules, updates) = content
        .split_once("\n\n")
        .ok_or(AdventError::FileFormatError)?;

    let rules = rules
        .lines()
        .map(parse_rule)
        .collect::<Result<Vec<Rule>, AdventError>>()?;
    let updates = updates
        .lines()
        .map(parse_update)
        .collect::<Result<Vec<Update>, AdventError>>()?;

    Ok((rules, updates))
}
