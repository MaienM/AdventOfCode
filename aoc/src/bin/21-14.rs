aoc::setup!(title = "Extended Polymerization");

use std::collections::HashMap;

use itertools::Itertools;

type Pair = (char, char);
type Rules = HashMap<Pair, char>;
type PolymerPairCounts = HashMap<Pair, u64>;

#[derive(Debug, PartialEq)]
struct Polymer {
    pairs: PolymerPairCounts,
    start: char,
    end: char,
}
impl From<&str> for Polymer {
    fn from(value: &str) -> Self {
        let start = value.chars().next().unwrap();
        let end = value.chars().last().unwrap();

        let mut pairs = PolymerPairCounts::new();
        for pair in value.chars().tuple_windows() {
            *pairs.entry(pair).or_default() += 1;
        }

        Self { pairs, start, end }
    }
}

fn parse_input(input: &str) -> (Polymer, Rules) {
    parse!(input => {
        [polymer as Polymer]
        "\n\n"
        [rules split on '\n' into (Rules) with
            { [pair chars] " -> " [insertion as char] }
            => (<[char; 2]>::try_from(pair).unwrap().into(), insertion)
        ]
    } => (polymer, rules))
}

fn do_step(polymer: Polymer, rules: &Rules) -> Polymer {
    let mut new_pairs = PolymerPairCounts::new();
    for (pair, count) in polymer.pairs {
        let insertion = rules.get(&pair).unwrap();
        let left = (pair.0, *insertion);
        let right = (*insertion, pair.1);

        *new_pairs.entry(left).or_default() += count;
        *new_pairs.entry(right).or_default() += count;
    }
    Polymer {
        pairs: new_pairs,
        ..polymer
    }
}

fn get_polymer_char_counts(polymer: &Polymer) -> HashMap<char, u64> {
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    for (pair, count) in &polymer.pairs {
        *char_counts.entry(pair.0).or_default() += *count;
        *char_counts.entry(pair.1).or_default() += *count;
    }
    for count in char_counts.values_mut() {
        *count /= 2;
    }
    *char_counts.entry(polymer.start).or_default() += 1;
    *char_counts.entry(polymer.end).or_default() += 1;
    char_counts
}

pub fn part1(input: &str) -> u64 {
    let (mut polymer, rules) = parse_input(input);
    for _ in 0..10 {
        polymer = do_step(polymer, &rules);
    }
    let counts = get_polymer_char_counts(&polymer);
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

pub fn part2(input: &str) -> u64 {
    let (mut polymer, rules) = parse_input(input);
    for _ in 0..40 {
        polymer = do_step(polymer, &rules);
    }
    let counts = get_polymer_char_counts(&polymer);
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 1588, part2 = 2_188_189_693_529)]
    static EXAMPLE_INPUT: &str = "
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    ";

    #[test]
    fn example_parse() {
        let (actual_polymer, actual_rules) = parse_input(&EXAMPLE_INPUT);

        let expected_polymer_counts = hash_map![
            ('N', 'N') => 1,
            ('N', 'C') => 1,
            ('C', 'B') => 1,
        ];
        assert_eq!(actual_polymer.pairs, expected_polymer_counts);
        assert_eq!(actual_polymer.start, 'N');
        assert_eq!(actual_polymer.end, 'B');

        let expected_rules = hash_map![
            ('C', 'H') => 'B',
            ('H', 'H') => 'N',
            ('C', 'B') => 'H',
            ('N', 'H') => 'C',
            ('H', 'B') => 'C',
            ('H', 'C') => 'B',
            ('H', 'N') => 'C',
            ('N', 'N') => 'C',
            ('B', 'H') => 'H',
            ('N', 'C') => 'B',
            ('N', 'B') => 'B',
            ('B', 'N') => 'B',
            ('B', 'B') => 'N',
            ('B', 'C') => 'B',
            ('C', 'C') => 'N',
            ('C', 'N') => 'C',
        ];
        assert_eq!(actual_rules, expected_rules);
    }

    #[test]
    fn example_polymer_count() {
        let (mut polymer, rules) = parse_input(&EXAMPLE_INPUT);
        for _ in 0..10 {
            polymer = do_step(polymer, &rules);
        }
        let counts = get_polymer_char_counts(&polymer);
        assert_eq!(*counts.get(&'B').unwrap(), 1749);
        assert_eq!(*counts.get(&'C').unwrap(), 298);
        assert_eq!(*counts.get(&'H').unwrap(), 161);
        assert_eq!(*counts.get(&'N').unwrap(), 865);
    }

    #[test]
    fn example_step1() {
        let (mut actual_polymer, actual_rules) = parse_input(&EXAMPLE_INPUT);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        let expected_polymer = "NCNBCHB".into();
        assert_eq!(actual_polymer, expected_polymer);
    }

    #[test]
    fn example_step2() {
        let (mut actual_polymer, actual_rules) = parse_input(&EXAMPLE_INPUT);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        let expected_polymer = "NBCCNBBBCBHCB".into();
        assert_eq!(actual_polymer, expected_polymer);
    }

    #[test]
    fn example_step3() {
        let (mut actual_polymer, actual_rules) = parse_input(&EXAMPLE_INPUT);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        let expected_polymer = "NBBBCNCCNBBNBNBBCHBHHBCHB".into();
        assert_eq!(actual_polymer, expected_polymer);
    }

    #[test]
    fn example_step4() {
        let (mut actual_polymer, actual_rules) = parse_input(&EXAMPLE_INPUT);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        let expected_polymer = "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".into();
        assert_eq!(actual_polymer, expected_polymer);
    }
}
