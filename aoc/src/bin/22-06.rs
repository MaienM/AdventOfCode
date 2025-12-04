puzzle_runner::register_chapter!(book = "2022", title = "Tuning Trouble");

use std::collections::HashSet;

fn find_marker(sequence: &str, length: usize) -> usize {
    for i in 0..=(sequence.len() - length) {
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

pub fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

pub fn part2(input: &str) -> usize {
    find_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 7, part2 = 19)]
    static EXAMPLE_INPUT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[example_input(part1 = 5, part2 = 23)]
    static EXAMPLE_INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

    #[example_input(part1 = 6, part2 = 23)]
    static EXAMPLE_INPUT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";

    #[example_input(part1 = 10, part2 = 29)]
    static EXAMPLE_INPUT_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

    #[example_input(part1 = 11, part2 = 26)]
    static EXAMPLE_INPUT_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
}
