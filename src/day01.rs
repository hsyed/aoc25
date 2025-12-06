use nom::character::complete::anychar;
use nom::{IResult, character::complete::i32 as nom_i32, sequence::tuple};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_problem_1(main_file: &str) -> std::io::Result<()> {
    let file = File::open(main_file)?;
    let reader = BufReader::new(file);
    let mut pos = 50;
    let mut zero_visits = 0;

    fn parse_line(input: &str) -> IResult<&str, (char, i32)> {
        tuple((anychar, nom_i32))(input)
    }

    reader.lines().for_each(|line| {
        if let Ok((_, (dir_char, steps))) = parse_line(line.unwrap().trim()) {
            pos = match dir_char {
                'L' => pos - steps,
                'R' => pos + steps,
                _ => panic!("unexpected dir char"),
            }
            .rem_euclid(100);

            if pos == 0 {
                zero_visits += 1;
            }
        }
    });

    println!("problem 1: zero was visited {} times", zero_visits);

    Ok(())
}

pub fn solve_problem_2(main_file: &str) -> std::io::Result<()> {
    let file = File::open(main_file)?;
    let reader = BufReader::new(file);
    let mut pos = 50;
    let mut zero_visits = 0;

    fn parse_line(input: &str) -> IResult<&str, (char, i32)> {
        tuple((anychar, nom_i32))(input)
    }

    reader.lines().for_each(|line| {
        if let Ok((_, (dir_char, steps))) = parse_line(line.unwrap().trim()) {
            // accumulate the full rotations
            zero_visits += steps / 100;

            // next seperate out the remainder and determine if the partial rotation crosses zero_visits
            let remainder = steps % 100;

            pos = match dir_char {
                'R' => {
                    // We hit 0 if we wrap around: pos + remainder >= 100
                    if pos + remainder >= 100 {
                        zero_visits += 1;
                    }
                    (pos + remainder).rem_euclid(100)
                }
                'L' => {
                    // check if we cross zero going left, starting at zero does not count as a
                    // crossing and -- the remainder is always <100 so if pos is 0 a crossing is
                    // impossible.
                    if pos != 0 && pos - remainder <= 0 {
                        zero_visits += 1;
                    }
                    (pos - remainder).rem_euclid(100)
                }
                _ => panic!("unexpected dir char"),
            }
        }
    });

    println!(
        "problem_2: zero was crossed or visitied {} times",
        zero_visits
    );
    Ok(())
}
