use aoc_parse::{parser, prelude::*};

struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse_input(text: &str) -> Vec<(usize, Vec<Vec<Cubes>>)> {
    let p = parser!(
        rule color: Cubes = {
            red:usize " red" => Cubes {red, green: 0, blue: 0},
            green:usize " green" => Cubes {red: 0, green, blue: 0},
            blue:usize " blue" => Cubes {red: 0, green: 0, blue},
        };

        rule grab: Vec<Cubes> = repeat_sep(color, ", ");

        lines("Game " usize ": " repeat_sep(grab, "; "))
    );
    p.parse(text).unwrap()
}

#[advent_of_code::main(2023, 02)]
pub fn part_1(input: &str) -> usize {
    let fitting_cubes = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    parse_input(input)
        .iter()
        .map(|(idx, cube_games)| {
            let max_cubes_seen = Cubes {
                red: cube_games
                    .iter()
                    .flatten()
                    .map(|c| c.red)
                    .max()
                    .unwrap_or(0),
                green: cube_games
                    .iter()
                    .flatten()
                    .map(|c| c.green)
                    .max()
                    .unwrap_or(0),
                blue: cube_games
                    .iter()
                    .flatten()
                    .map(|c| c.blue)
                    .max()
                    .unwrap_or(0),
            };
            (idx, max_cubes_seen)
        })
        .filter_map(|(idx, cubes)| {
            if cubes.red <= fitting_cubes.red
                && cubes.green <= fitting_cubes.green
                && cubes.blue <= fitting_cubes.blue
            {
                Some(idx)
            } else {
                None
            }
        })
        .sum()
}
