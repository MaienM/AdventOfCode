puzzle_lib::setup!(title = "Lavaduct Lagoon");

use std::{collections::HashSet, ops::RangeInclusive};

use puzzle_lib::point::{Direction2, Point2};

type Point = Point2<isize>;
type Direction = Direction2;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
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
            _ => panic!("Invalid corner {value:?}."),
        }
    }
}

fn parse_direction(value: &str) -> Direction {
    match value {
        "U" => Direction::North,
        "R" => Direction::East,
        "D" => Direction::South,
        "L" => Direction::West,
        _ => panic!("Invalid direction {value:?}."),
    }
}

#[derive(Debug, PartialEq)]
struct Instruction<'a> {
    direction: Direction,
    distance: usize,
    color: &'a str,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    parse!(input => {
        [instructions split on '\n' with
            { [direction with (parse_direction)] " " [distance as usize] " (#" color ")" }
            => Instruction { direction, distance, color }
        ]
    } => instructions)
}

fn verticals_to_ranges(verticals: &HashSet<isize>) -> Vec<RangeInclusive<isize>> {
    let mut iter = verticals.iter().sort_unstable();
    let mut ranges = Vec::new();
    while let (Some(start), Some(end)) = (iter.next(), iter.next()) {
        ranges.push(*start..=*end);
    }
    ranges
}

fn count_inside(verticals: &HashSet<isize>) -> usize {
    verticals_to_ranges(verticals)
        .into_iter()
        .map(|r| (r.end() - r.start() + 1) as usize)
        .sum()
}

fn count_overlap(old_verticals: &HashSet<isize>, new_verticals: &HashSet<isize>) -> usize {
    let mut count = 0;
    for old_range in verticals_to_ranges(old_verticals) {
        for new_range in verticals_to_ranges(new_verticals) {
            if old_range.contains(new_range.start())
                || old_range.contains(new_range.end())
                || new_range.contains(old_range.start())
                || new_range.contains(old_range.end())
            {
                let start = old_range.start().max(new_range.start());
                let end = old_range.end().min(new_range.end());
                count += (end - start + 1) as usize;
            }
        }
    }
    count
}

fn solve(instructions: &[Instruction]) -> usize {
    let mut map = Vec::new();
    let mut current = Point::new(0, 0);

    let mut last_direction = instructions.last().unwrap().direction;
    for instruction in instructions {
        let tile: Tile = (last_direction, instruction.direction).into();
        map.push((tile, current));

        last_direction = instruction.direction;
        match instruction.direction {
            Direction::North => {
                current.y = current
                    .y
                    .checked_sub(instruction.distance as isize)
                    .unwrap();
            }
            Direction::South => {
                current.y = current
                    .y
                    .checked_add(instruction.distance as isize)
                    .unwrap();
            }
            Direction::East => {
                current.x = current
                    .x
                    .checked_add(instruction.distance as isize)
                    .unwrap();
            }
            Direction::West => {
                current.x = current
                    .x
                    .checked_sub(instruction.distance as isize)
                    .unwrap();
            }
        };
    }

    let mut by_row: Vec<(isize, Vec<(Tile, Point)>)> = map
        .iter()
        .map(|(_, p)| p.y)
        .count_occurences()
        .into_keys()
        .map(|y| (y, map.iter().filter(|(_, p)| p.y == y).copied().collect()))
        .collect();
    by_row.sort_unstable_by_key(|(y, _)| *y);

    let mut sum = 0usize;
    let mut last_y = by_row.first().unwrap().0;
    let mut verticals = HashSet::new();
    for (y, row) in by_row {
        // Add size of rectangles formed until this point.
        sum += count_inside(&verticals) * (y - last_y + 1) as usize;

        // Update verticals.
        let mut new_verticals = verticals.clone();
        for (tile, point) in row {
            match tile {
                Tile::NorthEast | Tile::NorthWest => {
                    new_verticals.remove(&point.x);
                }
                Tile::SouthEast | Tile::SouthWest => {
                    new_verticals.insert(point.x);
                }
            }
        }

        // Remove overlap between old and new rectangles, else these would be counted twice.
        sum -= count_overlap(&verticals, &new_verticals);

        last_y = y;
        verticals = new_verticals;
    }
    sum
}

fn swap_instructions(instructions: Vec<Instruction>) -> Vec<Instruction> {
    instructions
        .into_iter()
        .map(|instruction| {
            let distance = usize::from_str_radix(&instruction.color[..5], 16).unwrap();
            let direction = match &instruction.color[5..6] {
                "0" => Direction::East,
                "1" => Direction::South,
                "2" => Direction::West,
                "3" => Direction::North,
                d => panic!("Invalid direction {d:?}."),
            };
            Instruction {
                direction,
                distance,
                color: instruction.color,
            }
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let instructions = parse_input(input);
    solve(&instructions)
}

pub fn part2(input: &str) -> usize {
    let instructions = parse_input(input);
    let instructions = swap_instructions(instructions);
    solve(&instructions)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 62, part2 = 952_408_144_115)]
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

    #[allow(clippy::too_many_lines)]
    #[test]
    fn swap_instructions() {
        let start = vec![
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
        let expected = vec![
            Instruction {
                direction: Direction::East,
                distance: 461_937,
                color: "70c710",
            },
            Instruction {
                direction: Direction::South,
                distance: 56_407,
                color: "0dc571",
            },
            Instruction {
                direction: Direction::East,
                distance: 356_671,
                color: "5713f0",
            },
            Instruction {
                direction: Direction::South,
                distance: 863_240,
                color: "d2c081",
            },
            Instruction {
                direction: Direction::East,
                distance: 367_720,
                color: "59c680",
            },
            Instruction {
                direction: Direction::South,
                distance: 266_681,
                color: "411b91",
            },
            Instruction {
                direction: Direction::West,
                distance: 577_262,
                color: "8ceee2",
            },
            Instruction {
                direction: Direction::North,
                distance: 829_975,
                color: "caa173",
            },
            Instruction {
                direction: Direction::West,
                distance: 112_010,
                color: "1b58a2",
            },
            Instruction {
                direction: Direction::South,
                distance: 829_975,
                color: "caa171",
            },
            Instruction {
                direction: Direction::West,
                distance: 491_645,
                color: "7807d2",
            },
            Instruction {
                direction: Direction::North,
                distance: 686_074,
                color: "a77fa3",
            },
            Instruction {
                direction: Direction::West,
                distance: 5411,
                color: "015232",
            },
            Instruction {
                direction: Direction::North,
                distance: 500_254,
                color: "7a21e3",
            },
        ];
        assert_eq!(super::swap_instructions(start), expected);
    }
}
