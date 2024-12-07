use std::i64;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 7: Bridge Repair";

#[macro_use]
mod error;
mod input;

pub type Equation = (i64, Vec<i64>);

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Multiply,
    Concat,
}

impl Op {
    pub fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Op::Add => lhs + rhs,
            Op::Multiply => lhs * rhs,
            Op::Concat => lhs * i64::pow(10, rhs.to_string().len() as u32) + rhs,
        }
    }
}

fn apply_ops(nums: &Vec<i64>, ops: &Vec<Op>) -> i64 {
    let mut sum = nums[0];
    for i in 1..nums.len() {
        sum = ops[i - 1].apply(sum, nums[i])
    }
    sum
}

fn find_equation(
    equation: &Equation,
    ops: &mut Vec<Op>,
    with: Op,
    concat_enabled: bool,
) -> Option<Vec<Op>> {
    ops.push(with);
    if equation.1.len() - 1 < ops.len() + 1 {
        if apply_ops(&equation.1, ops) == equation.0 {
            return Some(ops.clone());
        }
        ops.pop();
        return None;
    }
    if let Some(correct) = find_equation(equation, ops, Op::Add, concat_enabled) {
        return Some(correct);
    }
    if let Some(correct) = find_equation(equation, ops, Op::Multiply, concat_enabled) {
        return Some(correct);
    }
    if concat_enabled {
        if let Some(correct) = find_equation(equation, ops, Op::Concat, concat_enabled) {
            return Some(correct);
        }
    }
    ops.pop();
    None
}

fn is_valid_equation(equation: &Equation, concat_enabled: bool) -> Option<Vec<Op>> {
    let mut ops = Vec::new();
    if let Some(correct) = find_equation(equation, &mut ops, Op::Add, concat_enabled) {
        return Some(correct);
    }
    if let Some(correct) = find_equation(equation, &mut ops, Op::Multiply, concat_enabled) {
        return Some(correct);
    }
    if concat_enabled {
        if let Some(correct) = find_equation(equation, &mut ops, Op::Concat, concat_enabled) {
            return Some(correct);
        }
    }
    None
}

pub fn solve_a() {
    let equations = unwrap!(input::get_input());

    let mut sum = 0;
    for eq in &equations {
        if is_valid_equation(eq, false).is_some() {
            sum += eq.0;
        }
    }
    println!("Sum {sum}");
}

pub fn solve_b() {
    assert_eq!(156, Op::Concat.apply(15, 6));
    assert_eq!(178, Op::Concat.apply(17, 8));
    assert_eq!(1781, Op::Concat.apply(178, 1));
    assert_eq!(1234, Op::Concat.apply(1, 234));

    let equations = unwrap!(input::get_input());

    let mut sum = 0;
    for eq in &equations {
        if is_valid_equation(eq, true).is_some() {
            sum += eq.0;
        }
    }
    println!("Sum {sum}");
}
