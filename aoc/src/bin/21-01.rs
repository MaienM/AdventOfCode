puzzle_runner::register_chapter!(book = 2021, title = "Sonar Sweep");

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<u16> {
    parse!(input => {
        [nums split on '\n' as u16]
    } => nums)
}

fn count_incrementing(nums: &[u16]) -> usize {
    nums.iter()
        .tuple_windows()
        .filter(|(prev_depth, depth)| depth > prev_depth)
        .count()
}

#[register_part]
fn part1(input: &str) -> usize {
    let nums = parse_input(input);
    count_incrementing(&nums)
}

#[register_part]
fn part2(input: &str) -> usize {
    let nums = parse_input(input);
    let sums: Vec<_> = nums
        .into_iter()
        .tuple_windows()
        .map(|(v1, v2, v3)| v1 + v2 + v3)
        .collect();
    count_incrementing(&sums)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 7, part2 = 5)]
    static EXAMPLE_INPUT: &str = "
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    ";
}
