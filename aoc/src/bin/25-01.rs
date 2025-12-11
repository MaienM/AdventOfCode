puzzle_runner::register_chapter!(book = 2025, title = "Secret Entrance");

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<(Direction, u16)> {
    parse!(input => {
        [rotations split on '\n' with
            {
                [direction take 1 match {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                }]
                [steps as u16]
            }
            => (direction, steps)
        ]
    } => rotations)
}

#[register_part]
fn part1(input: &str) -> u16 {
    let instructions = parse_input(input);
    let mut location = 50;
    let mut hits = 0;
    for (direction, steps) in instructions {
        match direction {
            Direction::Left => {
                location = (location + 1000 - steps) % 100;
            }
            Direction::Right => {
                location = (location + steps) % 100;
            }
        }
        if location == 0 {
            hits += 1;
        }
    }
    hits
}

#[register_part]
fn part2(input: &str) -> u16 {
    let instructions = parse_input(input);
    let mut location = 50;
    let mut hits = 0;
    for (direction, steps) in instructions {
        match direction {
            Direction::Left => {
                if location == 0 {
                    hits -= 1;
                }
                location += 1000 - steps;
                while location <= 1000 {
                    location += 100;
                    hits += 1;
                }
                location %= 100;
            }
            Direction::Right => {
                location += steps;
                while location >= 100 {
                    location -= 100;
                    hits += 1;
                }
            }
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 3, part2 = 6)]
    static EXAMPLE_INPUT: &str = "
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            (Direction::Left, 68),
            (Direction::Left, 30),
            (Direction::Right, 48),
            (Direction::Left, 5),
            (Direction::Right, 60),
            (Direction::Left, 55),
            (Direction::Left, 1),
            (Direction::Left, 99),
            (Direction::Right, 14),
            (Direction::Left, 82),
        ];
        assert_eq!(actual, expected);
    }
}
