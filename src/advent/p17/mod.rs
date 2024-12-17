use device::{Device, DeviceErr, Num, Op, Program};

pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 17: Chronospatial Computer";

#[macro_use]
mod error;
mod device;
mod input;

fn run(device: &mut Device, program: &Program) {
    loop {
        match device.step(program) {
            Err(DeviceErr::Halt) => {
                return;
            }
            Err(err) => {
                println!("{err:?}");
                return;
            }
            _ => {}
        }
    }
}

pub fn solve_a() {
    let (mut device, program) = unwrap!(input::get_input());
    run(&mut device, &program);
    let output: Vec<String> = device.output.iter().map(Op::to_string).collect();
    println!("Output: {}", output.join(","));
}

fn find_reg_a(offset: usize, program: &Program, value: Num) -> Option<Num> {
    for i in 0..8 {
        let value = value + i;
        let mut device = Device::new(value, 0, 0);
        run(&mut device, program);
        if program.get(offset) != device.output.get(0) {
            continue;
        }
        if offset == 0 {
            return Some(value);
        }
        if let Some(reg_a) = find_reg_a(offset - 1, program, value << 3) {
            return Some(reg_a);
        }
    }
    None
}

pub fn solve_b() {
    let (_, program) = unwrap!(input::get_input());
    if let Some(reg_a) = find_reg_a(program.len() - 1, &program, 0) {
        let mut device = Device::new(reg_a, 0, 0);
        run(&mut device, &program);
        println!("                  Program: {program:?}");
        println!("Output for fixed register: {:?}", device.output);
        println!("Fixed Register: {reg_a}");
    } else {
        println!("Could not find solution :(");
    }
}
