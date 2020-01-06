# Advent of Code 2019

Solutions for [Advent of Code 2019](https://adventofcode.com/2019) in not-very-idiomatic Rust.

Some solutions are heavily inspired by other, more capable developers' solutions, while the Day 10 solution is a 98%
copy-paste job.

In abject violation of DRY principles, solutions that increment on a previous day's work (eg. Intcode puzzles) have been
copy-pasted wholesale and adapted/extended to the given puzzle. A refactor may or may not happen.

### Running the project

Requires Rust 1.31+ (2018 edition). To run the solution for eg. day1a, type `cargo run --bin day1a`. All solutions except
day5 will run to completion without user input.
