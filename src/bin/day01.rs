use aoc::generate_day_main;

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let mut iter = line.chars().filter_map(|c| c.to_digit(10));
            let first = iter.next().unwrap();
            let last = iter.last().unwrap_or(first);
            (first, last)
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let input = parse_input(input);
    let mut sum = 0;
    for (first, last) in input {
        sum += (first * 10) + last;
    }
    sum
}

generate_day_main!(part1);

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &str = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT);
        let expected = vec![(1, 2), (3, 8), (1, 5), (7, 7)];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 142);
    }
}
