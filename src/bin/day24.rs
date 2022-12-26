use std::collections::HashSet;
use std::ops::Range;

use aoc::grid::Point as BasePoint;
use aoc::runner::*;

type Point = BasePoint<u8>;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq)]
struct Blizard(Point, Direction);

type Generation = HashSet<Point>;

#[derive(Debug, Eq, PartialEq)]
struct Map {
    start: Point,
    end: Point,
    xrange: Range<u8>,
    yrange: Range<u8>,
    blizards: Vec<Blizard>,
}
impl Map {
    fn next_generation(&mut self) -> Generation {
        let mut generation = Generation::new();
        generation.insert(self.start);
        generation.insert(self.end);
        for x in self.xrange.clone() {
            for y in self.yrange.clone() {
                generation.insert(Point::new(x, y));
            }
        }

        for Blizard(point, direction) in self.blizards.iter_mut() {
            match direction {
                Direction::North => {
                    if point.y == self.yrange.start {
                        point.y = self.yrange.end - 1;
                    } else {
                        point.y -= 1;
                    }
                }
                Direction::East => {
                    if point.x == self.xrange.end - 1 {
                        point.x = self.xrange.start;
                    } else {
                        point.x += 1;
                    }
                }
                Direction::South => {
                    if point.y == self.yrange.end - 1 {
                        point.y = self.yrange.start;
                    } else {
                        point.y += 1;
                    }
                }
                Direction::West => {
                    if point.x == self.xrange.start {
                        point.x = self.xrange.end - 1;
                    } else {
                        point.x -= 1;
                    }
                }
            }
            generation.remove(point);
        }

        return generation;
    }
}

fn parse_input(input: String) -> Map {
    let lines: Vec<&str> = input.trim().split("\n").map(str::trim).collect();
    let xrange = 1..((lines[0].len() - 1) as u8);
    let yrange = 1..((lines.len() - 1) as u8);
    let start = Point::new(
        lines[0].char_indices().find(|(_, c)| c == &'.').unwrap().0 as u8,
        0,
    );
    let end = Point::new(
        lines[yrange.end as usize]
            .char_indices()
            .find(|(_, c)| c == &'.')
            .unwrap()
            .0 as u8,
        yrange.end as u8,
    );
    let mut blizards = Vec::new();
    for (y, line) in lines.into_iter().enumerate().skip(1).take(yrange.len()) {
        for (x, chr) in line.char_indices().skip(1).take(xrange.len()) {
            match chr {
                '^' => blizards.push(Blizard(Point::new(x as u8, y as u8), Direction::North)),
                '>' => blizards.push(Blizard(Point::new(x as u8, y as u8), Direction::East)),
                'v' => blizards.push(Blizard(Point::new(x as u8, y as u8), Direction::South)),
                '<' => blizards.push(Blizard(Point::new(x as u8, y as u8), Direction::West)),
                '.' => {}
                _ => panic!("Invalid character {:?} in map at ({}, {}).", chr, x, y),
            };
        }
    }
    return Map {
        xrange,
        yrange,
        start,
        end,
        blizards,
    };
}

fn navigate(map: &mut Map, start: &Point, end: &Point) -> usize {
    let mut points = HashSet::new();
    points.insert(start.clone());
    let mut i = 0;
    loop {
        i += 1;
        let generation = map.next_generation();

        let mut oldpoints = HashSet::new();
        std::mem::swap(&mut points, &mut oldpoints);
        for point in oldpoints {
            if point.x == end.x && (point.y as i8 - end.y as i8).abs() == 1 {
                return i;
            }

            if generation.contains(&point) {
                points.insert(point);
            }

            if point.x > map.xrange.start {
                let point = Point::new(point.x - 1, point.y);
                if generation.contains(&point) {
                    points.insert(point);
                }
            }

            if point.x < map.xrange.end - 1 {
                let point = Point::new(point.x + 1, point.y);
                if generation.contains(&point) {
                    points.insert(point);
                }
            }

            if point.y > map.yrange.start {
                let point = Point::new(point.x, point.y - 1);
                if generation.contains(&point) {
                    points.insert(point);
                }
            }

            if point.y < map.yrange.end - 1 {
                let point = Point::new(point.x, point.y + 1);
                if generation.contains(&point) {
                    points.insert(point);
                }
            }
        }
    }
}

pub fn part1(input: String) -> usize {
    let mut map = parse_input(input);
    let start = map.start.clone();
    let end = map.end.clone();
    return navigate(&mut map, &start, &end);
}

pub fn part2(input: String) -> usize {
    let mut map = parse_input(input);
    let start = map.start.clone();
    let end = map.end.clone();
    return navigate(&mut map, &start, &end)
        + navigate(&mut map, &end, &start)
        + navigate(&mut map, &start, &end);
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use common_macros::hash_set;
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = Map {
            xrange: 1..7,
            yrange: 1..5,
            start: Point::new(1, 0),
            end: Point::new(6, 5),
            blizards: vec![
                Blizard(Point::new(1, 1), Direction::East),
                Blizard(Point::new(2, 1), Direction::East),
                Blizard(Point::new(4, 1), Direction::West),
                Blizard(Point::new(5, 1), Direction::North),
                Blizard(Point::new(6, 1), Direction::West),
                Blizard(Point::new(2, 2), Direction::West),
                Blizard(Point::new(5, 2), Direction::West),
                Blizard(Point::new(6, 2), Direction::West),
                Blizard(Point::new(1, 3), Direction::East),
                Blizard(Point::new(2, 3), Direction::South),
                Blizard(Point::new(4, 3), Direction::East),
                Blizard(Point::new(5, 3), Direction::West),
                Blizard(Point::new(6, 3), Direction::East),
                Blizard(Point::new(1, 4), Direction::West),
                Blizard(Point::new(2, 4), Direction::North),
                Blizard(Point::new(3, 4), Direction::South),
                Blizard(Point::new(4, 4), Direction::North),
                Blizard(Point::new(5, 4), Direction::North),
                Blizard(Point::new(6, 4), Direction::East),
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_move_blizard() {
        let mut map = Map {
            xrange: 1..7,
            yrange: 1..5,
            start: Point::new(1, 0),
            end: Point::new(6, 5),
            blizards: vec![
                Blizard(Point::new(1, 1), Direction::East),
                Blizard(Point::new(2, 1), Direction::East),
                Blizard(Point::new(4, 1), Direction::West),
                Blizard(Point::new(5, 1), Direction::North),
                Blizard(Point::new(6, 1), Direction::West),
                Blizard(Point::new(2, 2), Direction::West),
                Blizard(Point::new(5, 2), Direction::West),
                Blizard(Point::new(6, 2), Direction::West),
                Blizard(Point::new(1, 3), Direction::East),
                Blizard(Point::new(2, 3), Direction::South),
                Blizard(Point::new(4, 3), Direction::East),
                Blizard(Point::new(5, 3), Direction::West),
                Blizard(Point::new(6, 3), Direction::East),
                Blizard(Point::new(1, 4), Direction::West),
                Blizard(Point::new(2, 4), Direction::North),
                Blizard(Point::new(3, 4), Direction::South),
                Blizard(Point::new(4, 4), Direction::North),
                Blizard(Point::new(5, 4), Direction::North),
                Blizard(Point::new(6, 4), Direction::East),
            ],
        };

        assert_eq!(
            map.next_generation(),
            hash_set![
                map.start,
                map.end,
                Point::new(1, 1),
                Point::new(4, 1),
                Point::new(6, 1),
                Point::new(2, 2),
                Point::new(3, 2),
                Point::new(6, 2),
                Point::new(3, 3),
                Point::new(6, 3),
                Point::new(3, 4),
                Point::new(4, 4),
            ],
        );
        assert_eq!(
            map.blizards,
            vec![
                Blizard(Point::new(2, 1), Direction::East),
                Blizard(Point::new(3, 1), Direction::East),
                Blizard(Point::new(3, 1), Direction::West),
                Blizard(Point::new(5, 4), Direction::North),
                Blizard(Point::new(5, 1), Direction::West),
                Blizard(Point::new(1, 2), Direction::West),
                Blizard(Point::new(4, 2), Direction::West),
                Blizard(Point::new(5, 2), Direction::West),
                Blizard(Point::new(2, 3), Direction::East),
                Blizard(Point::new(2, 4), Direction::South),
                Blizard(Point::new(5, 3), Direction::East),
                Blizard(Point::new(4, 3), Direction::West),
                Blizard(Point::new(1, 3), Direction::East),
                Blizard(Point::new(6, 4), Direction::West),
                Blizard(Point::new(2, 3), Direction::North),
                Blizard(Point::new(3, 1), Direction::South),
                Blizard(Point::new(4, 3), Direction::North),
                Blizard(Point::new(5, 3), Direction::North),
                Blizard(Point::new(1, 4), Direction::East),
            ]
        );

        assert_eq!(
            map.next_generation(),
            hash_set![
                map.start,
                map.end,
                Point::new(1, 1),
                Point::new(5, 1),
                Point::new(6, 1),
                Point::new(1, 2),
                Point::new(1, 3),
                Point::new(4, 3),
                Point::new(1, 4),
                Point::new(3, 4),
                Point::new(4, 4),
                Point::new(6, 4),
            ],
        );
        assert_eq!(
            map.blizards,
            vec![
                Blizard(Point::new(3, 1), Direction::East),
                Blizard(Point::new(4, 1), Direction::East),
                Blizard(Point::new(2, 1), Direction::West),
                Blizard(Point::new(5, 3), Direction::North),
                Blizard(Point::new(4, 1), Direction::West),
                Blizard(Point::new(6, 2), Direction::West),
                Blizard(Point::new(3, 2), Direction::West),
                Blizard(Point::new(4, 2), Direction::West),
                Blizard(Point::new(3, 3), Direction::East),
                Blizard(Point::new(2, 1), Direction::South),
                Blizard(Point::new(6, 3), Direction::East),
                Blizard(Point::new(3, 3), Direction::West),
                Blizard(Point::new(2, 3), Direction::East),
                Blizard(Point::new(5, 4), Direction::West),
                Blizard(Point::new(2, 2), Direction::North),
                Blizard(Point::new(3, 2), Direction::South),
                Blizard(Point::new(4, 2), Direction::North),
                Blizard(Point::new(5, 2), Direction::North),
                Blizard(Point::new(2, 4), Direction::East),
            ]
        );

        assert_eq!(
            map.next_generation(),
            hash_set![
                map.start,
                map.end,
                Point::new(6, 1),
                Point::new(1, 2),
                Point::new(4, 2),
                Point::new(6, 2),
                Point::new(5, 3),
                Point::new(6, 3),
                Point::new(1, 4),
                Point::new(2, 4),
                Point::new(5, 4),
                Point::new(6, 4),
            ],
        );
        assert_eq!(
            map.blizards,
            vec![
                Blizard(Point::new(4, 1), Direction::East),
                Blizard(Point::new(5, 1), Direction::East),
                Blizard(Point::new(1, 1), Direction::West),
                Blizard(Point::new(5, 2), Direction::North),
                Blizard(Point::new(3, 1), Direction::West),
                Blizard(Point::new(5, 2), Direction::West),
                Blizard(Point::new(2, 2), Direction::West),
                Blizard(Point::new(3, 2), Direction::West),
                Blizard(Point::new(4, 3), Direction::East),
                Blizard(Point::new(2, 2), Direction::South),
                Blizard(Point::new(1, 3), Direction::East),
                Blizard(Point::new(2, 3), Direction::West),
                Blizard(Point::new(3, 3), Direction::East),
                Blizard(Point::new(4, 4), Direction::West),
                Blizard(Point::new(2, 1), Direction::North),
                Blizard(Point::new(3, 3), Direction::South),
                Blizard(Point::new(4, 1), Direction::North),
                Blizard(Point::new(5, 1), Direction::North),
                Blizard(Point::new(3, 4), Direction::East),
            ]
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 18);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 54);
    }
}
