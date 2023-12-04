use std::collections::HashSet;

use aoc::{generate_day_main, splitn};

#[derive(Debug, PartialEq)]
struct Card {
    numbers: HashSet<u8>,
    winners: HashSet<u8>,
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .split('\n')
        .map(|line| {
            let (_, line) = splitn!(line, ": ", str, str);
            let (numbers, winners) = splitn!(line, " | ", str, str);
            Card {
                numbers: numbers.split(' ').filter_map(|n| n.parse().ok()).collect(),
                winners: winners.split(' ').filter_map(|n| n.parse().ok()).collect(),
            }
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let cards = parse_input(input);
    cards
        .into_iter()
        .map(|card| {
            let winning = card.numbers.intersection(&card.winners).count();
            match winning {
                0 => 0,
                n => 2usize.pow(n as u32 - 1),
            }
        })
        .sum()
}

generate_day_main!(part1);

#[cfg(test)]
mod tests {
    use aoc::example;
    use common_macros::hash_set;
    use macro_rules_attribute::apply;
    use pretty_assertions::assert_eq;

    use super::*;

    #[apply(example)]
    static EXAMPLE_INPUT: String = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Card {
                numbers: hash_set![41, 48, 83, 86, 17],
                winners: hash_set![83, 86, 6, 31, 17, 9, 48, 53],
            },
            Card {
                numbers: hash_set![13, 32, 20, 16, 61],
                winners: hash_set![61, 30, 68, 82, 17, 32, 24, 19],
            },
            Card {
                numbers: hash_set![1, 21, 53, 59, 44],
                winners: hash_set![69, 82, 63, 72, 16, 21, 14, 1],
            },
            Card {
                numbers: hash_set![41, 92, 73, 84, 69],
                winners: hash_set![59, 84, 76, 51, 58, 5, 54, 83],
            },
            Card {
                numbers: hash_set![87, 83, 26, 28, 32],
                winners: hash_set![88, 30, 70, 12, 93, 22, 82, 36],
            },
            Card {
                numbers: hash_set![31, 18, 13, 56, 72],
                winners: hash_set![74, 77, 10, 23, 35, 67, 36, 11],
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(&EXAMPLE_INPUT), 13);
    }
}
