use std::fs;

pub fn solve_problem_2(file_path: &str) -> std::io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    let reader = WorksheetReader::new(&content).unwrap();

    let mut tally = 0_u64;
    let mut operands: Vec<u64> = Vec::new();

    for it in reader {
        match it {
            Ok((operand, op)) => {
                operands.push(operand);
                match op {
                    Some(Op::Add) => {
                        let sum: u64 = operands.iter().sum();
                        tally += sum;
                        operands.clear();
                    }
                    Some(Op::Mull) => {
                        let product: u64 = operands.iter().product();
                        tally += product;
                        operands.clear();
                    }
                    None => {}
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    println!("problem 2: the total of the worksheet is {}", tally);
    Ok(())
}

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Mull,
}

struct WorksheetReader {
    operand_rows: Vec<String>,
    operator_row: String,
    pos: usize,
    scratch: String,
    finished: bool,
}

impl WorksheetReader {
    fn new(content: &str) -> Result<WorksheetReader, &str> {
        let mut operator_rows: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        if operator_rows.len() < 2 {
            return Err("Worksheet must have 2 lines minimum");
        }

        let len = operator_rows.first().unwrap().len();
        operator_rows.iter().try_for_each(|row| {
            if row.len() != len {
                Err("Inconsistent row lengths in worksheet")
            } else if !row.is_ascii() {
                Err("input is expected to be ascii")
            } else {
                Ok(())
            }
        })?;

        let operand_row = operator_rows.pop().unwrap();

        Ok(WorksheetReader {
            operand_rows: operator_rows,
            operator_row: operand_row,
            pos: len - 1,
            scratch: String::new(),
            finished: false,
        })
    }
}

impl Iterator for WorksheetReader {
    type Item = std::io::Result<(u64, Option<Op>)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            self.scratch.clear();
            for row in self.operand_rows.iter() {
                let c = row.as_bytes()[self.pos] as char;
                if c == ' ' {
                } else if c.is_ascii_digit() {
                    self.scratch.push(c);
                } else {
                    return Some(Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid character '{}' in operand rows", c),
                    )));
                }
            }
            let operand: u64 = self.scratch.parse().unwrap();

            let c = self.operator_row.as_bytes()[self.pos] as char;
            let op = if c == ' ' {
                None
            } else if c == '+' {
                Some(Op::Add)
            } else if c == '*' {
                Some(Op::Mull)
            } else {
                return Some(Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Invalid operator character '{}' in operator row", c),
                )));
            };

            if self.pos == 0 {
                self.finished = true;
            } else {
                self.pos -= 1; // we have read the entire column now
            }

            if op.is_some() && !self.finished {
                self.pos -= 1;
            }

            Some(Ok((operand, op)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> String {
        [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .join("\n")
    }

    #[test]
    fn test_worksheet_reader_creation() {
        let data = test_data();
        let reader = WorksheetReader::new(&data);
        assert!(
            reader.is_ok(),
            "WorksheetReader should be created successfully from test_data"
        );

        let reader = reader.unwrap();
        assert_eq!(reader.operand_rows.len(), 3);
        assert_eq!(reader.pos, 14);
    }

    #[test]
    fn test_iterator_next() {
        let data = test_data();
        let mut reader = WorksheetReader::new(&data).unwrap();

        let first = reader.next();
        assert!(first.is_some(), "Iterator should yield first item");
        let first_item = first.unwrap();
        assert!(first_item.is_ok(), "First item should be Ok");
        let (number, _op) = first_item.unwrap();

        assert_eq!(number, 4, "First number should be 123");

        let (number, _op) = reader.next().unwrap().unwrap();
        assert_eq!(number, 431);

        let (number, _op) = reader.next().unwrap().unwrap();
        assert_eq!(number, 623);
    }
}
