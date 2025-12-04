puzzle_runner::register_chapter!(book = "2022", title = "Calorie Counting");

fn parse_input(input: &str) -> Vec<u32> {
    parse!(input => {
        [elves split on "\n\n" with
            { [nums split on '\n' as u32] }
            => nums.into_iter().sum()
        ]
    } => elves)
}

pub fn part1(input: &str) -> u32 {
    let data = parse_input(input);
    data.into_iter().max().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let mut data = parse_input(input);
    data.sort_unstable_by(|a, b| b.cmp(a));
    data[0] + data[1] + data[2]
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 24_000, part2 = 45_000)]
    static EXAMPLE_INPUT: &str = "
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
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![6000, 4000, 11_000, 24_000, 10_000];
        assert_eq!(actual, expected);
    }
}
