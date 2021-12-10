use aoc::*;

fn parse_input(input: String) -> Vec<i32> {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .parse()
                .unwrap_or_else(|_| panic!("Unable to parse line {:?}.", line))
        })
        .collect();
}

fn part1(input: String) -> i64 {
    let numbers = parse_input(input);
    let mut count = 0;
    let mut last_number = numbers[0];
    for number in numbers {
        if number > last_number {
            count = count + 1;
        }
        last_number = number;
    }
    return count;
}

fn part2(input: String) -> i64 {
    let numbers = parse_input(input);
    let mut count = 0;
    let mut last_numbers = [numbers[0], numbers[1], numbers[2]];
    let mut last_sum: i32 = last_numbers.iter().sum();
    for number in numbers {
        last_numbers = [last_numbers[1], last_numbers[2], number];
        let sum = last_numbers.iter().sum();
        if sum > last_sum {
            count = count + 1;
        }
        last_sum = sum;
    }
    return count;
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 7);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 5);
    }
}
