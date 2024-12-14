pub const CONFIG: (&'static str, fn(), fn()) = (TITLE, solve_a, solve_b);
pub const TITLE: &'static str = "Day 13: Claw Contraption";

#[macro_use]
mod error;
mod input;
mod machine;



pub fn solve_a() {
    let machines = unwrap!(input::get_input());
    let mut sum = 0;
    for (i, machine) in machines.into_iter().enumerate() {
        let min = machine.into_iter()
            .map(|win| win.cost())
            .min();
        // println!("Machine {i} => {min:?}");
        if let Some(min) = min {
            sum += min;
        }
    }
    println!("Cost {sum}");
}

pub fn solve_b() {
    let machines = unwrap!(input::get_input());
    let mut sum = 0;
    for (i, mut machine) in machines.into_iter().enumerate() {
        machine.fix_prize();
        let a = machine.cram_a();
        let a = a.map(|a| machine.get_b(a)).flatten();
        // println!("Machine {i} => {a:?}");
        if let Some(cost) = a.map(|a| a.cost()) {
            sum += cost;
        }
    }
    println!("Cost {sum}");
}
