// Advent of code 2026
// This binary is run with the day in question as an argument --e.g., --day 1

use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day07;
mod day08;
mod day6a;
mod day6b;

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

    match args.day {
        1 => {
            day01::solve_problem_1("input/day01.txt").unwrap();
            day01::solve_problem_2("input/day01.txt").unwrap();
        }
        2 => {
            day02::solve_problem_1("input/day02.txt").unwrap();
            day02::solve_problem_2("input/day02.txt").unwrap();
        }
        3 => {
            day03::solve_problem_1("input/day03.txt").unwrap();
            day03::solve_problem_2("input/day03.txt").unwrap();
        }
        4 => {
            day04::solve_problem_1("input/day04.txt").unwrap();
            day04::solve_problem_2("input/day04.txt").unwrap();
        }
        5 => {
            day05::solve_problem_1("input/day05.txt").unwrap();
            day05::solve_problem_2("input/day05.txt").unwrap();
        }
        6 => {
            day6a::solve_problem_1("input/day06.txt").unwrap();
            day6b::solve_problem_2("input/day06.txt").unwrap();
        }
        7 => {
            day07::solve_problem_1("input/day07.txt").unwrap();
            day07::solve_problem_2("input/day07.txt").unwrap();
        }
        8 => {
            day08::solve_problem_1("input/day08.txt").unwrap();
            day08::solve_problem_2("input/day08.txt").unwrap();
        }
        _ => println!("Day {} not implemented yet", args.day),
    }
}
