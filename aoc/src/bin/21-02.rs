puzzle_lib::setup!(title = "Dive!");

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Invalid direction {value}"),
        }
    }
}

type Instruction = (Direction, u32);

fn parse_input(input: &str) -> Vec<Instruction> {
    parse!(input => {
        [instructions split on '\n' with
            { [direction as Direction] ' ' [distance as u32] }
            => (direction, distance)
        ]
    } => instructions)
}

pub fn part1(input: &str) -> i64 {
    let instructions = parse_input(input);
    let mut hpos = 0;
    let mut vpos = 0;
    for instruction in instructions {
        let (direction, distance) = instruction;
        match direction {
            Direction::Forward => hpos += distance,
            Direction::Down => vpos += distance,
            Direction::Up => vpos -= distance,
        }
    }
    (hpos * vpos).into()
}

pub fn part2(input: &str) -> i64 {
    let instructions = parse_input(input);
    let mut aim = 0;
    let mut hpos = 0;
    let mut vpos = 0;
    for instruction in instructions {
        let (direction, distance) = instruction;
        match direction {
            Direction::Forward => {
                hpos += distance;
                vpos += distance * aim;
            }
            Direction::Down => aim += distance,
            Direction::Up => aim -= distance,
        }
    }
    (hpos * vpos).into()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 150, part2 = 900)]
    static EXAMPLE_INPUT: &str = "
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            (Direction::Forward, 5),
            (Direction::Down, 5),
            (Direction::Forward, 8),
            (Direction::Up, 3),
            (Direction::Down, 8),
            (Direction::Forward, 2),
        ];
        assert_eq!(&actual, &expected);
    }
}
