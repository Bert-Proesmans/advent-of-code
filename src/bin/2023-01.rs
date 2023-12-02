use aoc_parse::{parser, prelude::*};

fn parse_input(text: &str) -> Vec<usize> {
    let p = parser!((lines({
            alpha* first:digit any_char* last:digit alpha* => 10 * first + last,
            // Special case; no second digit
            alpha* first:digit alpha* => 10* first + first,
    })));
    p.parse(text).unwrap()
}

#[advent_of_code::main(2023, 01)]
pub fn part_1(input: &str) -> usize {
    parse_input(input).iter().sum()
}
