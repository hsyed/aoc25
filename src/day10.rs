use bitvec::macros::internal::funty::Fundamental;
use bitvec::vec::BitVec;
use good_lp::*;
use indexmap::IndexSet;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{char, space1, u32};
use nom::multi::{fold_many1, separated_list0, separated_list1};
use nom::sequence::delimited;
use nom::{IResult, Parser};
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

// TODO refactor this!
pub fn solve_problem_1_dfs(main_file: &str) -> std::io::Result<()> {
    let input = std::fs::read_to_string(main_file)?;

    let data: ProblemData = input.parse().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Parse error: {}", e),
        )
    })?;

    let result: u32 = data
        .lines
        .iter()
        .map(|line| {
            println!("Line: {:?}", line);
            let res = line
                .find_solutions()
                .iter()
                .map(|s| s.len() as u32)
                .min()
                .unwrap_or(0);
            println!("{:?}", res);
            res
        })
        .sum();

    println!("problem 1: result = {}", result);
    Ok(())
}

pub fn solve_problem_1_iterative_deepening(main_file: &str) -> std::io::Result<()> {
    let input = std::fs::read_to_string(main_file)?;

    let data: ProblemData = input.parse().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Parse error: {}", e),
        )
    })?;

    let result: u32 = data
        .lines
        .iter()
        .map(|line| {
            println!("Line: {:?}", line);
            let res = line.find_optimal_solution().len();
            println!("{:?}", res);
            res as u32
        })
        .sum();

    println!("problem 1: result = {}", result);
    Ok(())
}

pub fn solve_problem_2_linear_programming(main_file: &str) -> std::io::Result<()> {
    let input = std::fs::read_to_string(main_file)?;

    let data: ProblemData = input.parse().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Parse error: {}", e),
        )
    })?;

    let result: u32 = data
        .lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            println!("Line {}: target = {:?}", i + 1, line.joltage_requirements);
            let presses = find_minimum_presses_ilp(
                &line.joltage_requirements,
                &line.wiring_schematics,
            )
            .unwrap_or_else(|| {
                eprintln!("Warning: No solution found for line {}", i + 1);
                0
            });
            presses
        })
        .sum();

    println!("problem 2: result = {}", result);
    Ok(())
}

// dfs expands a problem by searching maximally deeply into the search space
// bfs expands a problem by searching breadth-wise through the search space, expand the breadth of
// each level before going deeper. Especially useful when one or more exist at a shallow depth.

// bfs implementation for problem 2
fn find_minimum_presses(target: &Vec<u32>, wiring_schematics: &[Vec<u32>]) -> Option<u32> {
    // BFS: Find shortest path from [0,0,..] to target
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    {
        let initial_state = vec![0u32; target.len()];

        // terminate on an all 0 target
        if initial_state == *target {
            return Some(0);
        }

        queue.push_back((initial_state.clone(), 0u32)); // (counter_state, total_presses)
        visited.insert(initial_state);
    }

    let mut iterations = 0;
    while let Some((state, presses)) = queue.pop_front() {
        iterations += 1;
        if iterations % 100000 == 0 {
            eprintln!("Iterations: {}, Queue size: {}, Visited: {}, Depth: {}",
                     iterations, queue.len(), visited.len(), presses);
        }
        // Try pressing each button
        for positions in wiring_schematics.iter() {
            let mut new_state = state.clone();
            let mut valid = true;

            // Increment counters affected by this button
            for &pos in positions {
                new_state[pos as usize] += 1;
                // Prune if we exceed target (can't undo increments)
                if new_state[pos as usize] > target[pos as usize] {
                    valid = false;
                    break;
                }
            }

            // Where we are now could have been achieved in less button presses. So we skip it if
            // we've seen it before
            if valid && visited.insert(new_state.clone()) {
                if new_state == *target {
                    return Some(presses + 1); // Found the solution!
                }

                queue.push_back((new_state, presses + 1));
            }
        }
    }

    None // No solution found
}

// Integer Linear Programming approach using good_lp with microlp solver
// Much cleaner than float-based linear algebra!
fn find_minimum_presses_ilp(target: &[u32], wiring_schematics: &[Vec<u32>]) -> Option<u32> {
    use good_lp::{variables, constraint, SolverModel, Solution, microlp};

    let num_buttons = wiring_schematics.len();

    // Create integer variables for button presses (one per button)
    let mut vars = variables!();
    let button_presses: Vec<_> = (0..num_buttons)
        .map(|i| vars.add(variable().integer().min(0).name(format!("button_{}", i))))
        .collect();

    // Build the problem: minimize sum of all button presses, using solver FIRST
    let mut problem = vars.minimise(button_presses.iter().sum::<Expression>())
        .using(microlp);

    // Add constraints AFTER using() but BEFORE solve()
    // For each counter, sum of button presses affecting it must equal target
    for (counter_idx, &target_val) in target.iter().enumerate() {
        let affecting_buttons: Expression = wiring_schematics
            .iter()
            .enumerate()
            .filter_map(|(button_idx, positions)| {
                if positions.contains(&(counter_idx as u32)) {
                    Some(button_presses[button_idx])
                } else {
                    None
                }
            })
            .sum();

        // Chain .with() to add each constraint
        problem = problem.with(constraint!(affecting_buttons == target_val as i32));
    }

    // Solve!
    match problem.solve() {
        Ok(solution) => {
            let mut total_presses = 0u32;
            for &var in button_presses.iter() {
                let raw_value = solution.value(var);
                let presses = raw_value.round() as u32;  // ROUND instead of truncate!
                total_presses += presses;
            }

            Some(total_presses)
        }
        Err(e) => {
            eprintln!("❌ ILP solver failed: {:?}", e);
            None
        }
    }
}

// dead code from a dfs implementation

// struct State <'a> {
//     j_tally: Vec<u32>,
//     wiring_map: HashMap<u32, HashSet<u32>>,
//     j_req: & 'a Vec<u32>,
//     stack: Vec<(u32, u32)>, // (optimal_button_idx, presses_count)
// }
//
// fn find_minimum_presses(joltage_requirements: &Vec<u32>, wiring_schematics: &Vec<Vec<u32>>) {
//     let mut wiring_map: HashMap<u32, HashSet<u32>> = HashMap::new();
//     for (button_idx, positions) in wiring_schematics.iter().enumerate() {
//         for pos in positions {
//             wiring_map
//                 .entry(*pos)
//                 .or_insert_with(HashSet::new)
//                 .insert(button_idx as u32);
//         }
//     }
//     let mut state = State {
//         j_tally: vec![0; joltage_requirements.len()],
//         j_req: joltage_requirements,
//         wiring_map,
//         stack: Vec::new()
//     };
//     do_find_minimum_presses(&mut state);
// }
//
// fn do_find_minimum_presses(
//     st : &mut State,
// ) {
//
//     if let Some(head) = st.stack.last_mut() {
//         head.1 = 3;
//     }
// }

struct ProblemData {
    lines: Vec<ProblemLine>,
}

impl FromStr for ProblemData {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::character::complete::line_ending;
        use nom::combinator::all_consuming;

        let parser = separated_list1(line_ending, parse_problem_line);
        let result = all_consuming(parser)(s.trim())
            .map(|(_, lines)| ProblemData { lines })
            .map_err(|e| format!("Parse error: {:?}", e));

        result
    }
}

#[derive(Debug)]
struct ProblemLine {
    indicator_lights: BitVec,
    wiring_schematics: Vec<Vec<u32>>,
    joltage_requirements: Vec<u32>,
}

impl ProblemLine {
    fn select_candidates(
        &self,
        pos_to_toggle: &HashSet<u32>, // the positions that need to be toggled
        already_pressed: &IndexSet<u32>,
        pos_to_button_idx: &HashMap<u32, Vec<u32>>, // map from position to schematic button indices
    ) -> Vec<u32> {
        pos_to_toggle
            .iter()
            .flat_map(|i| {
                pos_to_button_idx
                    .get(i)
                    .unwrap()
                    .iter()
                    .filter_map(|button_idx| {
                        if already_pressed.contains(button_idx) {
                            None
                        } else {
                            Some(*button_idx)
                        }
                    })
            })
            .unique()
            .sorted_by_key(|x| {
                let schema = &self.wiring_schematics[*x as usize];
                let mut score = 0;
                for pos in schema {
                    score += if pos_to_toggle.contains(pos) { 1 } else { -1 }
                }
                score
            })
            .collect()
    }

    fn find_solutions(&self) -> Vec<Vec<u32>> {
        let positions_to_toggle = self
            .indicator_lights
            .iter()
            .enumerate()
            .filter_map(|(i, light_on)| {
                if !light_on.as_bool() {
                    Some(i as u32)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();
        if positions_to_toggle.is_empty() {
            return Vec::new();
        }

        let pos_to_button_idx: HashMap<u32, Vec<u32>> = self
            .wiring_schematics
            .iter()
            .enumerate()
            .flat_map(|(button_idx, positions)| {
                positions.iter().map(move |pos| (*pos, button_idx as u32))
            })
            .into_group_map();

        let already_pressed: IndexSet<u32> = IndexSet::new();

        let p = self.find_paths(
            positions_to_toggle,
            already_pressed,
            &pos_to_button_idx,
            u32::MAX,
        );

        p
    }

    // TODO first implement cheapest threshold termination
    //      then change return type to IndexSet to avoid duplicates and maintain insertion order.
    fn find_paths(
        &self,
        positions_to_toggle: HashSet<u32>,
        already_pressed: IndexSet<u32>,
        pos_to_button_idx: &HashMap<u32, Vec<u32>>,
        cheapest: u32,
    ) -> Vec<Vec<u32>> {
        let mut res = Vec::new();
        // TODO use cheapest threshold
        assert!(!positions_to_toggle.is_empty());

        if already_pressed.len() as u32 >= 50 {
            println!(
                "Aborting deep recursion at buttons pressed: {:?}",
                already_pressed
            );
        }
        let candidates =
            self.select_candidates(&positions_to_toggle, &already_pressed, pos_to_button_idx);

        // let mut dbg = Vec::new();
        // for c in candidates.iter() {
        //     let schema = &self.wiring_schematics[*c as usize];
        //     let mut score = 0;
        //     for pos in schema {
        //         score += if !positions_to_toggle.contains(pos) {
        //             1
        //         } else {
        //             -1
        //         }
        //     }
        //     dbg.push(score);
        // }
        // println!("{:?}", dbg);

        if candidates.is_empty() {
            return res;
        }
        let mut cheapest = cheapest;
        for button_idx in candidates {
            let mut new_already_pressed = already_pressed.clone();
            new_already_pressed.insert(button_idx);

            // early termination is failing
            assert!((new_already_pressed.len() as u32) < cheapest);

            let mut new_positions_to_toggle = positions_to_toggle.clone();
            for pos in &self.wiring_schematics[button_idx as usize] {
                if !new_positions_to_toggle.remove(pos) {
                    new_positions_to_toggle.insert(*pos);
                }
            }

            if new_positions_to_toggle.is_empty() {
                res.push(new_already_pressed.iter().copied().collect()); // TODO I don't I need to copy here I think
                break; // we found a solution, no need to continue as others would be longer
            } else if (new_already_pressed.len() as u32) < cheapest - 1 {
                // we don't recurse if we've already found a cheaper solution
                let paths = self.find_paths(
                    new_positions_to_toggle,
                    new_already_pressed,
                    pos_to_button_idx,
                    cheapest,
                );
                cheapest = paths
                    .iter()
                    .map(|p| p.len() as u32)
                    .min()
                    .unwrap_or(cheapest);

                res.extend(paths);
            }
        }

        res
    }

    fn find_optimal_solution(&self) -> Vec<u32> {
        let target = self.indicator_lights.clone();

        // Convert buttons to BitVec for XOR
        let buttons: Vec<BitVec> = self
            .wiring_schematics
            .iter()
            .map(|positions| {
                let mut bv = BitVec::repeat(false, self.indicator_lights.len());
                for &pos in positions {
                    bv.set(pos as usize, true);
                }
                bv
            })
            .collect();

        let n = buttons.len();

        // Try increasing numbers of button presses
        for num_presses in 0..=n {
            for combo in (0..n).combinations(num_presses) {
                let mut state = BitVec::repeat(false, self.indicator_lights.len());

                for &button_idx in &combo {
                    state ^= &buttons[button_idx];
                }

                if state == target {
                    // Found minimum solution!
                    return combo.iter().map(|&i| i as u32).collect();
                }
            }
        }

        Vec::new() // No solution found
    }
}

fn parse_problem_line(input: &str) -> IResult<&str, ProblemLine> {
    // parses [.##.]
    let (input, indicator_lights) = delimited(
        char('['),
        fold_many1(
            alt((char('#'), char('.'))),
            BitVec::new,
            |mut acc: BitVec, c| {
                acc.push(c == '#');
                acc
            },
        ),
        char(']'),
    )(input)?;

    let (input, _) = space1(input)?;

    // parses (3) (1,3) ...
    let (input, wiring_schematics) = separated_list0(
        char(' '),
        delimited(char('('), separated_list0(char(','), u32), char(')')),
    )(input)?;

    let (input, _) = space1(input)?;

    // parses {10,11,11,5,10,5}
    let (input, joltage_requirements) =
        delimited(char('{'), separated_list1(char(','), u32), char('}'))(input)?;

    Ok((
        input,
        ProblemLine {
            indicator_lights,
            wiring_schematics,
            joltage_requirements,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
[###.#...#.] (0,1,4,5,6,8,9) (1,3,4,7,8,9) (1,6,7,8) (0,2,3,5,7,8,9) (6,8,9) (1,3,4) (1,4,5) (1,2,6,8) (4,7,9) (0,2,3,4,5,6,7,9) (0,1,2,4,5,6,7) (4,6) (0,1,2,3,5,6,7,9) {46,102,50,59,84,57,75,80,62,55}
";

    #[test]
    fn test_sample_problem_1() {
        let data: ProblemData = TEST_INPUT.parse().unwrap();
        println!(
            "solutions: {:?}",
            data.lines[0].find_solutions().iter().map(|s| s.len()).min()
        );
        println!(
            "solutions: {:?}",
            data.lines[1].find_solutions().iter().map(|s| s.len()).min()
        );
        println!(
            "solutions: {:?}",
            data.lines[2].find_solutions().iter().map(|s| s.len()).min()
        );
        // TODO assert results
    }

    #[test]
    fn test_large_entry() {
        let mut data: ProblemData = TEST_INPUT.parse().unwrap();
        let res = data.lines[3].find_optimal_solution();
        println!("solutions: {:?}", res);
        let res2 = data.lines[3].find_solutions();
        println!("solutions: {:?}", res2);
        // TODO assert results
    }

    #[test]
    fn test_sample_problem_2() {
        let data: ProblemData = TEST_INPUT.parse().unwrap();
        let res: u32 = (0..3)  // Test all 4 lines including the hard one!
            .into_iter()
            .map(|i| {
                eprintln!("\n======= Testing line {} =======", i);
                eprintln!("Target: {:?}", data.lines[i].joltage_requirements);

                let r = find_minimum_presses_ilp(
                    &data.lines[i].joltage_requirements,
                    &data.lines[i].wiring_schematics,
                )
                .expect("must have solution");
                println!("Line {}: {} total presses\n", i, r);
                r
            })
            .sum();
        println!("\n=============================");
        println!("Total for all lines: {}", res);
    }
}
