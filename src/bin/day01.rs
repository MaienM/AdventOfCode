use aoc::{parse_number_list, runner::*};

fn parse_input(input: String) -> Vec<i32> {
    return input
        .trim()
        .split("\n\n")
        .map(|block| parse_number_list(block.to_string(), "\n").iter().sum())
        .collect();
}

pub fn part1(input: String) -> i32 {
    let data = parse_input(input);
    return data.into_iter().max().unwrap();
}

pub fn part2(input: String) -> i32 {
    let mut data = parse_input(input);
    data.sort_unstable_by(|a, b| b.cmp(a));
    return data[0] + data[1] + data[2];
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
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
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![6000, 4000, 11_000, 24_000, 10_000];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 24_000);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 45_000);
    }
}
