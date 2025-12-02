puzzle_lib::setup!(title = "Gift Shop");

use std::ops::RangeInclusive;

fn parse_input(input: &str) -> Vec<RangeInclusive<usize>> {
    parse!(input => {
        [ranges split on ',' with
            { [left as usize] '-' [right as usize] }
            => left..=right
        ]
    } => ranges)
}

fn valid_id(id: usize) -> bool {
    let string = id.to_string();
    let len = string.len();
    if len % 2 == 1 {
        return true;
    }
    string[0..(len / 2)] != string[(len / 2)..]
}

pub fn part1(input: &str) -> usize {
    let ranges = parse_input(input);
    ranges
        .into_iter()
        .map(|range| range.filter(|r| !valid_id(*r)).sum::<usize>())
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 1_227_775_554)]
    static EXAMPLE_INPUT: &str = "
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
    ";
}
