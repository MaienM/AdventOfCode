use std::collections::HashMap;

use aoc::utils::{ext::iter::IterExt as _, parse};

fn parse_input(input: &str) -> Vec<usize> {
    parse!(input => { [num split as usize] } => num)
}

fn cycle(num: usize) -> Vec<usize> {
    if num == 0 {
        return vec![1];
    }
    let s = num.to_string();
    if s.len() % 2 == 0 {
        vec![
            s[..(s.len() / 2)].parse().unwrap(),
            s[(s.len() / 2)..].parse().unwrap(),
        ]
    } else {
        vec![num * 2024]
    }
}

fn solve(nums: Vec<usize>, cycles: usize) -> usize {
    let mut map = nums.into_iter().count_occurences();
    for _ in 0..cycles {
        let mut new_map = HashMap::new();
        for (num, count) in map {
            for new_num in cycle(num) {
                *new_map.entry(new_num).or_insert(0) += count;
            }
        }
        map = new_map;
    }
    map.into_values().sum()
}

pub fn part1(input: &str) -> usize {
    let stones = parse_input(input);
    solve(stones, 25)
}

pub fn part2(input: &str) -> usize {
    let stones = parse_input(input);
    solve(stones, 75)
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input()]
    static EXAMPLE_INPUT_1: &str = "0 1 10 99 999";

    #[example_input(part1 = 55_312)]
    static EXAMPLE_INPUT_2: &str = "125 17";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = vec![0, 1, 10, 99, 999];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_cycle() {
        let mut actual = vec![125, 17];

        actual = actual.into_iter().flat_map(cycle).collect();
        let expected = vec![253_000, 1, 7];
        assert_eq!(actual, expected);

        actual = actual.into_iter().flat_map(cycle).collect();
        let expected = vec![253, 0, 2024, 14168];
        assert_eq!(actual, expected);

        actual = actual.into_iter().flat_map(cycle).collect();
        let expected = vec![512_072, 1, 20, 24, 28_676_032];
        assert_eq!(actual, expected);

        actual = actual.into_iter().flat_map(cycle).collect();
        let expected = vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032];
        assert_eq!(actual, expected);

        actual = actual.into_iter().flat_map(cycle).collect();
        let expected = vec![103_6288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32];
        assert_eq!(actual, expected);

        actual = actual.into_iter().flat_map(cycle).collect();
        let expected = vec![
            2_097_446_912,
            14_168,
            4048,
            2,
            0,
            2,
            4,
            40,
            48,
            2024,
            40,
            48,
            80,
            96,
            2,
            8,
            6,
            7,
            6,
            0,
            3,
            2,
        ];
        assert_eq!(actual, expected);
    }
}
