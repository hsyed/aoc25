use nom::{
    IResult,
    character::complete::{char, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
};
use rangemap::RangeSet;
use std::fs;

pub fn solve_problem_1(main_file: &str) -> std::io::Result<()> {
    let db = read_database_file(main_file)?;
    println!(
        "problem 1: fresh ingredients in database: {}",
        db.current_fresh_ingredient_count()
    );
    Ok(())
}

pub fn solve_problem_2(main_file: &str) -> std::io::Result<()> {
    let db = read_database_file(main_file)?;
    println!(
        "problem 2: count of unique fresh ingredient ids in the ranges: {}",
        db.unique_fresh_ingredient_count()
    );
    Ok(())
}

struct Database {
    fresh_ranges: RangeSet<u64>,
    ingredients: Vec<u64>,
}

impl Database {
    fn from(ranges: Vec<(u64, u64)>, values: Vec<u64>) -> Self {
        let mut rs = RangeSet::new();
        for range in ranges {
            let r = range.0..range.1 + 1;
            rs.insert(r);
        }
        Database {
            fresh_ranges: rs,
            ingredients: values,
        }
    }

    fn is_fresh(&self, ingredient: &u64) -> bool {
        self.fresh_ranges.contains(ingredient)
    }

    fn current_fresh_ingredient_count(&self) -> u64 {
        self.ingredients
            .iter()
            .filter(|id| self.is_fresh(id))
            .count() as u64
    }

    fn unique_fresh_ingredient_count(&self) -> u64 {
        self.fresh_ranges.iter().map(|r| r.end - r.start + 1).sum()
    }
}

// TODO implement a streaming parser
fn parse_database(input: &str) -> IResult<&str, Database> {
    fn parse_u64(input: &str) -> IResult<&str, u64> {
        map_res(digit1, str::parse)(input)
    }

    fn parse_range(input: &str) -> IResult<&str, (u64, u64)> {
        separated_pair(parse_u64, char('-'), parse_u64)(input)
    }
    let (input, ranges) = separated_list1(line_ending, parse_range)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, ingredients) = separated_list1(line_ending, parse_u64)(input)?;

    Ok((input, Database::from(ranges, ingredients)))
}

fn read_database_file(file_path: &str) -> std::io::Result<Database> {
    let content = fs::read_to_string(file_path)?;
    parse_database(&content)
        .map(|(_, db)| db)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_fresh_ranges() {
        let db = Database::from(vec![(3, 5)], vec![]);

        assert!(db.is_fresh(&3), "3 should be fresh");
        assert!(db.is_fresh(&4), "4 should be fresh");
        assert!(db.is_fresh(&5), "5 should be fresh");
        assert!(!db.is_fresh(&2), "2 should not be fresh");
        assert!(!db.is_fresh(&6), "6 should not be fresh");
    }

    #[test]
    fn test_parse_database() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (remaining, db) = parse_database(input).unwrap();

        assert_eq!(remaining, "", "Should consume entire input");
        assert_eq!(db.ingredients, vec![1, 5, 8, 11, 17, 32]);

        assert!(db.is_fresh(&5), "5 should be fresh (in range 3-5)");
        assert!(db.is_fresh(&11), "11 should be fresh (in range 10-14)");
        assert!(
            db.is_fresh(&17),
            "17 should be fresh (in range 16-20 and 12-18)"
        );
        assert!(!db.is_fresh(&1), "1 should not be fresh");
        assert!(!db.is_fresh(&8), "8 should not be fresh");
        assert!(!db.is_fresh(&32), "32 should not be fresh");
    }
}
