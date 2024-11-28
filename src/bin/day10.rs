use std::collections::HashSet;

use aoc::utils::{
    parse,
    point::{Direction2, Point2},
};

type Point = Point2<usize>;
type Direction = Direction2;

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

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    parse!(input => {
        [lines split on '\n' with [chars as Tile]]
    } => lines)
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

    map[start.y][start.x] =
        if connections.contains(&start.wrapping_add_direction(Direction::North, &1)) {
            if connections.contains(&start.wrapping_add_direction(Direction::West, &1)) {
                Tile::NorthWest
            } else if connections.contains(&start.wrapping_add_direction(Direction::East, &1)) {
                Tile::NorthEast
            } else {
                Tile::Vertical
            }
        } else if connections.contains(&start.wrapping_add_direction(Direction::South, &1)) {
            if connections.contains(&start.wrapping_add_direction(Direction::West, &1)) {
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
        let next = point + direction;
        prev = point;
        curr = (next, &map[next.y][next.x]);

        mainloop.push(next);
        if next == start {
            break;
        }
    }
    mainloop
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
    let mainloop: HashSet<_> = find_loop(&map, start).into_iter().collect();

    map.into_iter()
        .enumerate()
        .map(|(y, row)| {
            let mut count = 0;
            let mut inside = false;
            for (x, tile) in row.into_iter().enumerate() {
                match tile {
                    _ if !mainloop.contains(&Point::new(x, y)) => {
                        if inside {
                            count += 1;
                        }
                    }
                    Tile::Vertical | Tile::NorthEast | Tile::NorthWest => {
                        inside = !inside;
                    }
                    _ => {}
                }
            }
            count
        })
        .sum()
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
