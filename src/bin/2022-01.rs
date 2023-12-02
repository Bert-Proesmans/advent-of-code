use aoc_parse::{parser, prelude::*};

fn parse_input(text: &str) -> Vec<Vec<i64>> {
    let p = parser!(sections(lines(i64)));
    p.parse(text).unwrap()
}

#[advent_of_code::main(2022, 01)]
pub fn most_calories_carrying_elf(input: &str) -> i64 {
    parse_input(input)
        .iter()
        .map(|grouped_records| grouped_records.iter().sum())
        .max()
        .expect("No output, meaning invalid input!")
}
