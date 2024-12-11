use aoc::utils::parse;

fn parse_input(input: &str) -> Vec<usize> {
    parse!(input => { [num split as usize] } => num)
}

fn cycle(nums: Vec<usize>) -> Vec<usize> {
    nums.into_iter()
        .flat_map(|num| {
            if num == 0 {
                return vec![1];
            }
            let s = num.to_string();
            if s.len() % 2 == 0 {
                vec![
                    s[..(s.len() / 2)].parse().unwrap(),
                    s[(s.len() / 2)..].parse().unwrap(),
                ]
            } else {
                vec![num * 2024]
            }
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let mut stones = parse_input(input);
    for _ in 0..25 {
        stones = cycle(stones);
    }
    stones.len()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input()]
    static EXAMPLE_INPUT_1: &str = "0 1 10 99 999";

    #[example_input(part1 = 55_312)]
    static EXAMPLE_INPUT_2: &str = "125 17";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = vec![0, 1, 10, 99, 999];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_cycle() {
        let mut actual = vec![125, 17];

        actual = cycle(actual);
        let expected = vec![253_000, 1, 7];
        assert_eq!(actual, expected);

        actual = cycle(actual);
        let expected = vec![253, 0, 2024, 14168];
        assert_eq!(actual, expected);

        actual = cycle(actual);
        let expected = vec![512_072, 1, 20, 24, 28_676_032];
        assert_eq!(actual, expected);

        actual = cycle(actual);
        let expected = vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032];
        assert_eq!(actual, expected);

        actual = cycle(actual);
        let expected = vec![103_6288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32];
        assert_eq!(actual, expected);

        actual = cycle(actual);
        let expected = vec![
            2_097_446_912,
            14_168,
            4048,
            2,
            0,
            2,
            4,
            40,
            48,
            2024,
            40,
            48,
            80,
            96,
            2,
            8,
            6,
            7,
            6,
            0,
            3,
            2,
        ];
        assert_eq!(actual, expected);
    }
}
