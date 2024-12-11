use std::collections::HashMap;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 11: Plutonian Pebbles";

#[macro_use]
mod error;
mod input;

pub type Stone = usize;

#[derive(Debug, PartialEq, Eq)]
enum StepResult {
	Value(Stone),
	Split(Stone, Stone)
}

fn step_stone(stone: Stone) -> StepResult {
	let stone_str = stone.to_string();
	if stone_str.len() % 2 == 0 {
		let (left, right) = stone_str.split_at(stone_str.len() / 2);
		let left = left.parse().unwrap();
		let right = right.parse().unwrap();
		return StepResult::Split(left, right);
	}
	if stone == 0 {
		StepResult::Value(1)
	} else {
		StepResult::Value(stone * 2024)
	}
}

fn step_stones(stones: Vec<Stone>) -> Vec<Stone> {
	let mut new_stones = Vec::with_capacity(stones.len());
	for stone in stones {
		match step_stone(stone) {
			StepResult::Split(left, right) => {
				new_stones.push(left);
				new_stones.push(right);
			},
			StepResult::Value(val) => {
				new_stones.push(val);
			}
		}
	}
	new_stones
}

fn step_stone_len(memo: &mut HashMap<(usize, usize), usize>, stone: Stone, amount: usize) -> usize {
	if amount == 0 { return 1; }
	if let Some(len) = memo.get(&(stone, amount)) {
		return *len;
	}
	let len = match step_stone(stone) {
    StepResult::Value(val) => {
			let len = step_stone_len(memo, val, amount - 1);
			len
		},
    StepResult::Split(left, right) => {
			let left_len = step_stone_len(memo, left, amount - 1);
			let right_len = step_stone_len(memo, right, amount - 1);
			left_len + right_len
		},
	};
	memo.insert((stone, amount), len);
	len
}

pub fn solve_a() {
	let mut stones = unwrap!(input::get_input());
	for _ in 0..25 {
		stones = step_stones(stones);
	}
	println!("Stones: {}", stones.len());
}

pub fn solve_b() {
	let stones = unwrap!(input::get_input());
	let mut memo = HashMap::new();
	let mut sum = 0;
	for stone in stones {
		let c = step_stone_len(&mut memo, stone, 75);
		sum += c;
	}
	println!("Stones: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_stone() {
        assert_eq!(step_stone(0), StepResult::Value(1));
        assert_eq!(step_stone(1), StepResult::Value(2024));
        assert_eq!(step_stone(10), StepResult::Split(1, 0));
        assert_eq!(step_stone(1234), StepResult::Split(12, 34));
        assert_eq!(step_stone(123), StepResult::Value(123 * 2024));
        assert_eq!(step_stone(5), StepResult::Value(5 * 2024));
    }

    #[test]
    fn test_step_stone_example() {
			assert_eq!(step_stone(0), StepResult::Value(1));
			assert_eq!(step_stone(1), StepResult::Value(2024));
			assert_eq!(step_stone(10), StepResult::Split(1, 0));
			assert_eq!(step_stone(99), StepResult::Split(9, 9));
			assert_eq!(step_stone(999), StepResult::Value(2021976));

			assert_eq!(step_stones(vec![0, 1, 10, 99, 999]), vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test_step_stones_example() {
			let result = step_stones(vec![0, 1, 10, 99, 999]);
			assert_eq!(result, vec![1, 2024, 1, 0, 9, 9, 2021976]);

			let mut result = vec![125, 17];
			result = step_stones(result);
			assert_eq!(result, vec![253000, 1, 7]); // 1
			result = step_stones(result);
			assert_eq!(result, vec![253, 0, 2024, 14168]); // 2
			result = step_stones(result);
			assert_eq!(result, vec![512072, 1, 20, 24, 28676032]); // 3
			result = step_stones(result);
			assert_eq!(result, vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]); // 4
			result = step_stones(result);
			assert_eq!(result, vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]); // 5
			result = step_stones(result);
			assert_eq!(result, vec![2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3, 2]); // 6
    }
}