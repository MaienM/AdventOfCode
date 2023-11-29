use std::collections::HashSet;

use aoc::{grid::Point as BasePoint, runner::run};
use derive_new::new;

type Point = BasePoint<isize>;

#[non_exhaustive]
struct Direction {}
impl Direction {
    pub const UP: Point = Point { x: 0, y: 1 };
    pub const DOWN: Point = Point { x: 0, y: -1 };
    pub const LEFT: Point = Point { x: -1, y: 0 };
    pub const RIGHT: Point = Point { x: 1, y: 0 };
}

#[derive(new, Eq, PartialEq, Debug)]
struct Move {
    direction: Point,
    distance: usize,
}

fn parse_input(input: &str) -> Vec<Move> {
    return input
        .trim()
        .split('\n')
        .map(str::trim)
        .map(|line| {
            let direction = match &line[0..1] {
                "U" => Direction::UP,
                "D" => Direction::DOWN,
                "L" => Direction::LEFT,
                "R" => Direction::RIGHT,
                _ => panic!(),
            };
            let distance = line[2..].parse().unwrap();
            Move {
                direction,
                distance,
            }
        })
        .collect();
}

fn follow(follower: &Point, leader: &Point) -> Point {
    let delta = *leader - *follower;
    if delta.x.abs() > 1 || delta.y.abs() > 1 {
        *follower + Point::new(delta.x.clamp(-1, 1), delta.y.clamp(-1, 1))
    } else {
        *follower
    }
}

pub fn part1(input: &str) -> usize {
    let moves = parse_input(input);
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    let mut visited = HashSet::<Point>::new();
    visited.insert(tail);
    for mov in moves {
        for _ in 0..mov.distance {
            head += mov.direction;
            tail = follow(&tail, &head);
            visited.insert(tail);
        }
    }
    visited.len()
}

pub fn part2(input: &str) -> usize {
    let moves = parse_input(input);
    let mut chain = [Point::new(0, 0); 10];
    let mut visited = HashSet::<Point>::new();
    visited.insert(chain[9]);
    for mov in moves {
        for _ in 0..mov.distance {
            chain[0] += mov.direction;
            for i in 1..=9 {
                chain[i] = follow(&chain[i], &chain[i - 1]);
            }
            visited.insert(chain[9]);
        }
    }
    visited.len()
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT_1: &str = "
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    ";
    const EXAMPLE_INPUT_2: &str = "
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT_1);
        let expected = vec![
            Move::new(Direction::RIGHT, 4),
            Move::new(Direction::UP, 4),
            Move::new(Direction::LEFT, 3),
            Move::new(Direction::DOWN, 1),
            Move::new(Direction::RIGHT, 4),
            Move::new(Direction::DOWN, 1),
            Move::new(Direction::LEFT, 5),
            Move::new(Direction::RIGHT, 2),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT_1), 13);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT_2), 36);
    }
}
