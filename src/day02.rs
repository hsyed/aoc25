use nom::{
    IResult,
    character::complete::{char, digit1, multispace0},
    combinator::map_res,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
};

pub fn solve_problem_1(main_file: &str) -> std::io::Result<()> {
    solve_problem_with(main_file, 1, is_twice_repeated)
}

pub fn solve_problem_2(main_file: &str) -> std::io::Result<()> {
    solve_problem_with(main_file, 2, |n| smallest_repeating_pattern(n).is_some())
}

pub fn solve_problem_with<F>(main_file: &str, num: i32, f: F) -> std::io::Result<()>
where
    F: Fn(u64) -> bool,
{
    let input = std::fs::read_to_string(main_file)?;

    let (_, ranges) = parse_ranges(&input)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;

    let sum: u64 = ranges
        .iter()
        .flat_map(|(low, high)| collect_invalid_numbers(*low, *high, &f))
        .sum();

    println!("problem {}: Sum of all invalid numbers: {}", num, sum);

    Ok(())
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    fn parse_u64(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(input)
    }

    fn parse_range(input: &str) -> IResult<&str, (u64, u64)> {
        separated_pair(parse_u64, char('-'), parse_u64)(input)
    }

    separated_list1(terminated(char(','), multispace0), parse_range)(input)
}

// coiunts the number of digits in n
fn count_digits(n: u64) -> u32 {
    let mut temp = n;
    let mut count = 0;
    while temp > 0 {
        count += 1;
        temp /= 10;
    }
    count
}

// determines the divisors of n
fn divisors(n: u32) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    let sqrt = (n as f32).sqrt() as u32;
    let mut divs = vec![1];

    for i in 2..=sqrt {
        if n.is_multiple_of(i) {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
    }

    divs.sort_unstable();
    divs
}

fn is_twice_repeated(n: u64) -> bool {
    let digit_count = count_digits(n);

    // Must be even number of digits
    if !digit_count.is_multiple_of(2) {
        return false;
    }

    is_pattern_repeating(n, digit_count / 2).is_some()
}

// finds the smallest pattern of repeating digits in n, if any
fn smallest_repeating_pattern(n: u64) -> Option<u64> {
    let digit_count = count_digits(n);

    // Check all possible pattern lengths (divisors of digit_count)
    for pattern_length in divisors(digit_count) {
        if pattern_length == digit_count {
            continue; // Skip the full length (no repetition)
        }

        if let Some(pattern) = is_pattern_repeating(n, pattern_length) {
            return Some(pattern);
        }
    }

    None
}

fn is_pattern_repeating(n: u64, pattern_length: u32) -> Option<u64> {
    let divisor = 10_u64.pow(pattern_length);
    let pattern = n % divisor;

    let mut remaining = n / divisor;

    while remaining > 0 {
        if remaining % divisor != pattern {
            return None;
        }
        remaining /= divisor;
    }

    Some(pattern)
}

fn collect_invalid_numbers<F>(low: u64, high: u64, f: F) -> Vec<u64>
where
    F: Fn(u64) -> bool,
{
    (low..=high).filter(|n| f(*n)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn test_data() -> [(u64, u64, &'static [u64]); 8] {
        [
            (11, 22, &[11, 22]),                     // has two invalid IDs
            (95, 115, &[99]),                        // has one invalid ID
            (998, 1012, &[1010]),                    // has one invalid ID
            (1188511880, 1188511890, &[1188511885]), // has one invalid ID
            (222220, 222224, &[222222]),             // has one invalid ID
            (1698522, 1698528, &[]),                 // contains no invalid IDs
            (446443, 446449, &[446446]),             // has one invalid ID
            (38593856, 38593862, &[38593859]),       // has one invalid ID
        ]
    }

    #[test]
    fn test_is_twice_repeated() {
        // Should return true for repeating sequences
        assert!(is_twice_repeated(123123));
        assert!(is_twice_repeated(1188511885));
        assert!(is_twice_repeated(1212));
        assert!(is_twice_repeated(11));
        assert!(is_twice_repeated(00)); // Edge case: 0

        // Should return false for non-repeating sequences
        assert!(!is_twice_repeated(123456));
        assert!(!is_twice_repeated(1234));
        assert!(!is_twice_repeated(123)); // Odd number of digits
        assert!(!is_twice_repeated(12));
    }

    #[test]
    fn test_collect_invalid_numbers() {
        for (low, high, expected) in test_data() {
            assert_eq!(
                collect_invalid_numbers(low, high, is_twice_repeated),
                expected
            );
        }
    }

    #[test]
    fn validate_test_data() {
        let mut acc: u64 = 0;
        for (low, high, _expected) in test_data() {
            acc += collect_invalid_numbers(low, high, is_twice_repeated)
                .iter()
                .sum::<u64>();
        }
        assert_eq!(acc, 1227775554);
    }

    #[test]
    fn test_parse_ranges() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862";
        let result = parse_ranges(input);
        assert!(result.is_ok());

        let (remaining, ranges) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(ranges.len(), 8);
        assert_eq!(ranges[0], (11, 22));
        assert_eq!(ranges[1], (95, 115));
        assert_eq!(ranges[2], (998, 1012));
        assert_eq!(ranges[3], (1188511880, 1188511890));
        assert_eq!(ranges[4], (222220, 222224));
        assert_eq!(ranges[5], (1698522, 1698528));
        assert_eq!(ranges[6], (446443, 446449));
        assert_eq!(ranges[7], (38593856, 38593862));
    }

    #[test]
    fn test_has_repeating_pattern() {
        // Should return Some(pattern) for repeating patterns
        assert_eq!(smallest_repeating_pattern(123123), Some(123)); // pattern "123" x2
        assert_eq!(smallest_repeating_pattern(12121212), Some(12)); // pattern "12" x4
        assert_eq!(smallest_repeating_pattern(1212), Some(12)); // pattern "12" x2
        assert_eq!(smallest_repeating_pattern(123412341234), Some(1234)); // pattern "1234" x3
        assert_eq!(smallest_repeating_pattern(111111), Some(1)); // pattern "1" x6
        assert_eq!(smallest_repeating_pattern(11), Some(1)); // pattern "1" x2
        assert_eq!(smallest_repeating_pattern(1188511885), Some(11885)); // pattern "11885" x2

        // Should return None for non-repeating patterns
        assert_eq!(smallest_repeating_pattern(123456), None);
        assert_eq!(smallest_repeating_pattern(1234), None);
        assert_eq!(smallest_repeating_pattern(123), None);
        assert_eq!(smallest_repeating_pattern(12345678), None);
    }

    #[test]
    fn test_divisors() {
        assert_eq!(divisors(20), vec![1, 2, 4, 5, 10]);
        assert_eq!(divisors(30), vec![1, 2, 3, 5, 6, 10, 15]);
        assert_eq!(divisors(36), vec![1, 2, 3, 4, 6, 9, 12, 18]);
        assert_eq!(divisors(1), vec![1]);
        assert_eq!(divisors(0), vec![]);
    }
}
