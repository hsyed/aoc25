use std::collections::HashSet;
use std::fmt;

pub fn solve_problem_1(main_file: &str) -> std::io::Result<()> {
    // read the file into a grid and then count the splits and print them out

    let input = std::fs::read_to_string(main_file)?;
    let mut grid = Grid::parse_grid(&input);
    grid.trace_tachyons_down();
    println!("problem 1: tachyons split times: {}", grid.split_count);

    // Implementation goes here
    Ok(())
}

pub fn solve_problem_2(main_file: &str) -> std::io::Result<()> {
    // read the file into a grid and then count the splits and print them out

    let input = std::fs::read_to_string(main_file)?;
    let grid = Grid::parse_grid(&input);
    let path_count = grid.count_all_paths();
    println!("problem 2: all possible tachyon paths: {}", path_count);

    // Implementation goes here
    Ok(())
}

#[derive(Hash, PartialEq, Eq)]
enum Direction {
    Up,
}

enum SquareType {
    Source,
    EmptySpace,
    Splitter,
}

impl SquareType {
    fn to_char(&self) -> char {
        match self {
            SquareType::Source => 'S',
            SquareType::Splitter => '^',
            SquareType::EmptySpace => '.',
        }
    }
}

struct Square {
    square_type: SquareType,
    visited_from: HashSet<Direction>,
}

struct Grid {
    squares: Vec<Vec<Square>>,
    split_count: u32,
}

impl Grid {
    fn parse_grid(input: &str) -> Grid {
        fn parse_square(c: char) -> SquareType {
            match c {
                'S' => SquareType::Source,
                '^' => SquareType::Splitter,
                '.' => SquareType::EmptySpace,
                _ => panic!("Unknown character: {}", c),
            }
        }

        let mut sourches = Vec::new();
        let mut squares = Vec::new();

        for (row_idx, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (col_idx, c) in line.chars().enumerate() {
                let square_type = parse_square(c);
                let mut directions = HashSet::new();

                if matches!(square_type, SquareType::Source) {
                    sourches.push((row_idx, col_idx));
                    directions.insert(Direction::Up);
                }

                row.push(Square {
                    square_type,
                    visited_from: directions,
                });
            }
            squares.push(row);
        }

        Grid {
            squares,
            split_count: 0,
        }
    }

    // trace out the beams downward from the sourches
    fn trace_tachyons_down(&mut self) {
        // this is n^2. but whatever, we could trace down from the sources since it can be captured during
        // parsing.
        for i in 1..self.squares.len() {
            for j in 0..self.squares[i].len() {
                let has_beam_above = self.squares[i - 1][j].visited_from.contains(&Direction::Up);
                if has_beam_above {
                    match self.squares[i][j].square_type {
                        SquareType::EmptySpace => {
                            self.squares[i][j].visited_from.insert(Direction::Up);
                        }
                        SquareType::Splitter => {
                            // I am modelling things as the splitter diverting beams to left and
                            // right, but the go downwards "magically". The splitter doesn't count
                            // as being visited. This can be remodelled later by making the
                            // has_beam_above check more complex.
                            if j == 0 {
                                panic!("a splitter should not be at idx 0")
                            }
                            if !self.squares[i][j - 1].visited_from.contains(&Direction::Up) {
                                self.squares[i][j - 1].visited_from.insert(Direction::Up);
                            }
                            if !self.squares[i][j + 1].visited_from.contains(&Direction::Up) {
                                self.squares[i][j + 1].visited_from.insert(Direction::Up);
                            }

                            self.split_count += 1;
                        }
                        SquareType::Source => {
                            panic!("a source was not expected here")
                        }
                    }
                }
            }
        }
    }

    // notes:
    //  1. it says second-to-last row, but it looks like last row ?
    //  a. it's second to last because the ceil of a range is not inclusive
    //  2. a beam detects a splitter right above it, but it is it's sibblings merge with the
    //     splitter ?
    //  a. the algorithm is not tracing rays bottom up ! The nodes are being visited left to right bottom-up, but not tracing!

    // Count all unique paths using dynamic programming (bottom-up)
    fn count_all_paths(&self) -> u64 {
        let rows = self.squares.len();
        let cols = self.squares[0].len();

        // dp[row][col] = number of paths from (row, col) to bottom
        let mut dp: Vec<Vec<u64>> = vec![vec![0; cols]; rows];

        // Base case: bottom row - each position has exactly 1 path (stay there)
        (0..cols).for_each(|col| {
            dp[rows - 1][col] = 1;
        });

        // Work backwards from second-to-last row to top
        for row in (0..rows - 1).rev() {
            for col in 0..cols {
                let next_row = row + 1;

                // Only compute paths from positions that can be reached
                // (we'll filter by actual reachability later, but for now compute all)
                match self.squares[next_row][col].square_type {
                    SquareType::EmptySpace | SquareType::Source => {
                        // Path continues straight down
                        dp[row][col] = dp[next_row][col];
                    }
                    SquareType::Splitter => {
                        // Path splits into left and right
                        let mut paths = 0;

                        // Right branch
                        if col + 1 < cols {
                            paths += dp[next_row][col + 1];
                        }

                        // Left branch
                        if col > 0 {
                            paths += dp[next_row][col - 1];
                        }

                        dp[row][col] = paths;
                    }
                }
            }
        }

        // Sum up paths from all source positions
        let mut total_paths = 0;
        for (row_idx, row) in self.squares.iter().enumerate() {
            for (col_idx, square) in row.iter().enumerate() {
                if matches!(square.square_type, SquareType::Source) {
                    total_paths += dp[row_idx][col_idx];
                }
            }
        }

        total_paths
    }

    // OLD: Trace all unique paths using explicit stack (no recursion)
    // This is exponential and infeasible for large inputs
    #[allow(dead_code)]
    fn trace_all_paths(&self) -> HashSet<String> {
        let mut all_paths = HashSet::new();

        // Stack holds: (row, col, path_so_far)
        let mut stack: Vec<(usize, usize, Vec<usize>)> = Vec::new();

        // Find all sources and push them onto the stack
        for (row_idx, row) in self.squares.iter().enumerate() {
            for (col_idx, square) in row.iter().enumerate() {
                if matches!(square.square_type, SquareType::Source) {
                    stack.push((row_idx, col_idx, vec![]));
                }
            }
        }

        // Process stack (DFS)
        while let Some((row, col, mut path)) = stack.pop() {
            // Add current column to path
            path.push(col);

            // If we've reached the bottom row, save the path
            if row == self.squares.len() - 1 {
                let path_string = path
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("->");

                all_paths.insert(path_string);
                continue;
            }

            // Check what's below us
            let next_row = row + 1;
            match self.squares[next_row][col].square_type {
                SquareType::EmptySpace | SquareType::Source => {
                    // Continue straight down
                    stack.push((next_row, col, path));
                }
                SquareType::Splitter => {
                    // Split: push both left and right branches onto stack

                    // Push right branch (col + 1)
                    if col + 1 < self.squares[next_row].len() {
                        stack.push((next_row, col + 1, path.clone()));
                    }

                    // Push left branch (col - 1)
                    if col > 0 {
                        stack.push((next_row, col - 1, path));
                    }
                }
            }
        }

        all_paths
    }

    // fn tachyons_in_bottom_row(&self) -> u32 {
    //     self.squares
    //         .last()
    //         .unwrap()
    //         .iter()
    //         .filter(|sq| sq.visited_from.contains(&Direction::Up))
    //         .count() as u32
    // }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.squares {
            for square in row {
                write!(f, "{}", square.square_type.to_char())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_grid_fixture() -> Grid {
        let input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        Grid::parse_grid(input)
    }

    #[test]
    fn test_sample_grid() {
        let mut grid = test_grid_fixture();
        println!("{}", grid);

        grid.trace_tachyons_down();

        // println!(
        //     "After tracing tachyons down: {}",
        //     grid.tachyons_in_bottom_row()
        // );

        println!("tachoyn splits: {}", grid.split_count);
    }

    #[test]
    fn test_all_paths() {
        let grid = test_grid_fixture();
        let paths = grid.trace_all_paths();

        println!("Total unique paths: {}", paths.len());
        for path in &paths {
            println!("  {}", path);
        }
    }
}
