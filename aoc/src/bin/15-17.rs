puzzle_runner::register_chapter!(book = "2015", title = "No Such Thing as Too Much");

use std::cmp::Ordering;

fn parse_input(input: &str) -> Vec<u8> {
    parse!(input => {
        [nums split on '\n' as u8]
    } => nums)
}

fn count_options(containers: &[u8], left: u8) -> u16 {
    if left == 0 {
        return 1;
    }
    if containers.is_empty() {
        return 0;
    }

    let without_current = count_options(&containers[1..], left);
    if let Some(left) = left.checked_sub(containers[0]) {
        without_current + count_options(&containers[1..], left)
    } else {
        without_current
    }
}

fn part1impl(input: &str, liters: u8) -> u16 {
    let containers = parse_input(input);
    count_options(&containers, liters)
}

#[register_part]
fn part1(input: &str) -> u16 {
    part1impl(input, 150)
}

fn min_options(containers: &[u8], left: u8, used: u8) -> (u8, u16) {
    if left == 0 {
        return (used, 1);
    }
    if containers.is_empty() {
        return (u8::MAX, 0);
    }

    let without_current = min_options(&containers[1..], left, used);
    if let Some(left) = left.checked_sub(containers[0]) {
        let with_current = min_options(&containers[1..], left, used + 1);
        match with_current.0.cmp(&without_current.0) {
            Ordering::Less => with_current,
            Ordering::Equal => (with_current.0, with_current.1 + without_current.1),
            Ordering::Greater => without_current,
        }
    } else {
        without_current
    }
}

fn part2impl(input: &str, liters: u8) -> u16 {
    let containers = parse_input(input);
    min_options(&containers, liters, 0).1
}

#[register_part]
fn part2(input: &str) -> u16 {
    part2impl(input, 150)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input]
    static EXAMPLE_INPUT: &str = "
        20
        15
        10
        5
        5
    ";

    #[test]
    fn example_input_part1() {
        assert_eq!(part1impl(&EXAMPLE_INPUT, 25), 4);
    }

    #[test]
    fn example_input_part2() {
        assert_eq!(part2impl(&EXAMPLE_INPUT, 25), 3);
    }
}
