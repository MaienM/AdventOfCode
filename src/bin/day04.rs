use std::ops::RangeInclusive;

use aoc::runner::run;

type Range = RangeInclusive<i16>;

fn range_is_subset(left: &Range, right: &Range) -> bool {
    return left.contains(right.start()) && left.contains(right.end());
}

fn range_is_subset_two_ways(left: &Range, right: &Range) -> bool {
    range_is_subset(left, right) || range_is_subset(right, left)
}

fn ranges_overlap(left: &Range, right: &Range) -> bool {
    return left.contains(right.start())
        || left.contains(right.end())
        || right.contains(left.start())
        || right.contains(left.end());
}

fn parse_input(input: &str) -> Vec<(Range, Range)> {
    return input
        .trim()
        .split('\n')
        .map(|line| {
            let [left, right]: [Range; 2] = line
                .trim()
                .splitn(2, ',')
                .map(|part| {
                    let [lower, upper]: [i16; 2] = part
                        .splitn(2, '-')
                        .map(str::parse)
                        .map(Result::unwrap)
                        .collect::<Vec<i16>>()
                        .try_into()
                        .unwrap();
                    lower..=upper
                })
                .collect::<Vec<Range>>()
                .try_into()
                .unwrap();
            (left, right)
        })
        .collect();
}

pub fn part1(input: &str) -> usize {
    let pairs = parse_input(input);
    pairs
        .into_iter()
        .filter(|(left, right)| range_is_subset_two_ways(left, right))
        .count()
}

pub fn part2(input: &str) -> usize {
    let pairs = parse_input(input);
    pairs
        .into_iter()
        .filter(|(left, right)| ranges_overlap(left, right))
        .count()
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &str = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT);
        let expected = vec![
            (2..=4, 6..=8),
            (2..=3, 4..=5),
            (5..=7, 7..=9),
            (2..=8, 3..=7),
            (6..=6, 4..=6),
            (2..=6, 4..=8),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_range_is_subset_two_ways() {
        assert_eq!(range_is_subset_two_ways(&(0..=8), &(1..=7)), true);
        assert_eq!(range_is_subset_two_ways(&(1..=7), &(0..=8)), true);
        assert_eq!(range_is_subset_two_ways(&(1..=9), &(0..=8)), false);
    }

    #[test]
    fn test_ranges_overlap() {
        assert_eq!(ranges_overlap(&(0..=5), &(2..=7)), true);
        assert_eq!(ranges_overlap(&(5..=7), &(0..=5)), true);
        assert_eq!(ranges_overlap(&(6..=7), &(0..=5)), false);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 4);
    }
}
