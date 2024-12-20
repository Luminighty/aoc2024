use super::{error::AdventError, towel::{Pattern, Stripe}};

fn parse_stripe(stripe: char) -> Result<Stripe, AdventError> {
  match stripe {
    'r' => Ok(Stripe::Red),
    'w' => Ok(Stripe::White),
    'b' => Ok(Stripe::Black),
    'g' => Ok(Stripe::Green),
    'u' => Ok(Stripe::Blue),
    _ => Err(AdventError::UnknownStripe(stripe))
  } 
}

fn parse_pattern(pattern: &str) -> Result<Vec<Stripe>, AdventError> {
  pattern
    .chars()
    .map(parse_stripe)
    .collect()
}

pub fn get_input() -> Result<(Vec<Pattern>, Vec<Pattern>), AdventError> {
  let content = unwrap!(
    std::fs::read_to_string("inp/19.txt"),
    AdventError::FileReadError
  )?;
  let (patterns, desired) = content
    .split_once("\n\n")
    .ok_or(AdventError::FileFormatError)?;
  let patterns = patterns
    .split(", ")
    .map(parse_pattern)
    .map(|res| res.map(|stripes| Pattern::new(stripes)))
    .collect::<Result<Vec<Pattern>, AdventError>>()?;
  let desired = desired
    .lines()
    .map(parse_pattern)
    .map(|res| res.map(|stripes| Pattern::new(stripes)))
    .collect::<Result<Vec<Pattern>, AdventError>>()?;

  Ok((patterns, desired))
}
