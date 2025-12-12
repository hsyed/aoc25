use nom::{
    IResult,
    bytes::complete::{tag, take},
    character::complete::space1,
    multi::separated_list1,
    sequence::separated_pair,
};
use petgraph::{
    algo::all_simple_paths,
    graph::{DiGraph, NodeIndex},
};

use std::{collections::HashMap, collections::hash_map::RandomState, str::FromStr};

pub fn solve_problem_1(main_file: &str) -> std::io::Result<()> {
    let input = std::fs::read_to_string(main_file)?;

    let data: ProblemData = input.parse().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Parse error: {}", e),
        )
    })?;

    let result = data.solve_part1();
    println!("problem 1: result = {}", result);
    Ok(())
}

pub fn solve_problem_2(main_file: &str) -> std::io::Result<()> {
    let input = std::fs::read_to_string(main_file)?;

    let data: ProblemData = input.parse().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Parse error: {}", e),
        )
    })?;

    let result = data.solve_part2(&["dac", "fft"]);
    println!("problem 2: result = {}", result);
    Ok(())
}

struct ProblemData {
    data: HashMap<String, Vec<String>>,
}

impl ProblemData {
    fn build_graph(&self) -> (DiGraph<String, ()>, HashMap<String, NodeIndex>) {
        let mut graph = DiGraph::new();
        let mut nodes = HashMap::new();

        if !self.data.contains_key("out") {
            nodes.insert("out".to_string(), graph.add_node("out".to_string()));
        }

        // Add all nodes
        for key in self.data.keys() {
            let idx = graph.add_node(key.clone());
            nodes.insert(key.clone(), idx);
        }

        // Add edges based on values pointing to their dependencies
        for (key, values) in &self.data {
            let from = nodes[key];
            for value in values {
                if let Some(&to) = nodes.get(value) {
                    graph.add_edge(from, to, ());
                }
            }
        }

        (graph, nodes)
    }

    fn solve_part1(&self) -> u64 {
        let (graph, nodes) = self.build_graph();

        let start = nodes["you"];
        let end = nodes["out"];

        // Find all simple paths from "you" to "out"
        let paths: Vec<Vec<NodeIndex>> =
            all_simple_paths::<Vec<NodeIndex>, _, RandomState>(&graph, start, end, 0, None)
                .collect();

        println!("Found {} paths from 'you' to 'out':", paths.len());
        for (i, path) in paths.iter().enumerate() {
            let path_str: Vec<String> = path.iter().map(|&idx| graph[idx].clone()).collect();
            println!("  Path {}: {}", i + 1, path_str.join(" → "));
        }

        paths.len() as u64
    }

    /// Count paths from svr to out that visit all required nodes.
    /// Uses memoization on (node, bitmask). Only correct for DAGs.
    fn solve_part2(&self, required: &[&str]) -> u64 {
        let target_mask = (1u8 << required.len()) - 1;
        let mut memo: HashMap<(&str, u8), u64> = HashMap::new();

        fn dfs<'a>(
            graph: &'a HashMap<String, Vec<String>>,
            current: &'a str,
            mask: u8,
            target_mask: u8,
            required: &[&str],
            memo: &mut HashMap<(&'a str, u8), u64>,
        ) -> u64 {
            if current == "out" {
                return if mask == target_mask { 1 } else { 0 };
            }

            if let Some(&cached) = memo.get(&(current, mask)) {
                return cached;
            }

            let count = graph
                .get(current)
                .map(|neighbors| {
                    neighbors
                        .iter()
                        .map(|next| {
                            let new_mask =
                                required.iter().enumerate().fold(mask, |m, (i, &req)| {
                                    if next == req { m | (1 << i) } else { m }
                                });
                            dfs(graph, next, new_mask, target_mask, required, memo)
                        })
                        .sum()
                })
                .expect("missing key in graph");

            memo.insert((current, mask), count);
            count
        }

        dfs(&self.data, "svr", 0, target_mask, required, &mut memo)
    }
}

impl FromStr for ProblemData {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_line(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
            separated_pair(
                take(3usize),                          // key: 3 chars
                tag(": "),                             // separator
                separated_list1(space1, take(3usize)), // space-separated values
            )(input)
        }

        let mut map = HashMap::new();

        for line in s.lines() {
            let (_, (key, values)) = parse_line(line)
                .map_err(|e| format!("Failed to parse line '{}': {:?}", line, e))?;

            map.insert(
                key.to_string(),
                values.iter().map(|v| v.to_string()).collect(),
            );
        }

        Ok(ProblemData { data: map })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TEST_INPUT_2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_sample_problem_1() {
        let data: ProblemData = TEST_INPUT.parse().unwrap();
        assert_eq!(data.solve_part1(), 5);
    }

    #[test]
    fn test_sample_problem_2() {
        let data: ProblemData = TEST_INPUT_2.parse().unwrap();
        assert_eq!(data.solve_part2(&["dac", "fft"]), 2);
    }
}
