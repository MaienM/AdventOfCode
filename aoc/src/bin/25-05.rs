puzzle_runner::register_chapter!(book = "2025", title = "Cafeteria");

use std::ops::RangeInclusive;

fn parse_input(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    parse!(input => {
        [ranges split on '\n' with { [start as usize] '-' [end as usize] } => start..=end]
        "\n\n"
        [ingredients split on '\n' as usize]
    } => (ranges, ingredients))
}

pub fn part1(input: &str) -> usize {
    let (ranges, ingredients) = parse_input(input);
    ingredients
        .into_iter()
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 3)]
    static EXAMPLE_INPUT: &str = "
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    ";
}
