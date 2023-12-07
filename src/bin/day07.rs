use aoc::{generate_day_main, splitn};

type Card = u8;

const CARD_A: u8 = 14;
const CARD_K: u8 = 13;
const CARD_Q: u8 = 12;
const CARD_J: u8 = 11;
const CARD_T: u8 = 10;

#[derive(Debug, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .split('\n')
        .map(|line| {
            let (cards, bid) = splitn!(line, ' ', str, usize);
            let cards = cards
                .chars()
                .map(|c| match c {
                    'A' => CARD_A,
                    'K' => CARD_K,
                    'Q' => CARD_Q,
                    'J' => CARD_J,
                    'T' => CARD_T,
                    c => c.to_digit(10).unwrap() as u8,
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            Hand { cards, bid }
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let hands = parse_input(input);
    let mut scores: Vec<_> = hands
        .into_iter()
        .map(|hand| {
            let mut groups = [0usize; 15];
            for card in hand.cards {
                groups[card as usize] += 1;
            }
            groups.sort_unstable();
            groups.reverse();
            let typ = match (groups[0], groups[1]) {
                (5, _) => 6,
                (4, _) => 5,
                (3, 2) => 4,
                (3, _) => 3,
                (2, 2) => 2,
                (2, _) => 1,
                (_, _) => 0,
            };

            let score = typ * 15usize.pow(5)
                + hand.cards[0] as usize * 15usize.pow(4)
                + hand.cards[1] as usize * 15usize.pow(3)
                + hand.cards[2] as usize * 15usize.pow(2)
                + hand.cards[3] as usize * 15usize
                + hand.cards[4] as usize;

            (score, hand.bid)
        })
        .collect();
    scores.sort_unstable_by_key(|p| p.0);
    let scores: Vec<_> = scores
        .into_iter()
        .enumerate()
        .map(|(r, (_, b))| (r + 1) * b)
        .collect();
    scores.iter().sum()
}

generate_day_main!(part1);

#[cfg(test)]
mod tests {
    use aoc::example;
    use macro_rules_attribute::apply;
    use pretty_assertions::assert_eq;

    use super::*;

    #[apply(example)]
    static EXAMPLE_INPUT: String = "
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Hand {
                cards: [3, 2, CARD_T, 3, CARD_K],
                bid: 765,
            },
            Hand {
                cards: [CARD_T, 5, 5, CARD_J, 5],
                bid: 684,
            },
            Hand {
                cards: [CARD_K, CARD_K, 6, 7, 7],
                bid: 28,
            },
            Hand {
                cards: [CARD_K, CARD_T, CARD_J, CARD_J, CARD_T],
                bid: 220,
            },
            Hand {
                cards: [CARD_Q, CARD_Q, CARD_Q, CARD_J, CARD_A],
                bid: 483,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(&EXAMPLE_INPUT), 6440);
    }
}
