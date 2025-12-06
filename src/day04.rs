use nom::IResult;
use nom::character::complete::one_of;
use nom::multi::many1;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve_problem_1(main_file: &str) -> std::io::Result<()> {
    let mut floor = read_floor_file(main_file)?;
    floor.process_accessible();
    println!(
        "problem 1: accessible tiles on floor: {}",
        floor.accessible_slots.len()
    );
    Ok(())
}

pub fn solve_problem_2(main_file: &str) -> std::io::Result<()> {
    let mut floor = read_floor_file(main_file)?;
    let mut tally = 0;
    while floor.process_accessible() > 0 {
        tally += floor.accessible_slots.len();
        floor.clear_acessible();
    }
    print!("problem 2: {} rolls of paper were removed", tally);
    Ok(())
}

fn read_floor_file(main_file: &str) -> std::io::Result<Floor> {
    let file = File::open(main_file)?;
    let reader = BufReader::new(file);

    let floor_items = reader
        .lines()
        .map(|l| parse_floor_line(l.unwrap().as_str()).unwrap().1)
        .collect();

    let floor = Floor {
        tiles: floor_items,
        accessible_slots: Vec::new(),
    };
    Ok(floor)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    EmptySpace,
    PaperRoll {
        accessible: Option<bool>, /* if present indicates whether the roll is accessible*/
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Floor {
    tiles: Vec<Vec<Tile>>,
    accessible_slots: Vec<(usize, usize)>,
}

impl Floor {
    fn get(&self, row: usize, col: usize) -> Option<&Tile> {
        self.tiles.get(row).and_then(|r| r.get(col))
    }

    fn iter_neighbours(
        &self,
        row: usize,
        col: usize,
    ) -> impl Iterator<Item = ((usize, usize), Tile)> + '_ {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        directions.into_iter().filter_map(move |(dr, dc)| {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0 && new_col >= 0 {
                self.get(new_row as usize, new_col as usize)
                    .map(|tile| ((new_row as usize, new_col as usize), *tile))
            } else {
                None
            }
        })
    }

    fn process_accessible(&mut self) -> u32 {
        self.accessible_slots.clear();

        for row in 0..self.tiles.len() {
            for col in 0..self.tiles[row].len() {
                if matches!(self.tiles[row][col], Tile::PaperRoll { accessible: _ }) {
                    let non_empty_neighbours = self
                        .iter_neighbours(row, col)
                        .filter(|(_, inner_tile)| *inner_tile != Tile::EmptySpace)
                        .count();

                    self.tiles[row][col] = if non_empty_neighbours < 4 {
                        self.accessible_slots.push((row, col));
                        Tile::PaperRoll {
                            accessible: Some(true),
                        }
                    } else {
                        Tile::PaperRoll {
                            accessible: Some(false),
                        }
                    }
                }
            }
        }
        self.accessible_slots.len() as u32
    }

    fn clear_acessible(&mut self) {
        for (row, col) in &self.accessible_slots {
            self.tiles[*row][*col] = Tile::EmptySpace;
        }
        self.accessible_slots.clear();
    }
}

impl fmt::Display for Floor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.tiles {
            for tile in row {
                let c = match tile {
                    Tile::EmptySpace => '.',
                    Tile::PaperRoll {
                        accessible: Some(true),
                    } => 'x',
                    Tile::PaperRoll { accessible: _ } => '@',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_floor_line(input: &str) -> IResult<&str, Vec<Tile>> {
    fn parse_tile(input: &str) -> IResult<&str, Tile> {
        let (input, c) = one_of(".@x")(input)?;
        let tile = match c {
            '.' => Tile::EmptySpace,
            '@' => Tile::PaperRoll { accessible: None },
            'x' => Tile::PaperRoll {
                accessible: Some(true),
            },
            _ => unreachable!(),
        };
        Ok((input, tile))
    }
    many1(parse_tile)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Vec<Tile>> {
        [
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
        .iter()
        .map(|s| parse_floor_line(s).unwrap().1)
        .collect()
    }

    #[test]
    fn test_parse_floor_lines() {
        let test_cases = vec![(
            "..@x",
            vec![
                Tile::EmptySpace,
                Tile::EmptySpace,
                Tile::PaperRoll { accessible: None },
                Tile::PaperRoll {
                    accessible: Some(true),
                },
            ],
        )];

        for (input, expected) in test_cases {
            let (remaining, tiles) = parse_floor_line(input).unwrap();
            assert_eq!(remaining, "", "Should consume entire input");
            assert_eq!(tiles, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_iter_neighbours_boundary_conditions() {
        let tiles = test_data();
        let mut floor = Floor {
            tiles,
            accessible_slots: Vec::new(),
        };
        let max_row = floor.tiles.len() - 1;
        let max_col = floor.tiles[0].len() - 1;

        // Test top-left corner (0, 0)
        let neighbours_top_left: Vec<_> = floor.iter_neighbours(0, 0).collect();
        let expected_top_left = vec![
            ((0, 1), Tile::EmptySpace),
            ((1, 0), Tile::PaperRoll { accessible: None }),
            ((1, 1), Tile::PaperRoll { accessible: None }),
        ];
        assert_eq!(
            neighbours_top_left, expected_top_left,
            "Top-left corner neighbours mismatch"
        );

        // Test bottom-right corner (max_row, max_col) = (9, 9)
        let neighbours_bottom_right: Vec<_> = floor.iter_neighbours(max_row, max_col).collect();
        let expected_bottom_right = vec![
            ((8, 8), Tile::PaperRoll { accessible: None }),
            ((8, 9), Tile::EmptySpace),
            ((9, 8), Tile::PaperRoll { accessible: None }),
        ];
        assert_eq!(
            neighbours_bottom_right, expected_bottom_right,
            "Bottom-right corner neighbours mismatch"
        );

        // Test top-right corner (0, max_col) = (0, 9)
        let neighbours_top_right: Vec<_> = floor.iter_neighbours(0, max_col).collect();
        let expected_top_right = vec![
            ((0, 8), Tile::PaperRoll { accessible: None }),
            ((1, 8), Tile::PaperRoll { accessible: None }),
            ((1, 9), Tile::PaperRoll { accessible: None }),
        ];
        assert_eq!(
            neighbours_top_right, expected_top_right,
            "Top-right corner neighbours mismatch"
        );

        // Test bottom-left corner (max_row, 0) = (9, 0)
        let neighbours_bottom_left: Vec<_> = floor.iter_neighbours(max_row, 0).collect();
        let expected_bottom_left = vec![
            ((8, 0), Tile::EmptySpace),
            ((8, 1), Tile::PaperRoll { accessible: None }),
            ((9, 1), Tile::EmptySpace),
        ];
        assert_eq!(
            neighbours_bottom_left, expected_bottom_left,
            "Bottom-left corner neighbours mismatch"
        );

        // Test interior cell (1, 1) - should have all 8 neighbours
        let neighbours_interior: Vec<_> = floor.iter_neighbours(1, 1).collect();
        let expected_interior = vec![
            ((0, 0), Tile::EmptySpace),
            ((0, 1), Tile::EmptySpace),
            ((0, 2), Tile::PaperRoll { accessible: None }),
            ((1, 0), Tile::PaperRoll { accessible: None }),
            ((1, 2), Tile::PaperRoll { accessible: None }),
            ((2, 0), Tile::PaperRoll { accessible: None }),
            ((2, 1), Tile::PaperRoll { accessible: None }),
            ((2, 2), Tile::PaperRoll { accessible: None }),
        ];
        assert_eq!(
            neighbours_interior, expected_interior,
            "Interior cell (1,1) neighbours mismatch"
        );

        floor.process_accessible();
        assert_eq!(floor.accessible_slots.len(), 13);
        floor.clear_acessible();
        println!("{}", floor);

        floor.process_accessible();
        assert_eq!(floor.accessible_slots.len(), 12);
        println!("Remove 12 rolls of paper:\n{}", floor);
        floor.clear_acessible();

        floor.process_accessible();
        assert_eq!(floor.accessible_slots.len(), 7);
        println!("Remove 7 rolls of paper:\n{}", floor);
    }
}
