// Advent of code 2026
// This binary is run with the day in question as an argument --e.g., --day 1

use clap::Parser;

mod day01;
mod day02;

#[derive(Parser, Debug)]
#[command(name = "aoc26")]
#[command(about = "Advent of Code 2026 Solutions", long_about = None)]
struct Args {
    /// Day number to run (1-25)
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    println!("{}", (-18_i32).rem_euclid(100));
    println!("{}", (0_i32).div_euclid(100));

    match args.day {
        1 => day01::solve(),
        2 => day02::solve(),
        _ => println!("Day {} not implemented yet", args.day),
    }
}
