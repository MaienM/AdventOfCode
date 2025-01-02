use std::collections::HashSet;

use aoc::utils::{
    parse,
    point::{Direction2, Point2},
};
use derive_new::new;

type Point = Point2<isize>;

#[derive(new, Eq, PartialEq, Debug)]
struct Move {
    direction: Direction2,
    distance: usize,
}

fn parse_direction(input: &str) -> Direction2 {
    match input {
        "U" => Direction2::North,
        "D" => Direction2::South,
        "L" => Direction2::West,
        "R" => Direction2::East,
        _ => panic!(),
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    parse!(input => {
        [moves split on '\n' with
            { [direction with parse_direction] " " [distance as usize] }
            => Move { direction, distance }
        ]
    } => moves)
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

aoc_runner::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 13)]
    static EXAMPLE_INPUT_1: &str = "
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    ";

    #[example_input(part2 = 36)]
    static EXAMPLE_INPUT_2: &str = "
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
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = vec![
            Move::new(Direction2::East, 4),
            Move::new(Direction2::North, 4),
            Move::new(Direction2::West, 3),
            Move::new(Direction2::South, 1),
            Move::new(Direction2::East, 4),
            Move::new(Direction2::South, 1),
            Move::new(Direction2::West, 5),
            Move::new(Direction2::East, 2),
        ];
        assert_eq!(actual, expected);
    }
}
