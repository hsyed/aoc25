# Advent of Code 2025

Solutions for Advent of Code 2025 in Rust.

## The journey

I'm trying my first AOC and I am doing it pair programming with Claude.

### My goals

* Refresh rust syntax and semantics.
* See how capable language models are with DSA problems and what pair programming feels like.
* Refresh basic problem busting strategies (DFS/BFS, DP, backtracking, etc).
* Explore multiple approaches to problems and see how they compare in terms of readability, performance, and ease of implementation.

### My process

1. Get the agent to stub out the day.
2. Attempt the problem by hand.
3. Get alternatives when stuck or to explore different approaches.
4. Let the agent implement the other approaches.
5. Understand, refactor or document as needed (the ascii art for day9 from the model = :fire:).

Day 12 is NP hard, so I left it to Claude. It produced a working solution which we then optimised, I manually added
rayon to parallelize. 35x improvement over initial working version, ~4 seconds on my i9900k.

### Stuff  I've learned or re-learned along the way

* Euclidean division in Rust a = qb + r (for normalising arithmetic done with wrap around).
* What dynamic programming was -- I remembered it memoizes sub-computations structurally, I had just forgotten how.
* Intuition for selecting between DFS and BFS.
* Linear programming! I haven't worked with a linear equation solver in code before, just Excel.
* Ray casting to determine inside or outside rectilinear polygon bounds.
* Parser combinator lib: nom.
* Implemented [union find](https://yuminlee2.medium.com/union-find-algorithm-ffa9cd7d2dba) in terms of `Rc<HashSet>` for 
connected components.

## Usage

```bash
cargo run --release -- --day <day_number>
```
