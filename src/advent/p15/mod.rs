pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 15: Warehouse Woes";

#[macro_use]
mod error;
mod input;
mod warehouse;
mod wide_warehouse;

pub fn solve_a() {
	let (mut warehouse, mut robot, actions) = unwrap!(input::get_input());
	for action in actions {
		warehouse::step(&mut warehouse, &mut robot, action);
	}
	println!("GPS score: {}", warehouse.gps());
}

pub fn solve_b() {
	let (warehouse, mut robot, actions) = unwrap!(input::get_input());
	let mut warehouse = wide_warehouse::WideWarehouse::from(warehouse);
	robot.x *= 2;
	for action in actions {
		wide_warehouse::step(&mut warehouse, &mut robot, action);
		//warehouse.print(&robot);
	}
	// warehouse.print(&robot);
	println!("GPS score: {}", warehouse.gps());
}
