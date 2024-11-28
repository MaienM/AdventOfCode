#![allow(unused_variables)]

use aoc::utils::parse;

fn parse_input(input: &str) -> Vec<usize> {
    parse!(input => {
        [num split on '\n' as usize]
    } => num)
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    1
}

// aoc::cli::single::generate_main!();
fn main() {
}

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 1, test)]
    static EXAMPLE_INPUT: &str = "
        1
        2
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![1, 2];
        assert_eq!(actual, expected);
    }
}
