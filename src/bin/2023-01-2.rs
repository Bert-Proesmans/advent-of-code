use aoc_parse::{parser, prelude::*};

fn parse_input(text: &str) -> Vec<usize> {
    let numbers = &[
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    // Offset focused string matcher
    let has_number_within = |input: &[char], at_end: bool, debug: bool| -> Option<usize> {
        if debug {
            dbg!(input, at_end);
        }

        // WARN; Calculations with start_idx need to be in the same domain (0- or 1-indexed)!
        for start_idx in 1..=input.len() {
            let haystack = if at_end == false {
                String::from_iter(&input[(start_idx-1)..])
            } else {
                // Must work from the input string backwards, without reversing the character order
                let start_idx = input.len() - start_idx;
                String::from_iter(&input[start_idx..])
            };
            if debug {
                dbg!(start_idx, &haystack);
            }

            for (needle, value) in numbers {
                if haystack.starts_with(needle) {
                    if debug {
                        dbg!(needle, haystack);
                    }
                    return Some(*value);
                }
            }
        }

        None
    };

    let parse_numbers =
        |prefix: Vec<char>, first: usize, last: usize, suffix: Vec<char>| -> usize {
            let debug = false;
            if debug {
                dbg!(debug);
            }

            let prefix_num = has_number_within(&prefix, false, debug);
            let suffix_num = has_number_within(&suffix, true, debug);

            if debug {
                dbg!(prefix, prefix_num, suffix, suffix_num);
            }

            match (prefix_num, suffix_num) {
                (Some(a_first), Some(a_last)) => 10 * a_first + a_last,
                (Some(a_first), None) => 10 * a_first + last,
                (None, Some(a_last)) => 10 * first + a_last,
                (None, None) => 10 * first + last,
            }
        };

    let p = parser!(
        lines({
            prefix:alpha* first:digit any_char* last:digit suffix:alpha* => parse_numbers(prefix, first, last, suffix),
            // Special case; no second digit
            prefix:alpha* first:digit suffix:alpha* => parse_numbers(prefix, first, first, suffix),
    }));
    p.parse(text).unwrap()
}

#[advent_of_code::main(2023, 01)]
pub fn part_2(input: &str) -> usize {
    // dbg!(parse_input(input).iter().take(10).collect::<Vec<_>>());
    // return 0;
    parse_input(input).iter().sum()
}
