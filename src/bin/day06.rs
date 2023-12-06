use aoc::{generate_day_main, splitn};

#[derive(Debug, PartialEq)]
struct Race {
    duration: usize,
    record: usize,
}

fn parse_input(input: &str) -> Vec<Race> {
    let (line_time, line_distance) = splitn!(input, '\n', str, str);
    let times = line_time.split(' ').filter_map(|p| p.parse().ok());
    let distances = line_distance.split(' ').filter_map(|p| p.parse().ok());
    times
        .zip(distances)
        .map(|(duration, record)| Race { duration, record })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let races = parse_input(input);
    let mut result = 1;
    for race in races {
        let wins = (1..race.duration)
            .map(|charge| charge * (race.duration - charge))
            .filter(|s| s > &race.record)
            .count();
        result *= wins;
    }
    result
}

generate_day_main!(part1);

#[cfg(test)]
mod tests {
    use aoc::example;
    use macro_rules_attribute::apply;
    use pretty_assertions::assert_eq;

    use super::*;

    #[apply(example)]
    static EXAMPLE_INPUT: String = "
        Time:      7  15   30
        Distance:  9  40  200
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Race {
                duration: 7,
                record: 9,
            },
            Race {
                duration: 15,
                record: 40,
            },
            Race {
                duration: 30,
                record: 200,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(&EXAMPLE_INPUT), 288);
    }
}
