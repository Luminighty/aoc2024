pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "";

#[macro_use]
mod error;
mod input;
mod track;

pub fn solve_a() {
	let (track, (sx, sy), end) = unwrap!(input::get_input());
	let path = track::Path::from_track(&track, sx, sy);
}

pub fn solve_b() {

}
