puzzle_runner::register_chapter!(title = "Rock Paper Scissors");

use derive_new::new;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
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
            (Shape::Paper, Shape::Scissors) => 2,
            (Shape::Scissors, Shape::Rock) => 3,

            (Shape::Rock, Shape::Rock) => 4,
            (Shape::Paper, Shape::Paper) => 5,
            (Shape::Scissors, Shape::Scissors) => 6,

            (Shape::Rock, Shape::Scissors) => 7,
            (Shape::Paper, Shape::Rock) => 8,
            (Shape::Scissors, Shape::Paper) => 9,
        }
    }
}

fn parse_input_part1(input: &str) -> Vec<Round> {
    parse!(input => {
        [rounds split on '\n' with
            {
                [opponent match {
                    "A" => Shape::Rock,
                    "B" => Shape::Paper,
                    "C" => Shape::Scissors,
                }]
                " "
                [player match {
                    "X" => Shape::Rock,
                    "Y" => Shape::Paper,
                    "Z" => Shape::Scissors,
                }]
            }
            => Round { player, opponent }
        ]
    } => rounds)
}

fn parse_input_part2(input: &str) -> Vec<Round> {
    parse!(input => {
        [rounds split on '\n' with
            {
                [opponent match {
                    "A" => Shape::Rock,
                    "B" => Shape::Paper,
                    "C" => Shape::Scissors,
                }]
                " "
                [player match {
                    "X" => [Shape::Scissors, Shape::Rock, Shape::Paper][opponent as usize - 1], // lose
                    "Y" => [Shape::Rock, Shape::Paper, Shape::Scissors][opponent as usize - 1], // draw
                    "Z" => [Shape::Paper, Shape::Scissors, Shape::Rock][opponent as usize - 1], // win
                }]
            }
            => Round { player, opponent }
        ]
    } => rounds)
}

#[register_part]
fn part1(input: &str) -> u16 {
    let rounds = parse_input_part1(input);
    rounds.iter().map(Round::score).sum()
}

#[register_part]
fn part2(input: &str) -> u16 {
    let rounds = parse_input_part2(input);
    rounds.iter().map(Round::score).sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

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
        assert_eq!(Round::new(Shape::Rock, Shape::Scissors).score(), 7);
        assert_eq!(Round::new(Shape::Paper, Shape::Rock).score(), 8);
        assert_eq!(Round::new(Shape::Paper, Shape::Paper).score(), 5);
        assert_eq!(Round::new(Shape::Paper, Shape::Scissors).score(), 2);
        assert_eq!(Round::new(Shape::Scissors, Shape::Rock).score(), 3);
        assert_eq!(Round::new(Shape::Scissors, Shape::Paper).score(), 9);
        assert_eq!(Round::new(Shape::Scissors, Shape::Scissors).score(), 6);
    }

    #[test]
    fn example_parse_part1() {
        let actual = parse_input_part1(&EXAMPLE_INPUT);
        let expected = vec![
            Round::new(Shape::Paper, Shape::Rock),
            Round::new(Shape::Rock, Shape::Paper),
            Round::new(Shape::Scissors, Shape::Scissors),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_part2() {
        let actual = parse_input_part2(&EXAMPLE_INPUT);
        let expected = vec![
            Round::new(Shape::Rock, Shape::Rock),
            Round::new(Shape::Rock, Shape::Paper),
            Round::new(Shape::Rock, Shape::Scissors),
        ];
        assert_eq!(actual, expected);
    }
}
