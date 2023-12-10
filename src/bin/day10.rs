use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use aoc::point::Point2;

type Point = Point2<usize>;

#[derive(Debug, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
    None,
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            'S' => Tile::Start,
            '.' => Tile::None,
            _ => panic!("Unknown character {value:?}."),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
impl Direction {
    fn reverse(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .split('\n')
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

fn extract_start(map: &mut [Vec<Tile>]) -> Point {
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| {
                if tile == &Tile::Start {
                    Some(Point::new(x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();
    let connections: Vec<_> = start
        .neighbours_ortho()
        .into_iter()
        .filter(|point| {
            let tile = &map[point.y][point.x];
            match tile {
                Tile::Vertical if point.y != start.y => true,
                Tile::Horizontal if point.x != start.x => true,
                Tile::NorthEast if point.y > start.y || point.x < start.x => true,
                Tile::NorthWest if point.y > start.y || point.x > start.x => true,
                Tile::SouthEast if point.y < start.y || point.x < start.x => true,
                Tile::SouthWest if point.y < start.y || point.x > start.x => true,
                _ => false,
            }
        })
        .collect();

    map[start.y][start.x] = if connections.contains(&(Direction::North + start)) {
        if connections.contains(&(Direction::West + start)) {
            Tile::NorthWest
        } else if connections.contains(&(Direction::East + start)) {
            Tile::NorthEast
        } else {
            Tile::Vertical
        }
    } else if connections.contains(&(Direction::South + start)) {
        if connections.contains(&(Direction::West + start)) {
            Tile::SouthWest
        } else {
            Tile::SouthEast
        }
    } else {
        Tile::Horizontal
    };

    start
}

fn find_loop(map: &[Vec<Tile>], start: Point) -> Vec<Point> {
    let mut mainloop = Vec::new();
    let mut prev = start;
    let mut curr = (start, &map[start.y][start.x]);
    loop {
        let (point, tile) = curr;
        let direction = match tile {
            Tile::Vertical => {
                if prev.y < point.y {
                    Direction::South
                } else {
                    Direction::North
                }
            }
            Tile::Horizontal => {
                if prev.x < point.x {
                    Direction::East
                } else {
                    Direction::West
                }
            }
            Tile::NorthEast => {
                if prev.x == point.x {
                    Direction::East
                } else {
                    Direction::North
                }
            }
            Tile::NorthWest => {
                if prev.x == point.x {
                    Direction::West
                } else {
                    Direction::North
                }
            }
            Tile::SouthEast => {
                if prev.x == point.x {
                    Direction::East
                } else {
                    Direction::South
                }
            }
            Tile::SouthWest => {
                if prev.x == point.x {
                    Direction::West
                } else {
                    Direction::South
                }
            }
            _ => panic!("Ended up on {tile:?} at {point:?}, cannot proceed."),
        };
        let next = direction + point;
        prev = point;
        curr = (next, &map[next.y][next.x]);

        mainloop.push(next);
        if next == start {
            break;
        }
    }
    mainloop
}

#[allow(clippy::too_many_lines)]
fn collect_reachable(
    map: &HashMap<Point, &Tile>,
    bounds: &(usize, usize),
    visited: &mut Vec<Point>,
    reachable: &mut HashSet<Point>,
    point: Point,
    direction: Direction,
    squeeze: Option<Direction>,
) {
    reachable.insert(point);

    let tile = map.get(&point);
    #[rustfmt::skip]
    #[allow(clippy::match_same_arms)]
    let options = match (tile, direction, squeeze) {
        //
        // Horizontal.
        //

        // Moving alongside pipe.
        (
            Some(Tile::Horizontal),
            Direction::East | Direction::West,
            Some(Direction::North | Direction::South),
        ) => vec![
            (direction, squeeze),
            (squeeze.unwrap(), None),
        ],
        // Approaching from the side.
        (
            Some(Tile::Horizontal),
            Direction::North | Direction::South,
            _,
        ) => vec![
            (Direction::East, Some(direction.reverse())),
            (Direction::West, Some(direction.reverse())),
        ],

        //
        // Vertical.
        //

        // Moving alongside pipe.
        (
            Some(Tile::Vertical),
            Direction::North | Direction::South,
            Some(Direction::East | Direction::West),
        ) => vec![
            (direction, squeeze),
            (squeeze.unwrap(), None),
        ],
        // Approaching from the side.
        (
            Some(Tile::Vertical),
            Direction::East | Direction::West,
            _,
        ) => vec![
            (Direction::North, Some(direction.reverse())),
            (Direction::South, Some(direction.reverse())),
        ],

        //
        // NorthEast.
        //

        // Inside bend of corner.
        (
            Some(Tile::NorthEast),
            Direction::South,
            Some(Direction::East),
        ) => vec![
            (Direction::East, Some(Direction::North)),
        ],
        (
            Some(Tile::NorthEast),
            Direction::West,
            Some(Direction::North),
        ) => vec![
            (Direction::North, Some(Direction::East)),
        ],
        // Outside bend of corner.
        (
            Some(Tile::NorthEast),
            Direction::South,
            _,
        ) => vec![
            (Direction::East, Some(Direction::South)),
            (Direction::South, None),
            (Direction::West, None),
        ],
        (
            Some(Tile::NorthEast),
            Direction::West,
            _,
        ) => vec![
            (Direction::North, Some(Direction::West)),
            (Direction::West, None),
            (Direction::South, None),
        ],
        // Approaching from the side.
        (
            Some(Tile::NorthEast),
            Direction::North,
            _,
        ) => vec![
            (Direction::East, Some(Direction::South)),
            (Direction::West, None),
            (Direction::North, Some(Direction::West)),
        ],
        (
            Some(Tile::NorthEast),
            Direction::East,
            _,
        ) => vec![
            (Direction::North, Some(Direction::West)),
            (Direction::South, None),
            (Direction::East, Some(Direction::South)),
        ],

        //
        // SouthEast.
        //

        // Inside bend of corner.
        (
            Some(Tile::SouthEast),
            Direction::North,
            Some(Direction::East),
        ) => vec![
            (Direction::East, Some(Direction::South)),
        ],
        (
            Some(Tile::SouthEast),
            Direction::West,
            Some(Direction::South),
        ) => vec![
            (Direction::South, Some(Direction::East)),
        ],
        // Outside bend of corner.
        (
            Some(Tile::SouthEast),
            Direction::North,
            _,
        ) => vec![
            (Direction::East, Some(Direction::North)),
            (Direction::North, None),
            (Direction::West, None),
        ],
        (
            Some(Tile::SouthEast),
            Direction::West,
            _,
        ) => vec![
            (Direction::South, Some(Direction::West)),
            (Direction::West, None),
            (Direction::North, None),
        ],
        // Approaching from the side.
        (
            Some(Tile::SouthEast),
            Direction::South,
            _,
        ) => vec![
            (Direction::East, Some(Direction::North)),
            (Direction::West, None),
            (Direction::South, Some(Direction::West)),
        ],
        (
            Some(Tile::SouthEast),
            Direction::East,
            _,
        ) => vec![
            (Direction::South, Some(Direction::West)),
            (Direction::North, None),
            (Direction::East, Some(Direction::North)),
        ],

        //
        // SouthWest.
        //

        // Inside bend of corner.
        (
            Some(Tile::SouthWest),
            Direction::North,
            Some(Direction::West),
        ) => vec![
            (Direction::West, Some(Direction::South)),
        ],
        (
            Some(Tile::SouthWest),
            Direction::East,
            Some(Direction::South),
        ) => vec![
            (Direction::South, Some(Direction::West)),
        ],
        // Outside bend of corner.
        (
            Some(Tile::SouthWest),
            Direction::North,
            _,
        ) => vec![
            (Direction::West, Some(Direction::North)),
            (Direction::North, None),
            (Direction::East, None),
        ],
        (
            Some(Tile::SouthWest),
            Direction::East,
            _,
        ) => vec![
            (Direction::South, Some(Direction::East)),
            (Direction::East, None),
            (Direction::North, None),
        ],
        // Approaching from the side.
        (
            Some(Tile::SouthWest),
            Direction::South,
            _,
        ) => vec![
            (Direction::West, Some(Direction::North)),
            (Direction::East, None),
            (Direction::South, Some(Direction::East)),
        ],
        (
            Some(Tile::SouthWest),
            Direction::West,
            _,
        ) => vec![
            (Direction::South, Some(Direction::East)),
            (Direction::North, None),
            (Direction::West, Some(Direction::North)),
        ],

        //
        // NorthWest.
        //

        // Inside bend of corner.
        (
            Some(Tile::NorthWest),
            Direction::South,
            Some(Direction::West),
        ) => vec![
            (Direction::West, Some(Direction::North)),
        ],
        (
            Some(Tile::NorthWest),
            Direction::East,
            Some(Direction::North),
        ) => vec![
            (Direction::North, Some(Direction::West)),
        ],
        // Outside bend of corner.
        (
            Some(Tile::NorthWest),
            Direction::South,
            _,
        ) => vec![
            (Direction::West, Some(Direction::South)),
            (Direction::South, None),
            (Direction::East, None),
        ],
        (
            Some(Tile::NorthWest),
            Direction::East,
            _,
        ) => vec![
            (Direction::North, Some(Direction::East)),
            (Direction::East, None),
            (Direction::South, None),
        ],
        // Approaching from the side.
        (
            Some(Tile::NorthWest),
            Direction::North,
            _,
        ) => vec![
            (Direction::West, Some(Direction::South)),
            (Direction::East, None),
            (Direction::North, Some(Direction::East)),
        ],
        (
            Some(Tile::NorthWest),
            Direction::West,
            _,
        ) => vec![
            (Direction::North, Some(Direction::East)),
            (Direction::South, None),
            (Direction::West, Some(Direction::South)),
        ],

        //
        // Non-loop tiles.
        //

        _ => vec![
            (Direction::North, None),
            (Direction::East, None),
            (Direction::South, None),
            (Direction::West, None),
        ],
    };

    visited.push(point);
    for (direction, squeeze) in options {
        let next = direction + point;
        if next.x >= bounds.0
            || next.y >= bounds.1
            || reachable.contains(&next)
            || visited.contains(&next)
        {
            continue;
        }
        collect_reachable(map, bounds, visited, reachable, next, direction, squeeze);
    }
    visited.pop();
}

pub fn part1(input: &str) -> usize {
    let mut map = parse_input(input);
    let start = extract_start(&mut map);
    let mainloop = find_loop(&map, start);
    mainloop.len() / 2
}

pub fn part2(input: &str) -> usize {
    let mut map = parse_input(input);
    let start = extract_start(&mut map);
    let mainloop: HashMap<_, _> = find_loop(&map, start)
        .into_iter()
        .map(|point| (point, &map[point.y][point.x]))
        .collect();

    let mut reachable: HashSet<Point> = HashSet::new();
    let mut visited = Vec::new();
    let bounds = (map[0].len(), map.len());
    for x in 0..bounds.0 {
        collect_reachable(
            &mainloop,
            &bounds,
            &mut visited,
            &mut reachable,
            Point::new(x, 0),
            Direction::East,
            None,
        );
        collect_reachable(
            &mainloop,
            &bounds,
            &mut visited,
            &mut reachable,
            Point::new(x, bounds.1),
            Direction::West,
            None,
        );
    }
    for y in 0..bounds.1 {
        collect_reachable(
            &mainloop,
            &bounds,
            &mut visited,
            &mut reachable,
            Point::new(0, y),
            Direction::South,
            None,
        );
        collect_reachable(
            &mainloop,
            &bounds,
            &mut visited,
            &mut reachable,
            Point::new(bounds.0, y),
            Direction::North,
            None,
        );
    }

    let mut count = 0;
    for x in 0..bounds.0 {
        for y in 0..bounds.1 {
            let point = Point::new(x, y);
            if !mainloop.contains_key(&point) && !reachable.contains(&point) {
                count += 1;
            }
        }
    }
    count
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 4, test)]
    static EXAMPLE_INPUT_1: &str = "
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    ";

    #[example_input(part1 = 8, test)]
    static EXAMPLE_INPUT_2: &str = "
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    ";

    #[example_input(part2 = 4, test)]
    static EXAMPLE_INPUT_3: &str = "
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
    ";

    #[example_input(part2 = 8, test)]
    static EXAMPLE_INPUT_4: &str = "
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
    ";

    #[example_input(part2 = 10, test)]
    static EXAMPLE_INPUT_5: &str = "
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    ";

    #[test]
    fn example_parse_1() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = vec![
            vec![Tile::None, Tile::None, Tile::None, Tile::None, Tile::None],
            vec![
                Tile::None,
                Tile::Start,
                Tile::Horizontal,
                Tile::SouthWest,
                Tile::None,
            ],
            vec![
                Tile::None,
                Tile::Vertical,
                Tile::None,
                Tile::Vertical,
                Tile::None,
            ],
            vec![
                Tile::None,
                Tile::NorthEast,
                Tile::Horizontal,
                Tile::NorthWest,
                Tile::None,
            ],
            vec![Tile::None, Tile::None, Tile::None, Tile::None, Tile::None],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_2() {
        let actual = parse_input(&EXAMPLE_INPUT_2);
        let expected = vec![
            vec![
                Tile::None,
                Tile::None,
                Tile::SouthEast,
                Tile::SouthWest,
                Tile::None,
            ],
            vec![
                Tile::None,
                Tile::SouthEast,
                Tile::NorthWest,
                Tile::Vertical,
                Tile::None,
            ],
            vec![
                Tile::Start,
                Tile::NorthWest,
                Tile::None,
                Tile::NorthEast,
                Tile::SouthWest,
            ],
            vec![
                Tile::Vertical,
                Tile::SouthEast,
                Tile::Horizontal,
                Tile::Horizontal,
                Tile::NorthWest,
            ],
            vec![
                Tile::NorthEast,
                Tile::NorthWest,
                Tile::None,
                Tile::None,
                Tile::None,
            ],
        ];
        assert_eq!(actual, expected);
    }
}
