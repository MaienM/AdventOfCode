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

fn parse_input_part1(input: String) -> Vec<Round> {
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

fn parse_input_part2(input: String) -> Vec<Round> {
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
                Some("X") => [Shape::Scisors, Shape::Rock, Shape::Paper][opponent as usize - 1], // lose
                Some("Y") => [Shape::Rock, Shape::Paper, Shape::Scisors][opponent as usize - 1], // draw
                Some("Z") => [Shape::Paper, Shape::Scisors, Shape::Rock][opponent as usize - 1], // win
                v => panic!("Invalid round outcome {:?}.", v),
            };
            return Round { player, opponent };
        })
        .collect();
}

fn get_score(rounds: &Vec<Round>) -> u16 {
    return rounds
        .into_iter()
        .map(|round| round.player.score(round.opponent))
        .sum();
}

pub fn part1(input: String) -> u16 {
    let rounds = parse_input_part1(input);
    return get_score(&rounds);
}

pub fn part2(input: String) -> u16 {
    let rounds = parse_input_part2(input);
    return get_score(&rounds);
}

fn main() {
    run(part1, part2);
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
    fn example_parse_part1() {
        let actual = parse_input_part1(EXAMPLE_INPUT.to_string());
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
    fn example_parse_part2() {
        let actual = parse_input_part2(EXAMPLE_INPUT.to_string());
        let expected = vec![
            Round {
                player: Shape::Rock,
                opponent: Shape::Rock,
            },
            Round {
                player: Shape::Rock,
                opponent: Shape::Paper,
            },
            Round {
                player: Shape::Rock,
                opponent: Shape::Scisors,
            },
        ];
        assert_eq!(actual, expected);
    }
    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 15);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 12);
    }
}
