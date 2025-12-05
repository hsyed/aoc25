use nom::{
    IResult,
    character::complete::{anychar, i32 as nom_i32},
    sequence::tuple,
};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn process_instruction_file(_filename: &str) -> std::io::Result<i32> {
    let file = File::open(_filename)?;
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

    Ok(zero_visits)
}

pub fn solve() {
    println!("{:?}", process_instruction_file("input/day01.txt").unwrap());
}
