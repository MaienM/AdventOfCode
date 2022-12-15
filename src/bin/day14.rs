use aoc::grid::Point as BasePoint;
use aoc::runner::*;
use std::collections::HashSet;
use std::ops::RangeInclusive;

type Point = BasePoint<isize>;

const OFFSET: Point = Point { x: -500, y: 0 };
const DROP_POINT: Point = Point { x: 0, y: 0 };
const MOVES: [Point; 3] = [
    Point { x: 0, y: 1 },
    Point { x: -1, y: 1 },
    Point { x: 1, y: 1 },
];

type Points = HashSet<Point>;

fn collect_points_on_line(points: &mut Points, start: Point, end: Point) {
    if start.x > end.x || start.y > end.y {
        collect_points_on_line(points, end, start);
        return;
    }

    points.insert(start);
    let mut current = start;
    let delta = Point::new((end.x - start.x).min(1), (end.y - start.y).min(1));
    while current != end {
        current += delta;
        points.insert(current);
    }
}

fn parse_input(input: String) -> Points {
    let mut result = Points::new();
    for line in input.trim().split("\n") {
        let mut points = line.trim().split(" -> ").map(|part| {
            let mut parts = part.splitn(2, ",").map(str::parse).map(Result::unwrap);
            return Point::new(parts.next().unwrap(), parts.next().unwrap()) + OFFSET;
        });
        let mut start = points.next().unwrap();
        for end in points {
            collect_points_on_line(&mut result, start, end);
            start = end;
        }
    }
    return result;
}

fn sand_find_point(points: &Points, void_start: isize) -> Option<Point> {
    let mut current = DROP_POINT;
    'main: loop {
        if &current.y >= &void_start {
            return Option::None;
        }

        for move_ in MOVES {
            let new = current + move_;
            if !points.contains(&new) {
                current = new;
                continue 'main;
            }
        }

        return Option::Some(current);
    }
}

pub fn part1(input: String) -> usize {
    let mut points = parse_input(input);
    let void_start = points.iter().map(|p| p.y).max().unwrap();
    let size_start = points.len();
    loop {
        match sand_find_point(&points, void_start) {
            Option::Some(point) => {
                points.insert(point);
            }
            Option::None => {
                return points.len() - size_start;
            }
        }
    }
}

pub fn part2(input: String) -> usize {
    let mut points = parse_input(input);
    let floor = points.iter().map(|p| p.y).max().unwrap() + 2;
    for x in -(floor + 1)..(floor + 1) {
        points.insert(Point::new(x, floor));
    }
    let size_start = points.len();
    loop {
        match sand_find_point(&points, floor + 1) {
            Option::Some(DROP_POINT) => {
                return points.len() - size_start + 1;
            }
            Option::Some(point) => {
                points.insert(point);
            }
            Option::None => panic!("Hit void, but no void should exist."),
        }
    }
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = Points::from([
            Point::new(-2, 4),
            Point::new(-2, 5),
            Point::new(-2, 6),
            Point::new(-3, 6),
            Point::new(-4, 6),
            Point::new(3, 4),
            Point::new(2, 4),
            Point::new(2, 5),
            Point::new(2, 6),
            Point::new(2, 7),
            Point::new(2, 8),
            Point::new(2, 9),
            Point::new(1, 9),
            Point::new(0, 9),
            Point::new(-1, 9),
            Point::new(-2, 9),
            Point::new(-3, 9),
            Point::new(-4, 9),
            Point::new(-5, 9),
            Point::new(-6, 9),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 24);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 93);
    }
}
