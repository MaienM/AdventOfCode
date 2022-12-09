use aoc::grid::Point as BasePoint;
use aoc::runner::*;
use derive_new::new;
use std::collections::HashSet;

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

fn parse_input(input: String) -> Vec<Move> {
    return input
        .trim()
        .split("\n")
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
            return Move {
                direction,
                distance,
            };
        })
        .collect();
}

pub fn part1(input: String) -> usize {
    let moves = parse_input(input);
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    let mut visited = HashSet::<Point>::new();
    visited.insert(tail);
    for mov in moves {
        for _ in 0..mov.distance {
            head += mov.direction;
            let delta = head - tail;
            if delta.x.abs() > 1 || delta.y.abs() > 1 {
                tail += Point::new(delta.x.clamp(-1, 1), delta.y.clamp(-1, 1));
                visited.insert(tail);
            }
        }
    }
    println!("{:#?}", visited);
    return visited.len();
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
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
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 13);
    }
}
