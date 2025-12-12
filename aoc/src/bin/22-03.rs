puzzle_runner::register_chapter!(book = 2022, title = "Rucksack Reorganization");

use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct Rucksack(HashSet<char>, HashSet<char>);
impl Rucksack {
    fn contains(&self, chr: char) -> bool {
        self.0.contains(&chr) || self.1.contains(&chr)
    }
}

fn get_priority(chr: char) -> u16 {
    (chr as u16 - 38) % 58
}

fn parse_input(input: &str) -> Vec<Rucksack> {
    parse!(input => {
        [rucksacks split on '\n' with
            { line }
            => {
                let size = line.len();
                let mut chars = line.chars();
                let left = chars.by_ref().take(size / 2).collect::<HashSet<_>>();
                let right = chars.by_ref().collect::<HashSet<_>>();
                Rucksack(left, right)
            }
        ]
    } => rucksacks)
}

#[register_part]
fn part1(input: &str) -> u16 {
    let rucksacks = parse_input(input);
    let commonalities = rucksacks
        .iter()
        .map(|sack| sack.0.intersection(&sack.1).next().unwrap());
    commonalities.into_iter().copied().map(get_priority).sum()
}

#[register_part]
fn part2(input: &str) -> u16 {
    let mut rucksacks = parse_input(input).into_iter();
    let mut sum = 0u16;
    loop {
        match (rucksacks.next(), rucksacks.next(), rucksacks.next()) {
            (Some(r1), Some(r2), Some(r3)) => {
                let badge = (r1.0.union(&r1.1))
                    .find(|item| r2.contains(**item) && r3.contains(**item))
                    .unwrap();
                sum += get_priority(*badge);
            }
            (None, None, None) => return sum,
            v => invalid!(partial group v),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 157, part2 = 70)]
    static EXAMPLE_INPUT: &str = "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
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
}
