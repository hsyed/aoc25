use nom::{
    character::complete::{i32 as nom_i32}
    ,
    sequence::tuple,
    IResult,
};
use std::fs::File;
use std::io::{BufRead, BufReader};
use nom::character::complete::anychar;

fn process_instruction_file(_filename: &str) -> std::io::Result<i32> {
    let file = File::open(_filename)?;
    let reader = BufReader::new(file);
    let mut pos = 50;
    let mut zero_visits = 0;

    fn parse_line(input : &str) -> IResult<&str, (char, i32)> { tuple((anychar, nom_i32))(input) }

    reader.lines().for_each(|line| {
        if let Ok((_, (dir_char, steps))) = parse_line(line.unwrap().trim()) {
            match dir_char {
                'L' => pos = (pos - steps).rem_euclid(100),
                'R' => pos = (pos + steps).rem_euclid(100),
                _ => panic!("unexpected dir char")
            };

            println!("{}", pos);
            if pos == 0 {
                zero_visits += 1;
            }
        }
    });

    Ok(zero_visits)
}

pub fn solve() {
    println!("{:?}", process_instruction_file("input/day01.txt").unwrap());
}