puzzle_lib::setup!(title = "Cube Conundrum");

#[derive(Debug, PartialEq)]
struct Game {
    id: u8,
    rounds: Vec<Round>,
}
#[derive(Debug, PartialEq, Default)]
struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

#[inline]
fn parse_round(input: &str) -> Round {
    parse!(input =>
        [parts split on ", " into iterator with
            { [count as u8] " " color }
            => (count, color)
        ]
    );
    let mut round = Round::default();
    for (count, color) in parts {
        match color {
            "red" => round.red = count,
            "green" => round.green = count,
            "blue" => round.blue = count,
            _ => panic!("Invalid color {color}."),
        };
    }
    round
}

fn parse_input(input: &str) -> Vec<Game> {
    parse!(input => {
        [games split on '\n' with
            { "Game " [id as u8] ": " [rounds split on "; " with (parse_round)] }
            => Game { id, rounds }
        ]
    } => games)
}

pub fn part1(input: &str) -> usize {
    let games = parse_input(input);
    games
        .into_iter()
        .filter(|g| {
            g.rounds
                .iter()
                .all(|r| r.red <= 12 && r.green <= 13 && r.blue <= 14)
        })
        .map(|g| g.id as usize)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let games = parse_input(input);
    games
        .into_iter()
        .map(|g| {
            let red = g.rounds.iter().map(|r| r.red).max().unwrap() as usize;
            let green = g.rounds.iter().map(|r| r.green).max().unwrap() as usize;
            let blue = g.rounds.iter().map(|r| r.blue).max().unwrap() as usize;
            red * green * blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 8, part2 = 2286)]
    static EXAMPLE_INPUT: &str = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = [
            Game {
                id: 1,
                rounds: vec![
                    Round {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Round {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Round {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 2,
                rounds: vec![
                    Round {
                        red: 0,
                        green: 2,
                        blue: 1,
                    },
                    Round {
                        red: 1,
                        green: 3,
                        blue: 4,
                    },
                    Round {
                        red: 0,
                        green: 1,
                        blue: 1,
                    },
                ],
            },
            Game {
                id: 3,
                rounds: vec![
                    Round {
                        red: 20,
                        green: 8,
                        blue: 6,
                    },
                    Round {
                        red: 4,
                        green: 13,
                        blue: 5,
                    },
                    Round {
                        red: 1,
                        green: 5,
                        blue: 0,
                    },
                ],
            },
            Game {
                id: 4,
                rounds: vec![
                    Round {
                        red: 3,
                        green: 1,
                        blue: 6,
                    },
                    Round {
                        red: 6,
                        green: 3,
                        blue: 0,
                    },
                    Round {
                        red: 14,
                        green: 3,
                        blue: 15,
                    },
                ],
            },
            Game {
                id: 5,
                rounds: vec![
                    Round {
                        red: 6,
                        green: 3,
                        blue: 1,
                    },
                    Round {
                        red: 1,
                        green: 2,
                        blue: 2,
                    },
                ],
            },
        ];
        assert_eq!(actual, expected);
    }
}
