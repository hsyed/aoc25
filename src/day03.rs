use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_problem_1(main_file: &str) -> std::io::Result<()> {
    solve_problem(main_file, find_joltage_2)
}

pub(crate) fn solve_problem_2(main_file: &str) -> std::io::Result<()> {
    solve_problem(main_file, |inp| find_joltage_k(inp, 12))
}

fn solve_problem<F>(main_file: &str, find_joltage: F) -> std::io::Result<()>
where
    F: Fn(&[u8]) -> u64,
{
    let file = File::open(main_file)?;
    let reader = BufReader::new(file);
    let res: u64 = reader
        .lines()
        .map(|line| {
            let digits = input_from_str(line.unwrap().as_str()).expect("invalid line");
            find_joltage(&digits)
        })
        .sum();

    println!("problem 1: the total joltage is {}", res);

    Ok(())
}

fn build_suffix_max_array(digits: &[u8]) -> Vec<u8> {
    let mut max_right: Vec<u8> = vec![0; digits.len()];
    max_right[digits.len() - 1] = digits[digits.len() - 1];
    for i in (0..digits.len() - 1).rev() {
        max_right[i] = max_right[i + 1].max(digits[i]);
    }
    max_right
}

fn input_from_str(input: &str) -> Result<Vec<u8>, String> {
    if input.len() < 2 {
        return Err("Input too short, need at least 2 characters".to_string());
    }

    input
        .bytes()
        .enumerate()
        .try_fold(Vec::with_capacity(input.len()), |mut acc, (i, b)| {
            if b.is_ascii_digit() {
                acc.push(b - b'0');
                Ok(acc)
            } else {
                Err(format!(
                    "Invalid character '{}' at position {}, expect 0-9 digits only",
                    b as char, i
                ))
            }
        })
}

fn find_joltage_2(digits: &[u8]) -> u64 {
    let max_right = build_suffix_max_array(digits);

    // Find best pair by checking each left position with the max to its right
    let mut best_pair = (digits[0], max_right[1]);
    for i in 1..digits.len() - 1 {
        let candidate = (digits[i], max_right[i + 1]);
        if candidate.0 > best_pair.0 || (candidate.0 == best_pair.0 && candidate.1 > best_pair.1) {
            best_pair = candidate;
        }
    }

    best_pair.0 as u64 * 10 + best_pair.1 as u64
}

fn find_joltage_k(digits: &[u8], k: usize) -> u64 {
    if k > digits.len() {
        panic!(
            "k ({}) cannot be greater than digits length ({})",
            k,
            digits.len()
        );
    }

    let mut result = Vec::with_capacity(k);
    let mut to_skip = digits.len() - k; // how many digits we can afford to skip

    for &digit in digits {
        // Remove smaller digits from result if we still have room to skip
        while !result.is_empty() && result.last().unwrap() < &digit && to_skip > 0 {
            result.pop();
            to_skip -= 1;
        }
        result.push(digit);
    }

    // Truncate to exactly k digits
    result.truncate(k);

    // Convert to number
    result.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_suffix_array() {
        let test_cases = [
            ("123", vec![3, 3, 3]),
            ("987654321", vec![9, 8, 7, 6, 5, 4, 3, 2, 1]),
            ("54321678", vec![8, 8, 8, 8, 8, 8, 8, 8]),
            ("5555", vec![5, 5, 5, 5]),
            (
                "987654321111111",
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            ),
            (
                "811111111111119",
                vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
            ),
            (
                "234234234234278",
                vec![8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8],
            ),
            (
                "818181911112111",
                vec![9, 9, 9, 9, 9, 9, 9, 2, 2, 2, 2, 2, 1, 1, 1],
            ),
        ];

        for (input, expected) in test_cases {
            let digits = input_from_str(input).unwrap();
            let result = build_suffix_max_array(&digits);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_joltage_examples() {
        let test_cases = vec![
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
        ];

        for (input, expected_joltage) in test_cases {
            let digits = input_from_str(input).expect("should parse input");
            let joltage = find_joltage_2(&digits);
            assert_eq!(
                joltage, expected_joltage,
                "Failed for input: {}, expected: {}, got: {}",
                input, expected_joltage, joltage
            );
        }
    }

    #[test]
    fn test_joltage_k() {
        let test_cases = vec![
            // (input, k, expected)
            ("5432111119871", 4, 9871),
            ("5432111119871", 5, 59871),
            ("987654321", 3, 987),
            ("987654321", 5, 98765),
            ("811111111111119", 3, 819),
            ("811111111111119", 12, 811111111119),
            ("123456789", 5, 56789),
            ("999888777", 5, 99988),
            ("111119", 3, 119),
            ("987654321111111", 12, 987654321111),
            ("35355591115431111", 6, 954311),
        ];

        for (input, k, expected) in test_cases {
            let digits = input_from_str(input).expect("should parse input");
            let result = find_joltage_k(&digits, k);
            assert_eq!(
                result, expected,
                "Failed for input: {}, k: {}, expected: {}, got: {}",
                input, k, expected, result
            );
        }
    }

    #[test]
    fn test_joltage_k_edge_cases() {
        // Test k = 1 (should get max single digit)
        let digits = input_from_str("123456789").unwrap();
        assert_eq!(find_joltage_k(&digits, 1), 9);

        // Test k = length (should get all digits)
        let digits = input_from_str("54321").unwrap();
        assert_eq!(find_joltage_k(&digits, 5), 54321);

        // Test with all same digits
        let digits = input_from_str("5555").unwrap();
        assert_eq!(find_joltage_k(&digits, 2), 55);
    }
}
