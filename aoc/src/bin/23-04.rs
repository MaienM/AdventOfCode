puzzle_runner::register_chapter!(book = "2023", title = "Scratchcards");

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
struct Card {
    numbers: HashSet<u8>,
    winners: HashSet<u8>,
}

fn parse_input(input: &str) -> Vec<Card> {
    parse!(input => {
        [cards split on '\n' with
            { "Card " _ ": " [numbers split into (HashSet<_>) try as u8] " | " [winners split into (HashSet<_>) try as u8] }
            => Card { numbers, winners }
        ]
    } => cards)
}

#[register_part]
fn part1(input: &str) -> usize {
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

#[register_part]
fn part2(input: &str) -> u64 {
    let cards = parse_input(input);
    let mut sum = 0u64;
    let mut copies: HashMap<usize, u32> = HashMap::new();
    for (idx, card) in cards.into_iter().enumerate() {
        let amount = copies.remove(&idx).unwrap_or(1);
        let winning = card.numbers.intersection(&card.winners).count();
        for i in 0..winning {
            let idx = idx + i + 1;
            copies.insert(idx, copies.get(&idx).unwrap_or(&1) + amount);
        }
        sum += u64::from(amount);
    }
    sum
}

#[cfg(test)]
mod tests {
    use common_macros::hash_set;
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 13, part2 = 30)]
    static EXAMPLE_INPUT: &str = "
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
}
