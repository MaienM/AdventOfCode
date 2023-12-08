use std::collections::HashMap;

use aoc::{generate_day_main, splitn};

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Left = 0,
    Right = 1,
}

#[derive(Debug, PartialEq)]
struct Instructions<'a> {
    directions: Vec<Direction>,
    maps: HashMap<&'a str, [&'a str; 2]>,
}

fn parse_input(input: &str) -> Instructions {
    let (directions, maps) = splitn!(input, "\n\n", str, str);
    Instructions {
        directions: directions
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction {c:?}."),
            })
            .collect(),
        maps: maps
            .split('\n')
            .map(|line| {
                let (source, targets) = splitn!(line, " = ", str, str);
                let (left, right) = splitn!(targets[1..(targets.len() - 1)], ", ", str, str);
                (source, [left, right])
            })
            .collect(),
    }
}

pub fn part1(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut current = "AAA";
    let mut steps = 0;
    for direction in instructions.directions.iter().cloned().cycle() {
        current = instructions.maps.get(current).unwrap()[direction as usize];
        steps += 1;

        if current == "ZZZ" {
            return steps;
        }
    }
    0
}

generate_day_main!(part1);

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 2, test)]
    static EXAMPLE_INPUT_1: &str = "
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    ";

    #[example_input(part1 = 6, test)]
    static EXAMPLE_INPUT_2: &str = "
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    ";

    #[test]
    fn example_parse_1() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = Instructions {
            directions: vec![Direction::Right, Direction::Left],
            maps: hash_map!(
                "AAA" => ["BBB", "CCC"],
                "BBB" => ["DDD", "EEE"],
                "CCC" => ["ZZZ", "GGG"],
                "DDD" => ["DDD", "DDD"],
                "EEE" => ["EEE", "EEE"],
                "GGG" => ["GGG", "GGG"],
                "ZZZ" => ["ZZZ", "ZZZ"],
            ),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_2() {
        let actual = parse_input(&EXAMPLE_INPUT_2);
        let expected = Instructions {
            directions: vec![Direction::Left, Direction::Left, Direction::Right],
            maps: hash_map!(
                "AAA" => ["BBB", "BBB"],
                "BBB" => ["AAA", "ZZZ"],
                "ZZZ" => ["ZZZ", "ZZZ"],
            ),
        };
        assert_eq!(actual, expected);
    }
}
