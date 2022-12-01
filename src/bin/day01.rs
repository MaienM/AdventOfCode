use aoc::{parse_number_list, runner::*};

fn parse_input(input: String) -> Vec<Vec<i32>> {
    return input
        .trim()
        .split("\n\n")
        .map(|block| parse_number_list(block.to_string(), "\n"))
        .collect();
}

pub fn part1(input: String) -> i32 {
    let data = parse_input(input);
    return data
        .into_iter()
        .map(|elf| elf.into_iter().sum())
        .max()
        .unwrap();
}

fn main() {
    run(part1, missing::<i64>);
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
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 24_000);
    }
}
