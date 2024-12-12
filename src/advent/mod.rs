pub const CONFIG: (&'static str, fn(), fn()) = ("Advent of Code", run_all, || {});

mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;
mod p08;
mod p09;
mod p10;
mod p11;
mod p12;
// mod p13;
// mod p14;
// mod p15;
// mod p16;
// mod p17;
// mod p18;
// mod p19;
// mod p20;
// mod p21;
// mod p22;
// mod p23;
// mod p24;
// mod p25;

#[allow(dead_code)]
pub enum Advent {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
    AdventOfCode,
}

impl Advent {
    pub fn run(&self) {
        let (title, solve_a, solve_b) = match self {
            Advent::AdventOfCode => CONFIG,
            Advent::Day1 => p01::CONFIG,
            Advent::Day2 => p02::CONFIG,
            Advent::Day3 => p03::CONFIG,
            Advent::Day4 => p04::CONFIG,
            Advent::Day5 => p05::CONFIG,
            Advent::Day6 => p06::CONFIG,
            Advent::Day7 => p07::CONFIG,
            Advent::Day8 => p08::CONFIG,
            Advent::Day9 => p09::CONFIG,
            Advent::Day10 => p10::CONFIG,
            Advent::Day11 => p11::CONFIG,
            Advent::Day12 => p12::CONFIG,
            // Advent::Day13 => p13::CONFIG,
            // Advent::Day14 => p14::CONFIG,
            // Advent::Day15 => p15::CONFIG,
            // Advent::Day16 => p16::CONFIG,
            // Advent::Day17 => p17::CONFIG,
            // Advent::Day18 => p18::CONFIG,
            // Advent::Day19 => p19::CONFIG,
            // Advent::Day20 => p20::CONFIG,
            // Advent::Day21 => p21::CONFIG,
            // Advent::Day22 => p22::CONFIG,
            // Advent::Day23 => p23::CONFIG,
            // Advent::Day24 => p24::CONFIG,
            // Advent::Day25 => p25::CONFIG,
            _ => panic!("Day is missing!"),
        };

        println!("\n~~~~~ {title} ~~~~~");
        solve_a();
        solve_b();
    }
}

fn run_all() {
    DAYS.iter().for_each(|day| day.run());
}

const DAYS: [Advent; 25] = [
    Advent::Day1,
    Advent::Day2,
    Advent::Day3,
    Advent::Day4,
    Advent::Day5,
    Advent::Day6,
    Advent::Day7,
    Advent::Day8,
    Advent::Day9,
    Advent::Day10,
    Advent::Day11,
    Advent::Day12,
    Advent::Day13,
    Advent::Day14,
    Advent::Day15,
    Advent::Day16,
    Advent::Day17,
    Advent::Day18,
    Advent::Day19,
    Advent::Day20,
    Advent::Day21,
    Advent::Day22,
    Advent::Day23,
    Advent::Day24,
    Advent::Day25,
];
