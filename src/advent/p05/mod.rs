use print::{compare_update, is_valid_update, Update};

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 5: Print Queue";

#[macro_use]
mod error;
mod input;
mod print;

pub fn solve_a() {
    let (rules, updates) = unwrap!(input::get_input());

    let sum = updates
        .iter()
        .filter(|update| is_valid_update(&rules, update))
        .map(print::update_middle)
        .reduce(|acc, curr| acc + curr);

    println!("Sum: {sum:?}");
}

pub fn solve_b() {
    let (rules, updates) = unwrap!(input::get_input());

    let mut incorrect_updates: Vec<Update> = updates
        .into_iter()
        .filter(|update| !is_valid_update(&rules, update))
        .collect();

    for update in incorrect_updates.iter_mut() {
        update.sort_by(|a, b| compare_update(&rules, a, b));
        // println!("{update:?}");
    }

    let sum = incorrect_updates
        .iter()
        .map(print::update_middle)
        .reduce(|acc, curr| acc + curr);

    println!("Sum: {sum:?}");
}
