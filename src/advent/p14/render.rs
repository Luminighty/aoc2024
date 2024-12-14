use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};

use super::robot::{Robot, HEIGHT, WIDTH};

enum Playback {
	Forward, Backward, Stop,
}
const EASTER_EGG: usize = 6532;

pub fn renderer(mut robots: Vec<Robot>) {
	let mut seconds: usize = 0;
	let mut playback = Playback::Forward;
	let mut delay = 8;
	let mut current_delay = 8;

	let mut buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];
	let mut options = WindowOptions::default();
	options.scale = Scale::X2;
	let mut window = Window::new(
		"AoC24 - Day 14",
		 WIDTH as usize, HEIGHT as usize, 
		 options,
		).unwrap();
	
  window.set_target_fps(60);
	while window.is_open() && !window.is_key_down(Key::Escape) {
		current_delay -= 1;
		let step = input(&window, &mut playback, &mut delay);
		match step {
			Playback::Forward => { update(&Playback::Forward, &mut robots, &mut seconds); },
			Playback::Backward => { update(&Playback::Backward, &mut robots, &mut seconds); },
			_ => {},
		}

		if current_delay == 0 {
			current_delay = delay;
			update(&playback, &mut robots, &mut seconds);
			draw(&mut buffer, &robots);
		}

		if seconds == EASTER_EGG {
			playback = Playback::Stop;
		}

		window.update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize).unwrap();
	}
}

fn input(window: &Window, playback: &mut Playback, delay: &mut usize) -> Playback {
	if window.is_key_pressed(Key::A, KeyRepeat::No) {
		println!("Playback::Backward");
		*playback = Playback::Backward;
	}
	if window.is_key_pressed(Key::S, KeyRepeat::No) {
		println!("Playback::Stop");
		*playback = Playback::Stop;
	}
	if window.is_key_pressed(Key::D, KeyRepeat::No) {
		println!("Playback::Forward");
		*playback = Playback::Forward;
	}
	if window.is_key_pressed(Key::Key1, KeyRepeat::No) && *delay > 1 {
		println!("Delay set to {delay}");
		*delay -= 1;
	}
	if window.is_key_pressed(Key::Key2, KeyRepeat::No) {
		println!("Delay set to {delay}");
		*delay += 1;
	}
	if window.is_key_pressed(Key::Right, KeyRepeat::No) {
		*playback = Playback::Stop;
		return Playback::Forward;
	}
	if window.is_key_pressed(Key::Left, KeyRepeat::No) {
		*playback = Playback::Stop;
		return Playback::Backward;
	}
	Playback::Stop
}

fn update(playback: &Playback, robots: &mut Vec<Robot>, seconds: &mut usize) {
	match playback {
		Playback::Backward if *seconds > 0 => {
			*seconds -= 1;
			for robot in robots.iter_mut() {
				robot.back();
			}
		},
		Playback::Forward => {
			*seconds += 1;
			for robot in robots.iter_mut() {
				robot.step();
			}
		},
		_ => {},
	}
	println!("Seconds: {seconds}");
}

const ROBOT: u32 = 0x00FF00;
fn draw(buffer: &mut Vec<u32>, robots: &Vec<Robot>) {
	for i in buffer.iter_mut() {
		*i = 0;
	}
	for robot in robots {
		let i = (robot.x + robot.y * WIDTH) as usize;
		buffer[i] = ROBOT;
	}
}