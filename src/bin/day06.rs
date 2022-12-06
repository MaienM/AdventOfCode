use std::collections::HashSet;

use aoc::runner::*;

pub fn part1(input: String) -> usize {
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

pub fn part2(input: String) -> usize {
    let input = input.trim();
    return input
        .chars()
        .zip(input.chars().skip(1))
        .zip(input.chars().skip(2))
        .zip(input.chars().skip(3))
        .zip(input.chars().skip(4))
        .zip(input.chars().skip(5))
        .zip(input.chars().skip(6))
        .zip(input.chars().skip(7))
        .zip(input.chars().skip(8))
        .zip(input.chars().skip(9))
        .zip(input.chars().skip(10))
        .zip(input.chars().skip(11))
        .zip(input.chars().skip(12))
        .zip(input.chars().skip(13))
        .map(
            |(
                ((((((((((((c1, c2), c3), c4), c5), c6), c7), c8), c9), c10), c11), c12), c13),
                c14,
            )| [c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14],
        )
        .enumerate()
        .find(|(_idx, chars)| HashSet::<char>::from(*chars).len() == 14)
        .unwrap()
        .0
        + 14;
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg".to_string()), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 26);
    }
}
