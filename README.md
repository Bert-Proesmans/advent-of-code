# Advent of code solutions

## Preparation

None.

## Layout

This is a mono-repo containing all advent of code solutions I've ever created.  
Check [./src/bin] for all solution implementations.

This repository holds a simple procedural macro that wraps each solution to automatically inject the input data from the folder [./inputs]. This macro provides interaction with cargo, `cargo run --bin` works as expected.

Input parsing is handled by crate [aoc-parse](https://lib.rs/crates/aoc-parse), check [its documentation](https://docs.rs/aoc-parse/0.2.17/aoc_parse/index.html) for more details on the parser composition.

## Execute solutions

Either run `cargo run` to compile and run all solutions,  
or run `cargo run --bin [year]-[day]`, eg `cargo run --bin 2023-01` for [aoc 2023 day 1](https://adventofcode.com/2023/day/1), to run a specific solution.