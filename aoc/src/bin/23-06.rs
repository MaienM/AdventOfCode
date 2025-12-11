puzzle_runner::register_chapter!(book = 2023, title = "Wait For It");

#[derive(Debug, PartialEq)]
struct Race {
    duration: usize,
    record: usize,
}

fn parse_input(input: &str) -> Vec<Race> {
    parse!(input => "Time:" [times split try as usize] "\nDistance:" [distances split try as usize]);
    times
        .into_iter()
        .zip(distances)
        .map(|(duration, record)| Race { duration, record })
        .collect()
}

fn find_win_options(race: &Race) -> usize {
    let first = (1..race.duration)
        .partition_point(|charge| charge * (race.duration - charge) > race.record)
        .unwrap();
    race.duration + 1 - 2 * first
}

#[register_part]
fn part1(input: &str) -> usize {
    let races = parse_input(input);
    let mut result = 1;
    for race in races {
        result *= find_win_options(&race);
    }
    result
}

#[register_part]
fn part2(input: &str) -> usize {
    let races = parse_input(&input.replace(' ', "").replace(':', ": "));
    find_win_options(races.first().unwrap())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 288, part2 = 71_503)]
    static EXAMPLE_INPUT: &str = "
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
    fn test_find_win_options() {
        assert_eq!(
            find_win_options(&Race {
                duration: 7,
                record: 9,
            }),
            4
        );
        assert_eq!(
            find_win_options(&Race {
                duration: 15,
                record: 40,
            }),
            8
        );
        assert_eq!(
            find_win_options(&Race {
                duration: 30,
                record: 200,
            }),
            9
        );
    }
}
