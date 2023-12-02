use aoc::generate_day_main;

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

generate_day_main!(part1, part2);

#[cfg(test)]
mod tests {
    use aoc::example;
    use macro_rules_attribute::apply;
    use pretty_assertions::assert_eq;

    use super::*;

    #[apply(example)]
    static EXAMPLE_INPUT_1: String = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ";

    #[apply(example)]
    static EXAMPLE_INPUT_2: String = "
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

    #[test]
    fn example_part1() {
        assert_eq!(part1(&EXAMPLE_INPUT_1), 142);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&EXAMPLE_INPUT_2), 281);
    }
}
