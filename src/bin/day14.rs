use std::collections::HashMap;

use aoc::counter::Counter;
use aoc::runner::*;
use derive_new::new;

type Pair = (char, char);
type Rules = HashMap<Pair, char>;
type PolymerPairCounts = HashMap<Pair, u64>;
#[derive(Debug, PartialEq, new)]
struct Polymer {
    pairs: PolymerPairCounts,
    start: char,
    end: char,
}

fn parse_input(input: String) -> (Polymer, Rules) {
    let mut parts = input.trim().splitn(2, "\n");
    let polymer_input = parts.next().unwrap();
    let rules_input = parts.next().unwrap();

    let mut pairs = PolymerPairCounts::new();
    for (l, r) in polymer_input
        .chars()
        .into_iter()
        .zip(polymer_input.chars().into_iter().skip(1))
    {
        let pair = (l, r);
        pairs.count(pair, 1);
    }

    let mut rules = Rules::new();
    for line in rules_input.trim().split("\n").map(str::trim) {
        let pair = (line.chars().nth(0).unwrap(), line.chars().nth(1).unwrap());
        let insertion = line.chars().nth(6).unwrap();
        rules.insert(pair, insertion);
    }

    return (
        Polymer::new(
            pairs,
            polymer_input.chars().next().unwrap(),
            polymer_input.chars().last().unwrap(),
        ),
        rules,
    );
}

fn do_step(polymer: Polymer, rules: &Rules) -> Polymer {
    let mut new_pairs = PolymerPairCounts::new();
    for (pair, count) in polymer.pairs {
        let insertion = rules.get(&pair).unwrap();
        let left = (pair.0, *insertion);
        let right = (*insertion, pair.1);

        new_pairs.count(left, count);
        new_pairs.count(right, count);
    }
    return Polymer::new(new_pairs, polymer.start, polymer.end);
}

fn get_polymer_char_counts(polymer: &Polymer) -> HashMap<char, u64> {
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    for (pair, count) in &polymer.pairs {
        char_counts.count(pair.0, *count);
        char_counts.count(pair.1, *count);
    }
    for (_, count) in char_counts.iter_mut() {
        *count /= 2;
    }
    char_counts.count(polymer.start, 1);
    char_counts.count(polymer.end, 1);
    return char_counts;
}

pub fn part1(input: String) -> u64 {
    let (mut polymer, rules) = parse_input(input);
    for _ in 0..10 {
        polymer = do_step(polymer, &rules);
    }
    let counts = get_polymer_char_counts(&polymer);
    return counts.values().max().unwrap() - counts.values().min().unwrap();
}

pub fn part2(input: String) -> u64 {
    let (mut polymer, rules) = parse_input(input);
    for _ in 0..40 {
        polymer = do_step(polymer, &rules);
    }
    let counts = get_polymer_char_counts(&polymer);
    return counts.values().max().unwrap() - counts.values().min().unwrap();
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
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
        let (actual_polymer, actual_rules) = parse_input(EXAMPLE_INPUT.to_string());
        let mut expected_polymer_counts = PolymerPairCounts::new();
        expected_polymer_counts.insert(('N', 'N'), 1);
        expected_polymer_counts.insert(('N', 'C'), 1);
        expected_polymer_counts.insert(('C', 'B'), 1);
        assert_eq!(actual_polymer.pairs, expected_polymer_counts);
        assert_eq!(actual_polymer.start, 'N');
        assert_eq!(actual_polymer.end, 'B');
        let mut expected_rules = Rules::new();
        expected_rules.insert(('C', 'H'), 'B');
        expected_rules.insert(('H', 'H'), 'N');
        expected_rules.insert(('C', 'B'), 'H');
        expected_rules.insert(('N', 'H'), 'C');
        expected_rules.insert(('H', 'B'), 'C');
        expected_rules.insert(('H', 'C'), 'B');
        expected_rules.insert(('H', 'N'), 'C');
        expected_rules.insert(('N', 'N'), 'C');
        expected_rules.insert(('B', 'H'), 'H');
        expected_rules.insert(('N', 'C'), 'B');
        expected_rules.insert(('N', 'B'), 'B');
        expected_rules.insert(('B', 'N'), 'B');
        expected_rules.insert(('B', 'B'), 'N');
        expected_rules.insert(('B', 'C'), 'B');
        expected_rules.insert(('C', 'C'), 'N');
        expected_rules.insert(('C', 'N'), 'C');
        assert_eq!(actual_rules, expected_rules);
    }

    #[test]
    fn example_polymer_parse() {
        let (actual_polymer, _) = parse_input("NNNCB\nAB -> C".to_string());
        let mut expected_polymer = HashMap::new();
        expected_polymer.insert(('N', 'N'), 2);
        expected_polymer.insert(('N', 'C'), 1);
        expected_polymer.insert(('C', 'B'), 1);
        assert_eq!(actual_polymer.pairs, expected_polymer);
        assert_eq!(actual_polymer.start, 'N');
        assert_eq!(actual_polymer.end, 'B');
    }

    #[test]
    fn example_polymer_count() {
        let (mut polymer, rules) = parse_input(EXAMPLE_INPUT.to_string());
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
        let (mut actual_polymer, actual_rules) = parse_input(EXAMPLE_INPUT.to_string());
        actual_polymer = do_step(actual_polymer, &actual_rules);
        let (expected_polymer, _) = parse_input("NCNBCHB\nAB -> C".to_string());
        assert_eq!(actual_polymer, expected_polymer);
    }

    #[test]
    fn example_step2() {
        let (mut actual_polymer, actual_rules) = parse_input(EXAMPLE_INPUT.to_string());
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        let (expected_polymer, _) = parse_input("NBCCNBBBCBHCB\nAB -> C".to_string());
        assert_eq!(actual_polymer, expected_polymer);
    }

    #[test]
    fn example_step3() {
        let (mut actual_polymer, actual_rules) = parse_input(EXAMPLE_INPUT.to_string());
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        let (expected_polymer, _) = parse_input("NBBBCNCCNBBNBNBBCHBHHBCHB\nAB -> C".to_string());
        assert_eq!(actual_polymer, expected_polymer);
    }

    #[test]
    fn example_step4() {
        let (mut actual_polymer, actual_rules) = parse_input(EXAMPLE_INPUT.to_string());
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        let (expected_polymer, _) =
            parse_input("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB\nAB -> C".to_string());
        assert_eq!(actual_polymer, expected_polymer);
    }

    #[test]
    fn example_step10() {
        let (mut actual_polymer, actual_rules) = parse_input(EXAMPLE_INPUT.to_string());
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        actual_polymer = do_step(actual_polymer, &actual_rules);
        let (expected_polymer, _) =
            parse_input("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB\nAB -> C".to_string());
        assert_eq!(actual_polymer, expected_polymer);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 1588);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 2188189693529);
    }
}
