use std::{collections::HashSet, ops::Add, sync::Arc};

use aoc::utils::{parse, point::Point2};
use rayon::prelude::*;

type Point = Point2;

type Map<T> = Vec<Vec<T>>;

#[derive(Debug, PartialEq)]
enum Tile {
    MirrorUpRight,
    MirrorUpLeft,
    SplitterHorizontal,
    SplitterVertical,
    None,
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::SplitterVertical,
            '-' => Self::SplitterHorizontal,
            '/' => Self::MirrorUpRight,
            '\\' => Self::MirrorUpLeft,
            '.' => Self::None,
            _ => panic!("Invalid tile {value:?}."),
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

fn parse_input(input: &str) -> Map<Tile> {
    parse!(input => {
        [map split on '\n' with [chars as Tile]]
    } => map)
}

fn track_beams(
    map: &Map<Tile>,
    bounds: &Point,
    energized: &mut Map<bool>,
    processed: &mut HashSet<(Point, Direction)>,
    start: Point,
    direction: Direction,
) {
    let is_new = processed.insert((start, direction));
    if !is_new {
        return;
    }

    let mut point = start;
    while point.x < bounds.x && point.y < bounds.y {
        energized[point.y][point.x] = true;

        #[allow(clippy::match_on_vec_items)]
        match map[point.y][point.x] {
            Tile::MirrorUpRight => {
                let direction = match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                };
                track_beams(
                    map,
                    bounds,
                    energized,
                    processed,
                    direction + point,
                    direction,
                );
                return;
            }
            Tile::MirrorUpLeft => {
                let direction = match direction {
                    Direction::North => Direction::West,
                    Direction::West => Direction::North,
                    Direction::South => Direction::East,
                    Direction::East => Direction::South,
                };
                track_beams(
                    map,
                    bounds,
                    energized,
                    processed,
                    direction + point,
                    direction,
                );
                return;
            }
            Tile::SplitterHorizontal
                if matches!(direction, Direction::North | Direction::South) =>
            {
                track_beams(map, bounds, energized, processed, point, Direction::East);
                track_beams(map, bounds, energized, processed, point, Direction::West);
                return;
            }
            Tile::SplitterVertical if matches!(direction, Direction::East | Direction::West) => {
                track_beams(map, bounds, energized, processed, point, Direction::North);
                track_beams(map, bounds, energized, processed, point, Direction::South);
                return;
            }
            _ => {}
        }

        point = direction + point;
    }
}

fn solve_from_position(
    map: &Map<Tile>,
    bounds: &Point,
    start: Point,
    direction: Direction,
) -> usize {
    let mut results: Map<bool> = (0..bounds.y)
        .map(|_| (0..bounds.x).map(|_| false).collect())
        .collect();
    track_beams(
        map,
        bounds,
        &mut results,
        &mut HashSet::new(),
        start,
        direction,
    );
    results
        .into_iter()
        .map(|row| row.into_iter().filter(|v| *v).count())
        .sum()
}

pub fn part1(input: &str) -> usize {
    let map = parse_input(input);
    let bounds = Point::new(map[0].len(), map.len());
    solve_from_position(&map, &bounds, Point::new(0, 0), Direction::East)
}

pub fn part2(input: &str) -> usize {
    let map = Arc::new(parse_input(input));
    let bounds = Arc::new(Point::new(map[0].len(), map.len()));

    let mut options = Vec::new();
    options.extend((0..bounds.x).map(|x| (Point::new(x, 0), Direction::South)));
    options.extend((0..bounds.x).map(|x| (Point::new(x, bounds.y), Direction::North)));
    options.extend((0..bounds.y).map(|y| (Point::new(0, y), Direction::East)));
    options.extend((0..bounds.y).map(|y| (Point::new(bounds.x, y), Direction::West)));
    options
        .into_par_iter()
        .map(|(point, direction)| solve_from_position(&map, &bounds, point, direction))
        .max()
        .unwrap()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 46, part2 = 51, test)]
    static EXAMPLE_INPUT: &str = r#"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "#;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            vec![
                Tile::None,
                Tile::SplitterVertical,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::MirrorUpLeft,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
            ],
            vec![
                Tile::SplitterVertical,
                Tile::None,
                Tile::SplitterHorizontal,
                Tile::None,
                Tile::MirrorUpLeft,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
            ],
            vec![
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::SplitterVertical,
                Tile::SplitterHorizontal,
                Tile::None,
                Tile::None,
                Tile::None,
            ],
            vec![
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::SplitterVertical,
                Tile::None,
            ],
            vec![
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
            ],
            vec![
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::MirrorUpLeft,
            ],
            vec![
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::MirrorUpRight,
                Tile::None,
                Tile::MirrorUpLeft,
                Tile::MirrorUpLeft,
                Tile::None,
                Tile::None,
            ],
            vec![
                Tile::None,
                Tile::SplitterHorizontal,
                Tile::None,
                Tile::SplitterHorizontal,
                Tile::MirrorUpRight,
                Tile::None,
                Tile::None,
                Tile::SplitterVertical,
                Tile::None,
                Tile::None,
            ],
            vec![
                Tile::None,
                Tile::SplitterVertical,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::SplitterHorizontal,
                Tile::SplitterVertical,
                Tile::None,
                Tile::MirrorUpLeft,
            ],
            vec![
                Tile::None,
                Tile::None,
                Tile::MirrorUpRight,
                Tile::MirrorUpRight,
                Tile::None,
                Tile::SplitterVertical,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
            ],
        ];
        assert_eq!(actual, expected);
    }
}
