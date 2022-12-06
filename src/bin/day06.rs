use std::collections::HashSet;

use aoc::runner::*;

fn find_marker(input: String) -> usize {
    let input = input.trim();
    return input
        .chars()
        .zip(input.chars().skip(1))
        .zip(input.chars().skip(2))
        .zip(input.chars().skip(3))
        .map(|(((c1, c2), c3), c4)| [c1, c2, c3, c4])
        .enumerate()
        .find(|(_idx, chars)| HashSet::<char>::from(*chars).len() == 4)
        .unwrap()
        .0
        + 4;
}

pub fn part1(input: String) -> usize {
    return find_marker(input);
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        mjqjpqmgbljsphdztnvjfqwrcgsmlb
    ";

    #[test]
    fn example_part1() {
        assert_eq!(find_marker(EXAMPLE_INPUT.to_string()), 7);
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6);
        assert_eq!(
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            10
        );
        assert_eq!(
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            11
        );
    }
}
