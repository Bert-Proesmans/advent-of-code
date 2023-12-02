use aoc_parse::{parser, prelude::*};

#[derive(Debug)]
struct Cubes {
    red: Option<usize>,
    green: Option<usize>,
    blue: Option<usize>,
}

fn parse_input(text: &str) -> Vec<(usize, Vec<Vec<Cubes>>)> {
    let p = parser!(
        rule color: Cubes = {
            red:usize " red" => Cubes {red: Some(red), green: None, blue: None},
            green:usize " green" => Cubes {red: None, green: Some(green), blue: None},
            blue:usize " blue" => Cubes {red: None, green: None, blue: Some(blue)},
        };

        rule grab: Vec<Cubes> = repeat_sep(color, ", ");

        lines("Game " usize ": " repeat_sep(grab, "; "))
    );
    p.parse(text).unwrap()
}

#[advent_of_code::main(2023, 02)]
pub fn part_2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|(idx, cube_games)| {
            let min_cubes_seen = Cubes {
                red: cube_games.iter().flatten().filter_map(|c| c.red).max(),
                green: cube_games.iter().flatten().filter_map(|c| c.green).max(),
                blue: cube_games.iter().flatten().filter_map(|c| c.blue).max(),
            };
            (idx, min_cubes_seen)
        })
        .map(|(_, cubes)| {
            cubes.red.expect("Red should not be empty")
                * cubes.green.expect("Green should not be empty")
                * cubes.blue.expect("Blue should not be empty")
        })
        .sum()
}
