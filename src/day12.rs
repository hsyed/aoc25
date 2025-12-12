use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, one_of, space1},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use rayon::prelude::*;

pub fn solve_problem_1(main_file: &str) -> std::io::Result<()> {
    let input = std::fs::read_to_string(main_file)?;
    let data = parse_input(&input).expect("Failed to parse input");

    let result = data.solve_part1();
    println!("problem 1: result = {}", result);
    Ok(())
}

pub fn solve_problem_2(main_file: &str) -> std::io::Result<()> {
    let input = std::fs::read_to_string(main_file)?;
    let data = parse_input(&input).expect("Failed to parse input");

    let result = data.solve_part2();
    println!("problem 2: result = {}", result);
    Ok(())
}

type Coord = (i32, i32);
type Shape = Vec<Coord>;

#[derive(Debug, Clone)]
struct ProblemData {
    shapes: Vec<Vec<Shape>>, // shapes[shape_idx] = all orientations of that shape
    regions: Vec<Region>,
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    required: Vec<usize>, // required[shape_idx] = count needed
}

// ============ Rotation/Flip Logic ============

fn rotate_90(shape: &Shape) -> Shape {
    shape.iter().map(|(x, y)| (*y, -*x)).collect()
}

fn flip_horizontal(shape: &Shape) -> Shape {
    shape.iter().map(|(x, y)| (-*x, *y)).collect()
}

fn normalize(shape: &Shape) -> Shape {
    let min_x = shape.iter().map(|(x, _)| *x).min().unwrap_or(0);
    let min_y = shape.iter().map(|(_, y)| *y).min().unwrap_or(0);
    shape.iter().map(|(x, y)| (x - min_x, y - min_y)).collect()
}

fn all_orientations(shape: &Shape) -> Vec<Shape> {
    let mut results = Vec::new();
    let mut current = shape.clone();

    for _ in 0..4 {
        results.push(normalize(&current));
        results.push(normalize(&flip_horizontal(&current)));
        current = rotate_90(&current);
    }

    // Deduplicate
    results.sort_by(|a, b| {
        let mut a_vec: Vec<_> = a.iter().collect();
        let mut b_vec: Vec<_> = b.iter().collect();
        a_vec.sort();
        b_vec.sort();
        a_vec.cmp(&b_vec)
    });
    results.dedup_by(|a, b| {
        let mut a_vec: Vec<_> = a.iter().collect();
        let mut b_vec: Vec<_> = b.iter().collect();
        a_vec.sort();
        b_vec.sort();
        a_vec == b_vec
    });

    results
}

// ============ Parsing with nom ============

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_shape_header(input: &str) -> IResult<&str, usize> {
    terminated(parse_number, tuple((char(':'), line_ending)))(input)
}

fn parse_shape_row(input: &str) -> IResult<&str, Vec<bool>> {
    terminated(many1(map(one_of("#."), |c| c == '#')), line_ending)(input)
}

fn parse_shape(input: &str) -> IResult<&str, (usize, Shape)> {
    let (input, idx) = parse_shape_header(input)?;
    let (input, rows) = many1(parse_shape_row)(input)?;

    let mut shape = Vec::new();
    for (y, row) in rows.iter().enumerate() {
        for (x, &filled) in row.iter().enumerate() {
            if filled {
                shape.push((x as i32, y as i32));
            }
        }
    }

    Ok((input, (idx, shape)))
}

fn parse_region(input: &str) -> IResult<&str, Region> {
    let (input, (width, height)) =
        separated_pair(parse_number, char('x'), parse_number)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, required) = separated_list1(space1, parse_number)(input)?;

    Ok((
        input,
        Region {
            width,
            height,
            required,
        },
    ))
}

fn parse_input(input: &str) -> Option<ProblemData> {
    // Split into blocks by double newline
    let blocks: Vec<&str> = input.split("\n\n").collect();

    // Shapes are blocks that start with a digit followed by ':'
    // Regions are blocks that start with digits followed by 'x'
    let mut base_shapes: Vec<Shape> = Vec::new();
    let mut regions: Vec<Region> = Vec::new();

    for block in blocks {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        // Check if this looks like a shape (starts with "N:")
        if block.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
            && block.contains(':')
            && block.contains('#')
        {
            // Parse as shape
            let input_with_newline = format!("{}\n", block);
            if let Ok((_, (idx, shape))) = parse_shape(&input_with_newline) {
                // Ensure we have space for this index
                while base_shapes.len() <= idx {
                    base_shapes.push(Vec::new());
                }
                base_shapes[idx] = shape;
            }
        } else if block.contains('x') && block.contains(':') {
            // Parse as region(s) - might be multiple lines
            for line in block.lines() {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Ok((_, region)) = parse_region(line) {
                    regions.push(region);
                }
            }
        }
    }

    if base_shapes.is_empty() || regions.is_empty() {
        return None;
    }

    // Precompute all orientations for each shape
    let shapes: Vec<Vec<Shape>> = base_shapes.iter().map(|s| all_orientations(s)).collect();

    Some(ProblemData { shapes, regions })
}

// ============ Solver ============

struct Solver {
    width: i32,
    height: i32,
    grid: Vec<bool>,
    shapes: Vec<Vec<Shape>>,
}

impl Solver {
    fn new(width: usize, height: usize, shapes: &[Vec<Shape>]) -> Self {
        Self {
            width: width as i32,
            height: height as i32,
            grid: vec![false; width * height],
            shapes: shapes.to_vec(),
        }
    }

    #[inline(always)]
    fn can_place(&self, shape: &Shape, ox: i32, oy: i32) -> bool {
        for &(sx, sy) in shape {
            let x = ox + sx;
            let y = oy + sy;
            if x < 0 || y < 0 || x >= self.width || y >= self.height {
                return false;
            }
            let idx = (y * self.width + x) as usize;
            if self.grid[idx] {
                return false;
            }
        }
        true
    }

    #[inline(always)]
    fn place(&mut self, shape: &Shape, ox: i32, oy: i32) {
        for &(sx, sy) in shape {
            let idx = ((oy + sy) * self.width + (ox + sx)) as usize;
            self.grid[idx] = true;
        }
    }

    #[inline(always)]
    fn remove(&mut self, shape: &Shape, ox: i32, oy: i32) {
        for &(sx, sy) in shape {
            let idx = ((oy + sy) * self.width + (ox + sx)) as usize;
            self.grid[idx] = false;
        }
    }

    fn empty_cells(&self) -> usize {
        self.grid.iter().filter(|&&x| !x).count()
    }

    /// Count placements for a shape - inlined and optimized
    #[inline]
    fn count_placements(&self, shape_idx: usize, max_count: usize) -> usize {
        let mut count = 0;
        for orientation in &self.shapes[shape_idx] {
            for oy in 0..self.height {
                for ox in 0..self.width {
                    if self.can_place(orientation, ox, oy) {
                        count += 1;
                        if count > max_count {
                            return count;
                        }
                    }
                }
            }
        }
        count
    }

    /// Solve with most-constrained-variable heuristic
    fn solve(&mut self, to_place: &mut [(usize, usize)]) -> bool {
        // Check if all shapes placed
        let total_remaining: usize = to_place.iter().map(|(_, c)| *c).sum();
        if total_remaining == 0 {
            return true;
        }

        // Early pruning: check if we have enough space
        let empty = self.empty_cells();
        let cells_needed: usize = to_place
            .iter()
            .filter(|(_, c)| *c > 0)
            .map(|(shape_idx, count)| self.shapes[*shape_idx][0].len() * count)
            .sum();
        if cells_needed > empty {
            return false;
        }

        // Find the shape type with the LEAST valid placements (most constrained)
        let mut best_idx = None;
        let mut min_placements = usize::MAX;

        for idx in 0..to_place.len() {
            if to_place[idx].1 == 0 {
                continue;
            }

            let shape_idx = to_place[idx].0;
            let placement_count = self.count_placements(shape_idx, min_placements);

            // Early exit: if a shape has no valid placements, fail immediately
            if placement_count == 0 {
                return false;
            }

            if placement_count < min_placements {
                min_placements = placement_count;
                best_idx = Some(idx);

                // If we found a very constrained shape (only 1 placement), use it immediately
                if min_placements == 1 {
                    break;
                }
            }
        }

        let idx = match best_idx {
            Some(i) => i,
            None => return true, // No shapes left to place
        };

        let shape_idx = to_place[idx].0;
        let orientations = self.shapes[shape_idx].clone();

        // Try each orientation at each position
        for orientation in &orientations {
            for oy in 0..self.height {
                for ox in 0..self.width {
                    if self.can_place(orientation, ox, oy) {
                        self.place(orientation, ox, oy);
                        to_place[idx].1 -= 1;

                        if self.solve(to_place) {
                            return true;
                        }

                        to_place[idx].1 += 1;
                        self.remove(orientation, ox, oy);
                    }
                }
            }
        }

        false
    }
}

fn can_fit_region(region: &Region, shapes: &[Vec<Shape>]) -> bool {
    let mut solver = Solver::new(region.width, region.height, shapes);

    // Build to_place list
    let mut to_place: Vec<(usize, usize)> = region
        .required
        .iter()
        .enumerate()
        .map(|(idx, &count)| (idx, count))
        .collect();

    solver.solve(&mut to_place)
}

impl ProblemData {
    fn solve_part1(&self) -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};

        let count = AtomicU64::new(0);
        let prog = AtomicU64::new(0);
        let total = self.regions.len();

        self.regions.par_iter().for_each(|region| {
            if can_fit_region(region, &self.shapes) {
                count.fetch_add(1, Ordering::Relaxed);
            }
            let idx = prog.fetch_add(1, Ordering::Relaxed);
            if  idx % 50 == 0 {
                eprintln!("Processed {}/{} regions...", idx, total);
            }
        });

        count.load(Ordering::Relaxed)
    }

    fn solve_part2(&self) -> u64 {
        // Part 2 not yet defined
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;

    #[test]
    fn test_sample_problem_1() {
        let data = parse_input(TEST_INPUT).unwrap();
        eprintln!("Parsed {} shapes, {} regions", data.shapes.len(), data.regions.len());
        for (i, shape_orientations) in data.shapes.iter().enumerate() {
            eprintln!("Shape {}: {} cells, {} orientations", i, shape_orientations[0].len(), shape_orientations.len());
        }
        for (i, region) in data.regions.iter().enumerate() {
            eprintln!("Region {}: {}x{}, required: {:?}", i, region.width, region.height, region.required);
        }
        assert_eq!(data.solve_part1(), 2);
    }

    #[test]
    fn test_rotations() {
        let shape = vec![(0, 0), (1, 0), (0, 1)];

        let orientations = all_orientations(&shape);
        // L-shape should have 4 unique orientations
        assert_eq!(orientations.len(), 4);
    }

    #[test]
    fn test_parse_shape() {
        let input = "0:\n###\n##.\n##.\n";
        let (_, (idx, shape)) = parse_shape(input).unwrap();
        assert_eq!(idx, 0);
        assert_eq!(shape.len(), 7); // 3 + 2 + 2 = 7 cells
    }

    #[test]
    fn test_simple_4x4() {
        // Test: 4x4 with 2 shapes of type 0 (the U-shape)
        // Shape 0:
        // ###
        // #..
        // ###
        // Two of these should fit in 4x4 (16 cells, 14 used)
        let input = r#"0:
###
#..
###

4x4: 2"#;
        let data = parse_input(input).unwrap();
        assert_eq!(data.shapes.len(), 1);
        assert_eq!(data.regions.len(), 1);

        // Visualize orientations
        eprintln!("\nShape 0 orientations:");
        for (i, orientation) in data.shapes[0].iter().enumerate() {
            eprintln!("Orientation {}:", i);
            let max_x = orientation.iter().map(|(x, _)| *x).max().unwrap_or(0);
            let max_y = orientation.iter().map(|(_, y)| *y).max().unwrap_or(0);
            for y in 0..=max_y {
                for x in 0..=max_x {
                    if orientation.contains(&(x, y)) {
                        eprint!("#");
                    } else {
                        eprint!(".");
                    }
                }
                eprintln!();
            }
            eprintln!();
        }

        assert!(can_fit_region(&data.regions[0], &data.shapes));
    }
}