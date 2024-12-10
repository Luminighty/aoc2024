pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 9: Disk Fragmenter";

#[macro_use]
mod error;
mod input;
mod disk;
mod defragment;


pub fn solve_a() {
	let diskmap = unwrap!(input::get_input());
	let disk = disk::Disk::new(diskmap);
	let mut sum = 0;
	for (i, block) in disk.enumerate() {
		sum += i * block;
	}
	println!("checksum: {sum}");
}

pub fn solve_b() {
	let diskmap = unwrap!(input::get_input());
	let defrag = defragment::Defragment::new(diskmap);
	let mut i = 0;
	let mut checksum = 0;
	for block in defrag {
		for _ in 0..block.width {
			checksum += block.file_id.unwrap_or(0) * i;
			i += 1;
		}
	}
	println!("checksum: {checksum}");
}