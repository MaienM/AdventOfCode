use std::collections::{HashMap, HashSet};

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

fn run_until<'a>(
    instructions: &Instructions<'a>,
    offset: usize,
    start: &'a str,
    is_end: fn(&str) -> bool,
) -> (usize, &'a str) {
    let mut current = start;
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
        current = instructions.maps.get(current).unwrap()[idx];

        if is_end(current) {
            return (steps, current);
        }
    }
    panic!("Should never happen");
}

fn prime_factors(num: usize) -> HashSet<usize> {
    let mut current = num;
    let mut factors = HashSet::new();
    for i in 2.. {
        while current % i == 0 {
            current /= i;
            factors.insert(i);
        }
        if current == 1 {
            break;
        }
    }
    factors
}

fn least_common_multiple(mut numbers: Vec<usize>) -> usize {
    match (numbers.pop(), numbers.pop()) {
        (Some(a), Some(b)) => {
            let factors_a = prime_factors(a);
            let factors_b = prime_factors(b);
            let gcf = factors_a.intersection(&factors_b).max().unwrap_or(&1);
            numbers.push(a * b / gcf);
            least_common_multiple(numbers)
        }
        (Some(a), None) => a,
        _ => panic!("Should never happen"),
    }
}

pub fn part1(input: &str) -> usize {
    let instructions = parse_input(input);
    run_until(&instructions, 0, "AAA", |c| c == "ZZZ").0
}

pub fn part2(input: &str) -> usize {
    let instructions = parse_input(input);
    let cycles: Vec<_> = instructions
        .maps
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| {
            let (first, k) = run_until(&instructions, 0, k, |c| c.ends_with('Z'));
            let (cycle, _) = run_until(&instructions, first, k, |c| c.ends_with('Z'));
            assert_eq!(first, cycle, "This logic only works if start -> finish and finish -> finish take the same amount of steps.");
            first
        })
        .collect();
    least_common_multiple(cycles)
}

generate_day_main!(part1, part2);

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

    #[example_input(part2 = 6, test)]
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

    #[test]
    fn example_parse_3() {
        let actual = parse_input(&EXAMPLE_INPUT_3);
        let expected = Instructions {
            directions: vec![Direction::Left, Direction::Right],
            maps: hash_map!(
                "11A" => ["11B", "XXX"],
                "11B" => ["XXX", "11Z"],
                "11Z" => ["11B", "XXX"],
                "22A" => ["22B", "XXX"],
                "22B" => ["22C", "22C"],
                "22C" => ["22Z", "22Z"],
                "22Z" => ["22B", "22B"],
                "XXX" => ["XXX", "XXX"],
            ),
        };
        assert_eq!(actual, expected);
    }
}
