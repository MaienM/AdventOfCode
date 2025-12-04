puzzle_runner::register_chapter!(book = "2025", title = "Gift Shop");

use std::ops::RangeInclusive;

fn parse_input(input: &str) -> Vec<RangeInclusive<usize>> {
    parse!(input => {
        [ranges split on ',' with
            { [left as usize] '-' [right as usize] }
            => left..=right
        ]
    } => ranges)
}

fn valid_id_1(id: usize) -> bool {
    let len = id.ilog10() + 1;
    if len % 2 == 1 {
        return true;
    }
    let mul = 10usize.pow(len / 2);
    id / mul != id % mul
}

pub fn part1(input: &str) -> usize {
    let ranges = parse_input(input);
    ranges
        .into_iter()
        .map(|range| range.filter(|r| !valid_id_1(*r)).sum::<usize>())
        .sum()
}

fn valid_id_2(id: usize) -> bool {
    let len = id.ilog10() + 1;
    'top: for i in 1..=(len / 2) {
        if !len.is_multiple_of(i) {
            continue;
        }
        let mul = 10usize.pow(i);
        let end = id % mul;
        let mut num = id / mul;
        while num > 0 {
            if num % mul != end {
                continue 'top;
            }
            num /= mul;
        }
        return false;
    }
    true
}

pub fn part2(input: &str) -> usize {
    let ranges = parse_input(input);
    ranges
        .into_iter()
        .map(|range| range.filter(|r| !valid_id_2(*r)).sum::<usize>())
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 1_227_775_554, part2 = 4_174_379_265)]
    static EXAMPLE_INPUT: &str = "
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
    ";
}
