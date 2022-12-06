use std::collections::HashSet;

use aoc::runner::*;

fn find_marker(sequence: String, length: usize) -> usize {
    for i in 0..(sequence.len() - length + 1) {
        if sequence[i..(i + length)]
            .chars()
            .collect::<HashSet<char>>()
            .len()
            == length
        {
            return i + length;
        }
    }
    panic!("Did not find marker.");
}

pub fn part1(input: String) -> usize {
    return find_marker(input, 4);
}

pub fn part2(input: String) -> usize {
    return find_marker(input, 14);
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
