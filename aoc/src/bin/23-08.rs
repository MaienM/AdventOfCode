aoc::setup!();

use std::{collections::HashMap, sync::Arc};

use num::integer;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Left = 0,
    Right = 1,
}
impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction {value:?}."),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Instructions {
    directions: Vec<Direction>,
    maps: HashMap<String, [String; 2]>,
}

fn parse_map(line: &str) -> (String, [String; 2]) {
    parse!(line => [source as String] " = (" [left as String]", " [right as String] ")");
    (source, [left, right])
}

fn parse_input(input: &str) -> Instructions {
    parse!(input => {
        [directions chars as Direction]
        "\n\n"
        [maps split on '\n' into (HashMap<_, _>) with parse_map]
    } => Instructions { directions, maps })
}

fn run_until(
    instructions: &Instructions,
    offset: usize,
    start: &str,
    is_end: fn(&str) -> bool,
) -> (usize, String) {
    let mut current = &start.to_owned();
    let mut steps = 0;
    for direction in instructions
        .directions
        .iter()
        .cloned()
        .cycle()
        .skip(offset % instructions.directions.len())
    {
        let idx = direction as usize;
        steps += 1;
        current = &instructions.maps.get(current).unwrap()[idx];

        if is_end(current) {
            return (steps, current.clone());
        }
    }
    panic!("Should never happen");
}

pub fn part1(input: &str) -> usize {
    let instructions = parse_input(input);
    run_until(&instructions, 0, "AAA", |c| c == "ZZZ").0
}

pub fn part2(input: &str) -> usize {
    let instructions = Arc::new(parse_input(input));
    instructions
        .maps
        .keys()
        .par_bridge()
        .filter(|k| k.ends_with('A'))
        .map(|k| {
            #[allow(unused_variables)]
            let (first, k) = run_until(&instructions, 0, k, |c| c.ends_with('Z'));
            #[cfg(debug_assertions)]
            {
                let (cycle, _) = run_until(&instructions, first, &k, |c| c.ends_with('Z'));
                assert_eq!(first, cycle, "This logic only works if start -> finish and finish -> finish take the same amount of steps.");
            }
            first
        })
        .reduce(|| 1, integer::lcm)
}

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 2)]
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

    #[example_input(part1 = 6)]
    static EXAMPLE_INPUT_2: &str = "
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    ";

    #[example_input(part2 = 6)]
    static EXAMPLE_INPUT_3: &str = "
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    ";

    #[test]
    fn example_parse_1() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = Instructions {
            directions: vec![Direction::Right, Direction::Left],
            maps: hash_map!(
                "AAA".to_owned() => ["BBB".to_owned(), "CCC".to_owned()],
                "BBB".to_owned() => ["DDD".to_owned(), "EEE".to_owned()],
                "CCC".to_owned() => ["ZZZ".to_owned(), "GGG".to_owned()],
                "DDD".to_owned() => ["DDD".to_owned(), "DDD".to_owned()],
                "EEE".to_owned() => ["EEE".to_owned(), "EEE".to_owned()],
                "GGG".to_owned() => ["GGG".to_owned(), "GGG".to_owned()],
                "ZZZ".to_owned() => ["ZZZ".to_owned(), "ZZZ".to_owned()],
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
                "AAA".to_owned() => ["BBB".to_owned(), "BBB".to_owned()],
                "BBB".to_owned() => ["AAA".to_owned(), "ZZZ".to_owned()],
                "ZZZ".to_owned() => ["ZZZ".to_owned(), "ZZZ".to_owned()],
            ),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_3() {
        let actual = parse_input(&EXAMPLE_INPUT_3);
        let expected = Instructions {
            directions: vec![Direction::Left, Direction::Right],
            maps: hash_map!(
                "11A".to_owned() => ["11B".to_owned(), "XXX".to_owned()],
                "11B".to_owned() => ["XXX".to_owned(), "11Z".to_owned()],
                "11Z".to_owned() => ["11B".to_owned(), "XXX".to_owned()],
                "22A".to_owned() => ["22B".to_owned(), "XXX".to_owned()],
                "22B".to_owned() => ["22C".to_owned(), "22C".to_owned()],
                "22C".to_owned() => ["22Z".to_owned(), "22Z".to_owned()],
                "22Z".to_owned() => ["22B".to_owned(), "22B".to_owned()],
                "XXX".to_owned() => ["XXX".to_owned(), "XXX".to_owned()],
            ),
        };
        assert_eq!(actual, expected);
    }
}
