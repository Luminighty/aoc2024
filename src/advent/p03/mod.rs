pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 3: Mull It Over";

#[macro_use]
mod error;
mod input;
mod token;

pub fn solve_a() {
    let data = unwrap!(input::get_input());
    let tokenizer = token::Tokenizer::new(data);

    let mut sum = 0;
    for token in tokenizer {
        match token {
            token::Token::Mul(left, right) => {
                let res = left * right;
                sum += res;
            }
            _ => {}
        }
    }
    println!("Sum {sum}");
}

pub fn solve_b() {
    let data = unwrap!(input::get_input());
    let tokenizer = token::Tokenizer::new(data);

    let mut sum = 0;
    let mut enabled = true;
    for token in tokenizer {
        match token {
            token::Token::Mul(left, right) => {
                let res = left * right;
                if enabled {
                    sum += res;
                }
            }
            token::Token::DO => {
                enabled = true;
            }
            token::Token::DONT => {
                enabled = false;
            }
        }
    }
    println!("Sum {sum}");
}
