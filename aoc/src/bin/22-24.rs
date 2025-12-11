puzzle_runner::register_chapter!(book = "2022", title = "Blizzard Basin");

use std::{collections::HashSet, ops::Range};

use puzzle_lib::point::Point2;

type Point = Point2<u8>;

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
    x_range: Range<u8>,
    y_range: Range<u8>,
    blizards: Vec<Blizard>,
}
impl Map {
    fn next_generation(&mut self) -> Generation {
        let mut generation = Generation::new();
        generation.insert(self.start);
        generation.insert(self.end);
        for x in self.x_range.clone() {
            for y in self.y_range.clone() {
                generation.insert(Point::new(x, y));
            }
        }

        for Blizard(point, direction) in &mut self.blizards {
            match direction {
                Direction::North => {
                    if point.y == self.y_range.start {
                        point.y = self.y_range.end - 1;
                    } else {
                        point.y -= 1;
                    }
                }
                Direction::East => {
                    if point.x == self.x_range.end - 1 {
                        point.x = self.x_range.start;
                    } else {
                        point.x += 1;
                    }
                }
                Direction::South => {
                    if point.y == self.y_range.end - 1 {
                        point.y = self.y_range.start;
                    } else {
                        point.y += 1;
                    }
                }
                Direction::West => {
                    if point.x == self.x_range.start {
                        point.x = self.x_range.end - 1;
                    } else {
                        point.x -= 1;
                    }
                }
            }
            generation.remove(point);
        }

        generation
    }
}

fn parse_input(input: &str) -> Map {
    let lines: Vec<&str> = input.split('\n').collect();
    let x_range = 1..((lines[0].len() - 1) as u8);
    let y_range = 1..((lines.len() - 1) as u8);
    let start = Point::new(
        lines[0].char_indices().find(|(_, c)| c == &'.').unwrap().0 as u8,
        0,
    );
    let end = Point::new(
        lines[y_range.end as usize]
            .char_indices()
            .find(|(_, c)| c == &'.')
            .unwrap()
            .0 as u8,
        y_range.end,
    );
    let mut blizards = Vec::new();
    for (y, line) in lines.into_iter().enumerate().skip(1).take(y_range.len()) {
        for (x, chr) in line.char_indices().skip(1).take(x_range.len()) {
            match chr {
                '^' => blizards.push(Blizard(Point::new(x as u8, y as u8), Direction::North)),
                '>' => blizards.push(Blizard(Point::new(x as u8, y as u8), Direction::East)),
                'v' => blizards.push(Blizard(Point::new(x as u8, y as u8), Direction::South)),
                '<' => blizards.push(Blizard(Point::new(x as u8, y as u8), Direction::West)),
                '.' => {}
                _ => panic!("Invalid character {chr:?} in map at ({x}, {y})."),
            }
        }
    }
    Map {
        start,
        end,
        x_range,
        y_range,
        blizards,
    }
}

fn navigate(map: &mut Map, start: Point, end: Point) -> usize {
    let mut points = HashSet::new();
    points.insert(start);
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

            if point.x > map.x_range.start {
                let point = Point::new(point.x - 1, point.y);
                if generation.contains(&point) {
                    points.insert(point);
                }
            }

            if point.x < map.x_range.end - 1 {
                let point = Point::new(point.x + 1, point.y);
                if generation.contains(&point) {
                    points.insert(point);
                }
            }

            if point.y > map.y_range.start {
                let point = Point::new(point.x, point.y - 1);
                if generation.contains(&point) {
                    points.insert(point);
                }
            }

            if point.y < map.y_range.end - 1 {
                let point = Point::new(point.x, point.y + 1);
                if generation.contains(&point) {
                    points.insert(point);
                }
            }
        }
    }
}

#[register_part]
fn part1(input: &str) -> usize {
    let mut map = parse_input(input);
    let start = map.start;
    let end = map.end;
    navigate(&mut map, start, end)
}

#[register_part]
fn part2(input: &str) -> usize {
    let mut map = parse_input(input);
    let start = map.start;
    let end = map.end;
    navigate(&mut map, start, end) + navigate(&mut map, end, start) + navigate(&mut map, start, end)
}

#[cfg(test)]
mod tests {
    use common_macros::hash_set;
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 18, part2 = 54)]
    static EXAMPLE_INPUT: &str = "
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = Map {
            x_range: 1..7,
            y_range: 1..5,
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
    #[allow(clippy::too_many_lines)]
    fn example_move_blizard() {
        let mut map = Map {
            x_range: 1..7,
            y_range: 1..5,
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
}
