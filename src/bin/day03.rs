use std::collections::HashSet;

use aoc::runner::*;

#[derive(Debug, Eq, PartialEq)]
struct Rucksack(HashSet<char>, HashSet<char>);

fn get_priority(chr: char) -> u16 {
    return (chr as u16 - 38) % 58;
}

fn parse_input(input: String) -> Vec<Rucksack> {
    return input
        .trim()
        .split("\n")
        .map(str::trim)
        .map(|line| {
            let size = line.len();
            let mut chars = line.chars().into_iter();
            let left = HashSet::from_iter(chars.by_ref().take(size / 2));
            let right = HashSet::from_iter(chars.by_ref());
            return Rucksack(left, right);
        })
        .collect();
}

pub fn part1(input: String) -> u16 {
    let rucksacks = parse_input(input);
    let commonalities: Vec<&char> = rucksacks
        .iter()
        .map(|sack| sack.0.intersection(&sack.1).next().unwrap())
        .collect();
    return commonalities.into_iter().copied().map(get_priority).sum();
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![
            Rucksack(
                HashSet::from(['g', 'J', 'p', 'r', 't', 'v', 'w', 'W']),
                HashSet::from(['c', 'f', 'F', 'h', 'M', 'p', 's']),
            ),
            Rucksack(
                HashSet::from(['D', 'G', 'H', 'j', 'L', 'N', 'q', 'R', 'z']),
                HashSet::from(['f', 'F', 'L', 'M', 'r', 's', 'S', 'Z']),
            ),
            Rucksack(
                HashSet::from(['d', 'm', 'P', 'q', 'r', 'V', 'z']),
                HashSet::from(['B', 'g', 'P', 'T', 'v', 'w', 'W']),
            ),
            Rucksack(
                HashSet::from(['h', 'H', 'L', 'M', 'q', 'v', 'w', 'Z']),
                HashSet::from(['b', 'B', 'c', 'F', 'j', 'n', 'Q', 'S', 'T', 'v']),
            ),
            Rucksack(
                HashSet::from(['g', 'G', 'J', 'R', 't']),
                HashSet::from(['c', 'Q', 't', 'T', 'Z']),
            ),
            Rucksack(
                HashSet::from(['C', 'G', 'J', 'P', 'r', 's', 'z', 'Z']),
                HashSet::from(['D', 'L', 'm', 'M', 'p', 's', 'w']),
            ),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_priority() {
        assert_eq!(get_priority('p'), 16);
        assert_eq!(get_priority('L'), 38);
        assert_eq!(get_priority('P'), 42);
        assert_eq!(get_priority('v'), 22);
        assert_eq!(get_priority('t'), 20);
        assert_eq!(get_priority('s'), 19);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 157);
    }
}
