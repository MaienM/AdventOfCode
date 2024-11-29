use aoc::{parse_number_list, runner::run};

fn parse_input(input: &str) -> Vec<i32> {
    return input
        .trim()
        .split("\n\n")
        .map(|block| parse_number_list(block, "\n").iter().sum())
        .collect();
}

pub fn part1(input: &str) -> i32 {
    let data = parse_input(input);
    data.into_iter().max().unwrap()
}

pub fn part2(input: &str) -> i32 {
    let mut data = parse_input(input);
    data.sort_unstable_by(|a, b| b.cmp(a));
    data[0] + data[1] + data[2]
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &str = "
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT);
        let expected = vec![6000, 4000, 11_000, 24_000, 10_000];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 24_000);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 45_000);
    }
}
