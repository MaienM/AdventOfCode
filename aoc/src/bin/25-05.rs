puzzle_runner::register_chapter!(book = 2025, title = "Cafeteria");

use std::ops::RangeInclusive;

fn parse_input(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    parse!(input => {
        [ranges split on '\n' with { [start as usize] '-' [end as usize] } => start..=end]
        "\n\n"
        [ingredients split on '\n' as usize]
    } => (ranges, ingredients))
}

#[register_part]
fn part1(input: &str) -> usize {
    let (ranges, ingredients) = parse_input(input);
    ingredients
        .into_iter()
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count()
}

#[register_part]
fn part2(input: &str) -> usize {
    let (ranges, _) = parse_input(input);
    let mut ranges = ranges
        .into_iter()
        .map(|r| (*r.start(), *r.end()))
        .sorted_unstable()
        .chain([(usize::MAX, 0)]);
    let mut count = 0;
    let (mut start, mut end) = ranges.next().unwrap();
    for (nstart, nend) in ranges {
        if nstart <= end + 1 {
            end = usize::max(end, nend);
        } else {
            count += end - start + 1;
            start = nstart;
            end = nend;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 3, part2 = 14)]
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
