use std::collections::HashSet;

use aoc::{grid::Point as BasePoint, runner::run};

type Point = BasePoint<isize>;

fn parse_input(input: &str) -> HashSet<Point> {
    return input
        .trim()
        .split('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim()
                .char_indices()
                .filter(|(_, c)| c == &'#')
                .map(|(x, _)| Point::new(x as isize, y as isize))
                .collect::<Vec<Point>>()
        })
        .collect();
}

fn neighbours(point: &Point) -> [Point; 8] {
    [
        Point::new(point.x - 1, point.y - 1),
        Point::new(point.x, point.y - 1),
        Point::new(point.x + 1, point.y - 1),
        Point::new(point.x + 1, point.y),
        Point::new(point.x + 1, point.y + 1),
        Point::new(point.x, point.y + 1),
        Point::new(point.x - 1, point.y + 1),
        Point::new(point.x - 1, point.y),
    ]
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn get_points(&self, point: &Point) -> [Point; 3] {
        match self {
            Direction::North => [
                Point::new(point.x - 1, point.y - 1),
                Point::new(point.x, point.y - 1),
                Point::new(point.x + 1, point.y - 1),
            ],
            Direction::East => [
                Point::new(point.x + 1, point.y - 1),
                Point::new(point.x + 1, point.y),
                Point::new(point.x + 1, point.y + 1),
            ],
            Direction::South => [
                Point::new(point.x - 1, point.y + 1),
                Point::new(point.x, point.y + 1),
                Point::new(point.x + 1, point.y + 1),
            ],
            Direction::West => [
                Point::new(point.x - 1, point.y - 1),
                Point::new(point.x - 1, point.y),
                Point::new(point.x - 1, point.y + 1),
            ],
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

#[derive(Debug, Eq, PartialEq)]
struct State {
    elves: HashSet<Point>,
    directions: Vec<Direction>,
}

fn cycle(state: &mut State) {
    let mut once = HashSet::new();
    let mut twice = HashSet::new();
    state.elves = state
        .elves
        .iter()
        .map(|start| {
            let mut has_neighbours = false;
            for point in neighbours(start) {
                if state.elves.contains(&point) {
                    has_neighbours = true;
                    break;
                }
            }
            if !has_neighbours {
                return (start, Option::None);
            }

            'direction: for direction in &state.directions {
                let points = direction.get_points(start);
                for point in points {
                    if state.elves.contains(&point) {
                        continue 'direction;
                    }
                }
                let target = points[1];
                if !once.insert(target) {
                    twice.insert(target);
                }
                return (start, Option::Some(target));
            }

            (start, Option::None)
        })
        .collect::<Vec<(&Point, Option<Point>)>>()
        .into_iter()
        .map(|(start, target)| match target {
            Option::Some(target) => {
                if twice.contains(&target) {
                    *start
                } else {
                    target
                }
            }
            Option::None => *start,
        })
        .collect();

    let direction = state.directions.remove(0);
    state.directions.push(direction);
}

pub fn part1(input: &str) -> usize {
    let elves = parse_input(input);
    let mut state = State {
        elves,
        directions: Vec::from(DIRECTIONS),
    };
    for _ in 0..10 {
        cycle(&mut state);
    }

    let x_min = state.elves.iter().map(|point| point.x).min().unwrap();
    let x_max = state.elves.iter().map(|point| point.x).max().unwrap();
    let y_min = state.elves.iter().map(|point| point.y).min().unwrap();
    let y_max = state.elves.iter().map(|point| point.y).max().unwrap();

    ((x_max - x_min + 1) * (y_max - y_min + 1)) as usize - state.elves.len()
}

pub fn part2(input: &str) -> usize {
    let elves = parse_input(input);
    let mut prev = elves.clone();
    let mut state = State {
        elves,
        directions: Vec::from(DIRECTIONS),
    };
    let mut round = 0;
    loop {
        round += 1;
        cycle(&mut state);
        if state.elves == prev {
            return round;
        }

        prev = state.elves.clone();
    }
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use common_macros::hash_set;
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT_SMALL: &str = "
        .....
        ..##.
        ..#..
        .....
        ..##.
        .....
    ";

    const EXAMPLE_INPUT: &str = "
        ....#..
        ..###.#
        #...#.#
        .#...##
        #.###..
        ##.#.##
        .#..#..
    ";

    #[test]
    fn example_parse_small() {
        let actual = parse_input(EXAMPLE_INPUT_SMALL);
        let expected = hash_set![
            Point::new(2, 1),
            Point::new(3, 1),
            Point::new(2, 2),
            Point::new(2, 4),
            Point::new(3, 4),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT);
        let expected = hash_set![
            Point::new(4, 0),
            Point::new(2, 1),
            Point::new(3, 1),
            Point::new(4, 1),
            Point::new(6, 1),
            Point::new(0, 2),
            Point::new(4, 2),
            Point::new(6, 2),
            Point::new(1, 3),
            Point::new(5, 3),
            Point::new(6, 3),
            Point::new(0, 4),
            Point::new(2, 4),
            Point::new(3, 4),
            Point::new(4, 4),
            Point::new(0, 5),
            Point::new(1, 5),
            Point::new(3, 5),
            Point::new(5, 5),
            Point::new(6, 5),
            Point::new(1, 6),
            Point::new(4, 6),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_cycle_small() {
        let mut state = State {
            elves: hash_set![
                Point::new(2, 1),
                Point::new(3, 1),
                Point::new(2, 2),
                Point::new(2, 4),
                Point::new(3, 4),
            ],
            directions: Vec::from(DIRECTIONS),
        };

        cycle(&mut state);
        assert_eq!(
            state,
            State {
                elves: hash_set![
                    Point::new(2, 0),
                    Point::new(3, 0),
                    Point::new(2, 2),
                    Point::new(2, 4),
                    Point::new(3, 3),
                ],
                directions: vec![
                    Direction::South,
                    Direction::West,
                    Direction::East,
                    Direction::North,
                ],
            }
        );

        cycle(&mut state);
        assert_eq!(
            state,
            State {
                elves: hash_set![
                    Point::new(2, 1),
                    Point::new(3, 1),
                    Point::new(1, 2),
                    Point::new(2, 5),
                    Point::new(4, 3),
                ],
                directions: vec![
                    Direction::West,
                    Direction::East,
                    Direction::North,
                    Direction::South,
                ],
            }
        );

        cycle(&mut state);
        assert_eq!(
            state,
            State {
                elves: hash_set![
                    Point::new(2, 0),
                    Point::new(4, 1),
                    Point::new(0, 2),
                    Point::new(2, 5),
                    Point::new(4, 3),
                ],
                directions: vec![
                    Direction::East,
                    Direction::North,
                    Direction::South,
                    Direction::West,
                ],
            }
        );

        cycle(&mut state);
        assert_eq!(
            state,
            State {
                elves: hash_set![
                    Point::new(2, 0),
                    Point::new(4, 1),
                    Point::new(0, 2),
                    Point::new(2, 5),
                    Point::new(4, 3),
                ],
                directions: vec![
                    Direction::North,
                    Direction::South,
                    Direction::West,
                    Direction::East,
                ],
            }
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn example_cycle() {
        let mut state = State {
            elves: hash_set![
                Point::new(7, 2),
                Point::new(5, 3),
                Point::new(6, 3),
                Point::new(7, 3),
                Point::new(9, 3),
                Point::new(3, 4),
                Point::new(7, 4),
                Point::new(9, 4),
                Point::new(4, 5),
                Point::new(8, 5),
                Point::new(9, 5),
                Point::new(3, 6),
                Point::new(5, 6),
                Point::new(6, 6),
                Point::new(7, 6),
                Point::new(3, 7),
                Point::new(4, 7),
                Point::new(6, 7),
                Point::new(8, 7),
                Point::new(9, 7),
                Point::new(4, 8),
                Point::new(7, 8),
            ],
            directions: Vec::from(DIRECTIONS),
        };

        // 1
        cycle(&mut state);
        assert_eq!(
            state,
            State {
                elves: hash_set![
                    Point::new(7, 1),
                    Point::new(5, 2),
                    Point::new(9, 2),
                    Point::new(3, 3),
                    Point::new(6, 3),
                    Point::new(8, 3),
                    Point::new(7, 4),
                    Point::new(10, 4),
                    Point::new(4, 5),
                    Point::new(6, 5),
                    Point::new(8, 5),
                    Point::new(9, 5),
                    Point::new(2, 6),
                    Point::new(5, 6),
                    Point::new(7, 6),
                    Point::new(2, 7),
                    Point::new(4, 7),
                    Point::new(6, 7),
                    Point::new(8, 7),
                    Point::new(9, 7),
                    Point::new(4, 9),
                    Point::new(7, 9)
                ],
                directions: vec![
                    Direction::South,
                    Direction::West,
                    Direction::East,
                    Direction::North,
                ],
            }
        );

        // 2
        cycle(&mut state);
        assert_eq!(
            state,
            State {
                elves: hash_set![
                    Point::new(7, 1),
                    Point::new(4, 2),
                    Point::new(10, 2),
                    Point::new(3, 3),
                    Point::new(6, 3),
                    Point::new(8, 3),
                    Point::new(7, 4),
                    Point::new(11, 4),
                    Point::new(3, 5),
                    Point::new(6, 5),
                    Point::new(8, 5),
                    Point::new(1, 6),
                    Point::new(5, 6),
                    Point::new(7, 6),
                    Point::new(9, 6),
                    Point::new(2, 8),
                    Point::new(4, 8),
                    Point::new(6, 8),
                    Point::new(8, 8),
                    Point::new(9, 8),
                    Point::new(4, 9),
                    Point::new(7, 9)
                ],
                directions: vec![
                    Direction::West,
                    Direction::East,
                    Direction::North,
                    Direction::South,
                ],
            }
        );

        // 3
        cycle(&mut state);
        assert_eq!(
            state,
            State {
                elves: hash_set![
                    Point::new(7, 1),
                    Point::new(5, 2),
                    Point::new(10, 2),
                    Point::new(2, 3),
                    Point::new(5, 3),
                    Point::new(9, 3),
                    Point::new(7, 4),
                    Point::new(11, 4),
                    Point::new(3, 5),
                    Point::new(6, 5),
                    Point::new(8, 5),
                    Point::new(1, 6),
                    Point::new(4, 6),
                    Point::new(10, 6),
                    Point::new(7, 7),
                    Point::new(8, 7),
                    Point::new(2, 8),
                    Point::new(3, 8),
                    Point::new(5, 8),
                    Point::new(10, 8),
                    Point::new(3, 9),
                    Point::new(7, 10)
                ],
                directions: vec![
                    Direction::East,
                    Direction::North,
                    Direction::South,
                    Direction::West,
                ],
            }
        );

        // 4
        cycle(&mut state);
        assert_eq!(
            state,
            State {
                elves: hash_set![
                    Point::new(7, 1),
                    Point::new(6, 2),
                    Point::new(11, 2),
                    Point::new(2, 3),
                    Point::new(6, 3),
                    Point::new(7, 3),
                    Point::new(3, 4),
                    Point::new(9, 4),
                    Point::new(11, 4),
                    Point::new(9, 5),
                    Point::new(1, 6),
                    Point::new(5, 6),
                    Point::new(6, 6),
                    Point::new(7, 6),
                    Point::new(10, 6),
                    Point::new(2, 7),
                    Point::new(9, 7),
                    Point::new(4, 8),
                    Point::new(5, 8),
                    Point::new(10, 8),
                    Point::new(4, 9),
                    Point::new(7, 10)
                ],
                directions: vec![
                    Direction::North,
                    Direction::South,
                    Direction::West,
                    Direction::East,
                ],
            }
        );

        // 5
        cycle(&mut state);
        assert_eq!(
            state,
            State {
                elves: hash_set![
                    Point::new(7, 0),
                    Point::new(2, 2),
                    Point::new(5, 2),
                    Point::new(11, 2),
                    Point::new(9, 3),
                    Point::new(6, 4),
                    Point::new(7, 4),
                    Point::new(11, 4),
                    Point::new(1, 5),
                    Point::new(3, 5),
                    Point::new(5, 5),
                    Point::new(6, 5),
                    Point::new(7, 5),
                    Point::new(8, 5),
                    Point::new(11, 6),
                    Point::new(4, 7),
                    Point::new(5, 7),
                    Point::new(8, 7),
                    Point::new(2, 8),
                    Point::new(10, 9),
                    Point::new(4, 10),
                    Point::new(7, 10)
                ],
                directions: vec![
                    Direction::South,
                    Direction::West,
                    Direction::East,
                    Direction::North,
                ],
            }
        );

        // 10
        for _ in 0..5 {
            cycle(&mut state);
        }
        assert_eq!(
            state,
            State {
                elves: hash_set![
                    Point::new(7, 0),
                    Point::new(11, 1),
                    Point::new(2, 2),
                    Point::new(4, 2),
                    Point::new(7, 2),
                    Point::new(6, 3),
                    Point::new(3, 4),
                    Point::new(9, 4),
                    Point::new(12, 4),
                    Point::new(1, 5),
                    Point::new(8, 5),
                    Point::new(9, 5),
                    Point::new(5, 6),
                    Point::new(6, 6),
                    Point::new(2, 7),
                    Point::new(11, 7),
                    Point::new(4, 8),
                    Point::new(6, 8),
                    Point::new(9, 8),
                    Point::new(4, 10),
                    Point::new(7, 10),
                    Point::new(10, 10)
                ],
                directions: vec![
                    Direction::West,
                    Direction::East,
                    Direction::North,
                    Direction::South,
                ],
            }
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 110);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 20);
    }
}
