use nom::{
    IResult,
    branch::alt,
    character::complete::{char, digit1, line_ending, space0, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::terminated,
};
use std::fs;

pub fn solve_problem_1(main_file: &str) -> std::io::Result<()> {
    let problems = read_worksheet_file(main_file)?;
    let solution: u64 = problems.iter().map(|p| p.solve()).sum();
    println!("problem 1: solution: {}", solution);
    Ok(())
}

fn read_worksheet_file(file: &str) -> std::io::Result<Vec<Problem>> {
    let content = fs::read_to_string(file)?;
    let (_, problems) = parse_problems(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
    Ok(problems)
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
struct Problem {
    operands: Vec<u64>,
    op: Op,
}

impl Problem {
    fn new(operands: Vec<u64>, op: Op) -> Self {
        Self { operands, op }
    }

    fn solve(&self) -> u64 {
        match self.op {
            Op::Add => self.operands.iter().sum(),
            Op::Mul => self.operands.iter().product(),
        }
    }
}

fn parse_problems(input: &str) -> IResult<&str, Vec<Problem>> {
    fn parse_u64(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(input)
    }

    fn parse_op(input: &str) -> IResult<&str, Op> {
        alt((map(char('+'), |_| Op::Add), map(char('*'), |_| Op::Mul)))(input)
    }

    // Parse rows of numbers
    let (input, rows) = separated_list1(
        line_ending,
        terminated(separated_list1(space1, parse_u64), space0),
    )(input)?;
    let (input, _) = line_ending(input)?;

    // Parse operators separated by spaces, with optional trailing spaces
    let (input, ops) = terminated(separated_list1(space1, parse_op), space0)(input)?;

    // Transpose: each column becomes a problem with operands from each row
    let num_cols = rows.first().map(|r| r.len()).unwrap_or(0);
    let problems = (0..num_cols)
        .map(|col_idx| {
            let operands: Vec<u64> = rows.iter().map(|row| row[col_idx]).collect();
            let op = ops[col_idx];
            Problem::new(operands, op)
        })
        .collect();

    Ok((input, problems))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> &'static str {
        "123 328  51 64 \n45 64  387 23 \n6 98  215 314 \n*   +   *   +"
    }

    #[test]
    fn test_parse_problems() {
        let input = test_data();
        let (remaining, problems) = parse_problems(input).unwrap();

        assert_eq!(remaining, "");
        assert_eq!(problems.len(), 4);

        assert_eq!(problems[0].operands, vec![123, 45, 6]);
        assert!(matches!(problems[0].op, Op::Mul));

        assert_eq!(problems[1].operands, vec![328, 64, 98]);
        assert!(matches!(problems[1].op, Op::Add));

        assert_eq!(problems[2].operands, vec![51, 387, 215]);
        assert!(matches!(problems[2].op, Op::Mul));

        assert_eq!(problems[3].operands, vec![64, 23, 314]);
        assert!(matches!(problems[3].op, Op::Add));
    }
}
