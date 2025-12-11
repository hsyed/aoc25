# Advent of Code 2025

Solutions for Advent of Code 2025 in Rust.

## The journey

I have been on a sabbatical from work for a while and I am a decade+ away from leet coding. I'm trying my first AOC 
and I am doing it pair programming with Claude. I want to refresh my fundamentals but leetcoding is a burnout inducing 
process of willing an algorithm into existence from scratch.

### My goals:

* Refresh rust syntax and semantics.
* See how capable language models are with DSA problems and what pair programming feels like.
* Refresh basic problem busting strategies (DFS/BFS, DP, backtracking, etc) without the performative leetcoding/interview pressure.
* Explore multiple approaches to problems and see how they compare in terms of readability, performance, and ease of implementation.

### My process:

1. Get the agent to stub out the day.
2. Attempt the problem by hand. 
3. Get alternatives when stuck or to explore different approaches.
4. Let the agent implement the other approaches.
5. Understand, refactor or document as needed (the ascii art for day9 from the model = :fire:).

On the hardest problems --e.g., day10, day9. I spent 5+ hours before letting the agent carry me.

### Stuff re-learned

* Euclidean division in Rust a = qb + r (for normalising arithmetic done with wrap around).
* What dynamic programming was -- I remembered it memoizes sub-computations structurally, I had just forgotten how.
* Intuition for selecting between DFS and BFS.
* Linear programming! I have done this before but in Excel.
* Ray casting to determine inside or outside of rectilinear polygon bounds.

## Usage

```bash
cargo run -- --day <day_number>
```

## Structure

- `src/day*.rs` - Daily solutions
- `input/day*.txt` - Puzzle inputs

