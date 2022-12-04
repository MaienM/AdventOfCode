use std::ops::RangeInclusive;

use aoc::runner::*;

type Range = RangeInclusive<i16>;

fn range_is_subset(left: &Range, right: &Range) -> bool {
    return left.contains(right.start()) && left.contains(right.end());
}

fn range_is_subset_two_ways(left: &Range, right: &Range) -> bool {
    return range_is_subset(left, right) || range_is_subset(right, left);
}

fn ranges_overlap(left: &Range, right: &Range) -> bool {
    return left.contains(right.start())
        || left.contains(right.end())
        || right.contains(left.start())
        || right.contains(left.end());
}

fn parse_input(input: String) -> Vec<(Range, Range)> {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            let [left, right]: [Range; 2] = line
                .trim()
                .splitn(2, ",")
                .into_iter()
                .map(|part| {
                    let [lower, upper]: [i16; 2] = part
                        .splitn(2, "-")
                        .map(str::parse)
                        .map(Result::unwrap)
                        .collect::<Vec<i16>>()
                        .try_into()
                        .unwrap();
                    return lower..=upper;
                })
                .collect::<Vec<Range>>()
                .try_into()
                .unwrap();
            return (left, right);
        })
        .collect();
}

pub fn part1(input: String) -> usize {
    let pairs = parse_input(input);
    return pairs
        .into_iter()
        .filter(|(left, right)| range_is_subset_two_ways(left, right))
        .count();
}

pub fn part2(input: String) -> usize {
    let pairs = parse_input(input);
    return pairs
        .into_iter()
        .filter(|(left, right)| ranges_overlap(left, right))
        .count();
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
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
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 4);
    }
}
