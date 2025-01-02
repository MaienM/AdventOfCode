aoc::setup!();

use derive_new::new;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scisors = 3,
}
impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scisors,
            _ => panic!("Unknown shape {value:?}."),
        }
    }
}

#[derive(Debug, Eq, PartialEq, new)]
struct Round {
    player: Shape,
    opponent: Shape,
}
impl Round {
    pub fn score(&self) -> u16 {
        match (self.player, self.opponent) {
            (Shape::Rock, Shape::Paper) => 1,
            (Shape::Paper, Shape::Scisors) => 2,
            (Shape::Scisors, Shape::Rock) => 3,

            (Shape::Rock, Shape::Rock) => 4,
            (Shape::Paper, Shape::Paper) => 5,
            (Shape::Scisors, Shape::Scisors) => 6,

            (Shape::Rock, Shape::Scisors) => 7,
            (Shape::Paper, Shape::Rock) => 8,
            (Shape::Scisors, Shape::Paper) => 9,
        }
    }
}

fn parse_input_part1(input: &str) -> Vec<Round> {
    parse!(input => {
        [rounds split on '\n' with
            { [opponent as Shape] " " p }
            => {
                let player = match p {
                    "X" => Shape::Rock,
                    "Y" => Shape::Paper,
                    "Z" => Shape::Scisors,
                    v => panic!("Invalid player choice {v:?}."),
                };
                Round { player, opponent }
            }
        ]
    } => rounds)
}

fn parse_input_part2(input: &str) -> Vec<Round> {
    parse!(input => {
        [rounds split on '\n' with
            { [opponent as Shape] " " p }
            => {
                let player = match p {
                    "X" => [Shape::Scisors, Shape::Rock, Shape::Paper][opponent as usize - 1], // lose
                    "Y" => [Shape::Rock, Shape::Paper, Shape::Scisors][opponent as usize - 1], // draw
                    "Z" => [Shape::Paper, Shape::Scisors, Shape::Rock][opponent as usize - 1], // win
                    v => panic!("Invalid round outcome {v:?}."),
                };
                Round { player, opponent }
            }
        ]
    } => rounds)
}

pub fn part1(input: &str) -> u16 {
    let rounds = parse_input_part1(input);
    rounds.iter().map(Round::score).sum()
}

pub fn part2(input: &str) -> u16 {
    let rounds = parse_input_part2(input);
    rounds.iter().map(Round::score).sum()
}

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 15, part2 = 12)]
    static EXAMPLE_INPUT: &str = "
        A Y
        B X
        C Z
    ";

    #[test]
    fn shape_score() {
        assert_eq!(Round::new(Shape::Rock, Shape::Rock).score(), 4);
        assert_eq!(Round::new(Shape::Rock, Shape::Paper).score(), 1);
        assert_eq!(Round::new(Shape::Rock, Shape::Scisors).score(), 7);
        assert_eq!(Round::new(Shape::Paper, Shape::Rock).score(), 8);
        assert_eq!(Round::new(Shape::Paper, Shape::Paper).score(), 5);
        assert_eq!(Round::new(Shape::Paper, Shape::Scisors).score(), 2);
        assert_eq!(Round::new(Shape::Scisors, Shape::Rock).score(), 3);
        assert_eq!(Round::new(Shape::Scisors, Shape::Paper).score(), 9);
        assert_eq!(Round::new(Shape::Scisors, Shape::Scisors).score(), 6);
    }

    #[test]
    fn example_parse_part1() {
        let actual = parse_input_part1(&EXAMPLE_INPUT);
        let expected = vec![
            Round::new(Shape::Paper, Shape::Rock),
            Round::new(Shape::Rock, Shape::Paper),
            Round::new(Shape::Scisors, Shape::Scisors),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_part2() {
        let actual = parse_input_part2(&EXAMPLE_INPUT);
        let expected = vec![
            Round::new(Shape::Rock, Shape::Rock),
            Round::new(Shape::Rock, Shape::Paper),
            Round::new(Shape::Rock, Shape::Scisors),
        ];
        assert_eq!(actual, expected);
    }
}
