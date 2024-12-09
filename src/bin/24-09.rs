use std::{char, iter};

use aoc::utils::parse;

fn parse_input(input: &str) -> Vec<usize> {
    parse!(input => [nums chars as usize]);
    nums.into_iter()
        .enumerate()
        .flat_map(|(i, n)| {
            let id = if i % 2 == 0 { (i / 2) + 1 } else { 0 };
            iter::repeat_n(id, n)
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let mut filesystem = parse_input(input);
    loop {
        let empty = filesystem
            .iter()
            .enumerate()
            .find(|(_, c)| **c == 0)
            .unwrap()
            .0;
        let file = filesystem
            .iter()
            .enumerate()
            .rev()
            .find(|(_, c)| **c != 0)
            .unwrap()
            .0;
        if empty < file {
            filesystem.swap(empty, file);
        } else {
            break;
        }
    }
    filesystem
        .into_iter()
        .enumerate()
        .map(|(i, id)| if id == 0 { 0 } else { i * (id - 1) })
        .sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 1928)]
    static EXAMPLE_INPUT: &str = "2333133121414131402";

    // #[test]
    // fn example_parse() {
    //     let actual = parse_input(&EXAMPLE_INPUT);
    //     let expected: Vec<_> = "00...111...2...333.44.5555.6666.777.888899"
    //         .chars()
    //         .collect();
    //     assert_eq!(actual, expected);
    // }
}
