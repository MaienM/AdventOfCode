fn parse_input(input: &str) -> Vec<u32> {
    input
        .split('\n')
        .map(|line| {
            let mut iter = line.chars().filter_map(|c| c.to_digit(10));
            let first = iter.next().unwrap();
            let last = iter.last().unwrap_or(first);
            first * 10 + last
        })
        .collect()
}

fn parse_input_with_words(input: &str) -> Vec<u32> {
    parse_input(
        &input
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "4")
            .replace("five", "5e")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "e8t")
            .replace("nine", "n9e"),
    )
}

pub fn part1(input: &str) -> u32 {
    let input = parse_input(input);
    input.iter().sum()
}

pub fn part2(input: &str) -> u32 {
    let input = parse_input_with_words(input);
    input.iter().sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 142, test)]
    static EXAMPLE_INPUT_1: &str = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ";

    #[example_input(part2 = 281, test)]
    static EXAMPLE_INPUT_2: &str = "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = vec![12, 38, 15, 77];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_with_words() {
        let actual = parse_input_with_words(&EXAMPLE_INPUT_2);
        let expected = vec![29, 83, 13, 24, 42, 14, 76];
        assert_eq!(actual, expected);
    }
}
