puzzle_runner::register_chapter!(book = "2022", title = "Camp Cleanup");

use std::ops::RangeInclusive;

type Range = RangeInclusive<u16>;

fn parse_range(input: &str) -> Range {
    parse!(input => { [start as u16] "-" [end as u16] } => start..=end)
}

fn parse_input(input: &str) -> Vec<(Range, Range)> {
    parse!(input => {
        [ranges split on '\n' with
            { [left with parse_range] "," [right with parse_range] }
            => (left, right)
        ]
    } => ranges)
}

fn range_is_subset(left: &Range, right: &Range) -> bool {
    left.contains(right.start()) && left.contains(right.end())
}

fn range_is_subset_two_ways(left: &Range, right: &Range) -> bool {
    range_is_subset(left, right) || range_is_subset(right, left)
}

fn ranges_overlap(left: &Range, right: &Range) -> bool {
    left.contains(right.start())
        || left.contains(right.end())
        || right.contains(left.start())
        || right.contains(left.end())
}

#[register_part]
fn part1(input: &str) -> usize {
    let pairs = parse_input(input);
    pairs
        .into_iter()
        .filter(|(left, right)| range_is_subset_two_ways(left, right))
        .count()
}

#[register_part]
fn part2(input: &str) -> usize {
    let pairs = parse_input(input);
    pairs
        .into_iter()
        .filter(|(left, right)| ranges_overlap(left, right))
        .count()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 2, part2 = 4)]
    static EXAMPLE_INPUT: &str = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
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
    fn test_range_is_subset() {
        assert_eq!(range_is_subset(&(0..=8), &(1..=7)), true);
        assert_eq!(range_is_subset(&(1..=7), &(0..=8)), false);
        assert_eq!(range_is_subset(&(1..=9), &(0..=8)), false);
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
}
