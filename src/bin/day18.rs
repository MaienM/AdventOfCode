use std::ops::Add;

use aoc::utils::{ext::iter::IterExt, parse, point::Point2};

type Point = Point2;

#[derive(Debug, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}
impl From<(Direction, Direction)> for Tile {
    fn from(value: (Direction, Direction)) -> Self {
        match value {
            (Direction::South, Direction::East) | (Direction::West, Direction::North) => {
                Tile::NorthEast
            }
            (Direction::North, Direction::East) | (Direction::West, Direction::South) => {
                Tile::SouthEast
            }
            (Direction::North, Direction::West) | (Direction::East, Direction::South) => {
                Tile::SouthWest
            }
            (Direction::South, Direction::West) | (Direction::East, Direction::North) => {
                Tile::NorthWest
            }
            (Direction::North, Direction::North) | (Direction::South, Direction::South) => {
                Tile::Vertical
            }
            (Direction::East, Direction::East) | (Direction::West, Direction::West) => {
                Tile::Horizontal
            }
            _ => panic!("Invalid corner {value:?}."),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Add<Point> for Direction {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        match self {
            Direction::North => Point::new(rhs.x, rhs.y.wrapping_sub(1)),
            Direction::East => Point::new(rhs.x + 1, rhs.y),
            Direction::South => Point::new(rhs.x, rhs.y + 1),
            Direction::West => Point::new(rhs.x.wrapping_sub(1), rhs.y),
        }
    }
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Direction::North,
            "R" => Direction::East,
            "D" => Direction::South,
            "L" => Direction::West,
            _ => panic!("Invalid direction {value:?}."),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction<'a> {
    direction: Direction,
    distance: u8,
    color: &'a str,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    parse!(input => {
        [instructions split on '\n' with
            { direction " " [distance as u8] " (#" color ")" }
            => Instruction {
                direction: direction.into(),
                distance,
                color,
            }
        ]
    } => instructions)
}

const OFFSET: usize = 100_000;

pub fn part1(input: &str) -> usize {
    let instructions = parse_input(input);

    let mut map = Vec::new();
    let mut current = Point::new(OFFSET, OFFSET);

    let mut last_direction = instructions.last().unwrap().direction;
    for instruction in instructions {
        for _ in 0..(instruction.distance) {
            let tile: Tile = (last_direction, instruction.direction).into();
            map.push((tile, current));

            last_direction = instruction.direction;
            current = instruction.direction + current;
        }
    }

    let bounds = (
        map.iter().map(|(_, p)| p.y).min().unwrap(),
        map.iter().map(|(_, p)| p.y).max().unwrap(),
    );
    let mut sum = 0;
    for y in (bounds.0)..=(bounds.1) {
        let row = map
            .iter()
            .filter(|(_, p)| p.y == y)
            .sort_unstable_by_key(|(_, p)| p.x);
        let mut inside_start = None;
        for (tile, point) in row {
            match (tile, inside_start) {
                (Tile::Vertical | Tile::NorthEast | Tile::NorthWest, None) => {
                    inside_start = Some(point.x);
                    sum += 1;
                }
                (Tile::Vertical | Tile::NorthEast | Tile::NorthWest, Some(start)) => {
                    sum += point.x - start;
                    inside_start = None;
                }
                (_, None) => {
                    sum += 1;
                }
                _ => {}
            }
        }
    }
    sum
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 62, test)]
    static EXAMPLE_INPUT: &str = "
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Instruction {
                direction: Direction::East,
                distance: 6,
                color: "70c710",
            },
            Instruction {
                direction: Direction::South,
                distance: 5,
                color: "0dc571",
            },
            Instruction {
                direction: Direction::West,
                distance: 2,
                color: "5713f0",
            },
            Instruction {
                direction: Direction::South,
                distance: 2,
                color: "d2c081",
            },
            Instruction {
                direction: Direction::East,
                distance: 2,
                color: "59c680",
            },
            Instruction {
                direction: Direction::South,
                distance: 2,
                color: "411b91",
            },
            Instruction {
                direction: Direction::West,
                distance: 5,
                color: "8ceee2",
            },
            Instruction {
                direction: Direction::North,
                distance: 2,
                color: "caa173",
            },
            Instruction {
                direction: Direction::West,
                distance: 1,
                color: "1b58a2",
            },
            Instruction {
                direction: Direction::North,
                distance: 2,
                color: "caa171",
            },
            Instruction {
                direction: Direction::East,
                distance: 2,
                color: "7807d2",
            },
            Instruction {
                direction: Direction::North,
                distance: 3,
                color: "a77fa3",
            },
            Instruction {
                direction: Direction::West,
                distance: 2,
                color: "015232",
            },
            Instruction {
                direction: Direction::North,
                distance: 2,
                color: "7a21e3",
            },
        ];
        assert_eq!(actual, expected);
    }
}
