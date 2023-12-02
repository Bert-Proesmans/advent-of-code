use aoc_parse::{parser, prelude::*};

type Gift = (u64, u64, u64);

fn parse_input(text: &str) -> Vec<Gift> {
    let p = parser!(lines(u64 "x" u64 "x" u64));
    p.parse(text).unwrap()
}

#[advent_of_code::main(2015, 02)]
pub fn solve_part1(input: &str) -> u64 {
    parse_input(input)
        .iter()
        .map(|&(l, w, h)| {
            let (s1, s2) = smallest_side((l, w, h));
            2 * l * w + 2 * w * h + 2 * h * l + s1 * s2
        })
        .sum()
}

fn smallest_side((l, w, h): Gift) -> (u64, u64) {
    let mut vec = vec![l, w, h];
    vec.sort();

    (vec[0], vec[1])
}
