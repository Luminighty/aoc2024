use std::cmp::Ordering;

use input::Report;

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 2: Red-Nosed Reports";

#[macro_use]
mod error;
mod input;

const MAX_DIFF: i32 = 3;

fn get_ordering(report: &Report) -> Ordering {
	let mut increasing_count = 0;
	let mut decreasing_count = 0;
	for i in 1..report.len() {
		match report[i].cmp(&report[i - 1]) {
			Ordering::Greater => {increasing_count += 1;},
			Ordering::Less => {decreasing_count += 1;},
			_ => {},
		}
	}
	decreasing_count.cmp(&increasing_count)
}

fn is_bad_level(mode: Ordering, report: &Report, level: usize) -> bool {
	(report[level - 1] - report[level]).abs() > MAX_DIFF || report[level - 1].cmp(&report[level]) != mode
}

fn is_safe(report: &Report) -> bool {
	let mode = get_ordering(report);
	find_bad_level(report, mode).is_none()
}

fn find_bad_level(report: &Report, mode: Ordering) -> Option<usize> {
	for i in 1..report.len() {
		if is_bad_level(mode, report, i) {
			return Some(i);
		}
	}
	None
}

fn is_safe_with_dampener(report: &Report) -> bool {
	let mode = get_ordering(report);
	match find_bad_level(report, mode) {
		None => true,
		Some(i) => {
			let mut left = report.clone();
			let mut right = report.clone();
			left.remove(i - 1);
			right.remove(i);
			is_safe(&left) || is_safe(&right)
		}
	}
}

pub fn solve_a() {
	let reports = unwrap!(input::get_input());

	let c = reports.iter()
		.filter(|r| is_safe(r))
		.count();

	println!("Safe reports: {}", c);
}

pub fn solve_b() {
	let reports = unwrap!(input::get_input());

	let c = reports.iter()
		.filter(|r| is_safe_with_dampener(r))
		.count();
	println!("Safe reports with dampener: {}", c);
}
