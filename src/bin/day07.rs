use aoc::utils::parse::splitn;

type Card = char;

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
            let cards = cards.chars().collect::<Vec<_>>().try_into().unwrap();
            Hand { cards, bid }
        })
        .collect()
}

fn card_to_numbers(card: Card, value_for_j: u8) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => value_for_j,
        'T' => 10,
        c => c.to_digit(10).unwrap() as u8,
    }
}

fn cards_to_numbers(cards: [Card; 5], value_for_j: u8) -> [u8; 5] {
    [
        card_to_numbers(cards[0], value_for_j),
        card_to_numbers(cards[1], value_for_j),
        card_to_numbers(cards[2], value_for_j),
        card_to_numbers(cards[3], value_for_j),
        card_to_numbers(cards[4], value_for_j),
    ]
}

fn calculate_total_winnings(hands: Vec<Hand>, calculate_score: fn(Hand) -> (u8, [u8; 5])) -> usize {
    let mut scores: Vec<_> = hands
        .into_iter()
        .map(|hand| {
            let bid = hand.bid;
            let (typ, cards) = calculate_score(hand);
            let score = typ as usize * 15usize.pow(5)
                + cards[0] as usize * 15usize.pow(4)
                + cards[1] as usize * 15usize.pow(3)
                + cards[2] as usize * 15usize.pow(2)
                + cards[3] as usize * 15usize
                + cards[4] as usize;
            (score, bid)
        })
        .collect();
    scores.sort_unstable_by_key(|(score, _)| *score);
    let scores: Vec<_> = scores
        .into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .collect();
    scores.iter().sum()
}

pub fn part1(input: &str) -> usize {
    let hands = parse_input(input);
    calculate_total_winnings(hands, |hand| {
        let mut groups = [0usize; 15];
        let cards = cards_to_numbers(hand.cards, 11);
        for card in cards {
            groups[card as usize] += 1;
        }
        groups.sort_unstable();

        let typ = match (groups[14], groups[13]) {
            // Five of a kind.
            (5, _) => 6,
            // Four of a kind.
            (4, _) => 5,
            // Full house.
            (3, 2) => 4,
            // Three of a kind.
            (3, _) => 3,
            // Two pair.
            (2, 2) => 2,
            // One pair.
            (2, _) => 1,
            // High card.
            (_, _) => 0,
        };

        (typ, cards)
    })
}

pub fn part2(input: &str) -> usize {
    let hands = parse_input(input);
    calculate_total_winnings(hands, |hand| {
        let mut groups = [0usize; 15];
        let mut jokers = 0;
        let cards = cards_to_numbers(hand.cards, 0);
        for card in cards {
            match card {
                0 => jokers += 1,
                card => groups[card as usize] += 1,
            };
        }
        groups.sort_unstable();

        #[allow(clippy::match_same_arms)]
        let typ = match (jokers, groups[14], groups[13]) {
            // Five of a kind.
            (0, 5, _) => 6,
            (1, 4, _) => 6,
            (2, 3, _) => 6,
            (3, 2, _) => 6,
            (4, 1, _) => 6,
            (5, _, _) => 6,
            // Four of a kind.
            (0, 4, _) => 5,
            (1, 3, _) => 5,
            (2, 2, _) => 5,
            (3, 1, _) => 5,
            // Full house.
            (0, 3, 2) => 4,
            (1, 2, 2) => 4,
            // Three of a kind>
            (0, 3, _) => 3,
            (1, 2, _) => 3,
            (2, 1, _) => 3,
            // Two pair.
            (0, 2, 2) => 2,
            // One pair.
            (0, 2, _) => 1,
            (1, 1, _) => 1,
            // High card.
            (_, _, _) => 0,
        };

        (typ, cards)
    })
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 6440, part2 = 5905, test)]
    static EXAMPLE_INPUT: &str = "
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
                cards: ['3', '2', 'T', '3', 'K'],
                bid: 765,
            },
            Hand {
                cards: ['T', '5', '5', 'J', '5'],
                bid: 684,
            },
            Hand {
                cards: ['K', 'K', '6', '7', '7'],
                bid: 28,
            },
            Hand {
                cards: ['K', 'T', 'J', 'J', 'T'],
                bid: 220,
            },
            Hand {
                cards: ['Q', 'Q', 'Q', 'J', 'A'],
                bid: 483,
            },
        ];
        assert_eq!(actual, expected);
    }
}
