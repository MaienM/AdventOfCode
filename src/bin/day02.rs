use aoc::runner::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scisors = 3,
}
impl Shape {
    pub fn score(self, other: Shape) -> u16 {
        let result = (self as i16 - other as i16 + 4) % 3 - 1; // -1, 0, 1
        return self as u16 + ((result + 1) * 3) as u16;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Round {
    player: Shape,
    opponent: Shape,
}

fn parse_input(input: String) -> Vec<Round> {
    return input
        .trim()
        .split("\n")
        .into_iter()
        .map(|line| {
            let mut parts = line.trim().splitn(2, " ");
            let opponent = match parts.next() {
                Some("A") => Shape::Rock,
                Some("B") => Shape::Paper,
                Some("C") => Shape::Scisors,
                v => panic!("Invalid opponent choice {:?}.", v),
            };
            let player = match parts.next() {
                Some("X") => Shape::Rock,
                Some("Y") => Shape::Paper,
                Some("Z") => Shape::Scisors,
                v => panic!("Invalid player choice {:?}.", v),
            };
            return Round { player, opponent };
        })
        .collect();
}

pub fn part1(input: String) -> u16 {
    let rounds = parse_input(input);
    return rounds
        .into_iter()
        .map(|round| round.player.score(round.opponent))
        .sum();
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        A Y
        B X
        C Z
    ";

    #[test]
    fn shape_score() {
        assert_eq!(Shape::Rock.score(Shape::Rock), 4);
        assert_eq!(Shape::Rock.score(Shape::Paper), 1);
        assert_eq!(Shape::Rock.score(Shape::Scisors), 7);
        assert_eq!(Shape::Paper.score(Shape::Rock), 8);
        assert_eq!(Shape::Paper.score(Shape::Paper), 5);
        assert_eq!(Shape::Paper.score(Shape::Scisors), 2);
        assert_eq!(Shape::Scisors.score(Shape::Rock), 3);
        assert_eq!(Shape::Scisors.score(Shape::Paper), 9);
        assert_eq!(Shape::Scisors.score(Shape::Scisors), 6);
    }

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![
            Round {
                player: Shape::Paper,
                opponent: Shape::Rock,
            },
            Round {
                player: Shape::Rock,
                opponent: Shape::Paper,
            },
            Round {
                player: Shape::Scisors,
                opponent: Shape::Scisors,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 15);
    }
}
