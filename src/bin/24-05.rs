use std::cmp::Ordering;

use aoc::utils::parse;

fn parse_input(input: &str) -> (Vec<(u16, u16)>, Vec<Vec<u16>>) {
    parse!(input => {
        [rules split on '\n' with
            { [left as u16] '|' [right as u16] }
            => (left, right)
        ]
        "\n\n"
        [updates split on '\n' with [split on ',' as u16]]
    } => (rules, updates))
}

fn is_ordered(update: &[u16], rules: &[(u16, u16)]) -> bool {
    let mut forbidden = Vec::new();
    for page in update.iter().rev() {
        if forbidden.contains(page) {
            return false;
        }
        rules
            .iter()
            .filter(|(l, _)| l == page)
            .for_each(|(_, r)| forbidden.push(*r));
    }
    true
}

fn reorder(update: &mut [u16], rules: &[(u16, u16)]) {
    update.sort_unstable_by(|l, r| {
        if rules.contains(&(*l, *r)) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
}

pub fn part1(input: &str) -> u16 {
    let (rules, updates) = parse_input(input);
    updates
        .into_iter()
        .filter(|u| is_ordered(u, &rules))
        .map(|u| u[u.len() / 2])
        .sum()
}

pub fn part2(input: &str) -> u16 {
    let (rules, updates) = parse_input(input);
    updates
        .into_iter()
        .filter(|u| !is_ordered(u, &rules))
        .map(|mut u| {
            reorder(&mut u, &rules);
            u[u.len() / 2]
        })
        .sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 143, part2 = 123)]
    static EXAMPLE_INPUT: &str = "
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = (
            vec![
                (47, 53),
                (97, 13),
                (97, 61),
                (97, 47),
                (75, 29),
                (61, 13),
                (75, 53),
                (29, 13),
                (97, 29),
                (53, 29),
                (61, 53),
                (97, 53),
                (61, 29),
                (47, 13),
                (75, 47),
                (97, 75),
                (47, 61),
                (75, 61),
                (47, 29),
                (75, 13),
                (53, 13),
            ],
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ],
        );
        assert_eq!(actual, expected);
    }
}
