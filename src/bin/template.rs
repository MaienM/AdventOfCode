#![allow(unused_variables)]

use aoc::generate_day_main;

fn parse_input(input: &str) -> usize {
    0
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    0
}

generate_day_main!(part1);

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &str = "
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT);
        let expected = 0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 1);
    }
}
