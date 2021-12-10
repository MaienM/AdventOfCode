use aoc::*;
use std::{collections::HashSet, ops::Range};

#[macro_use]
extern crate derive_new;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, new)]
struct Point {
    x: i32,
    y: i32,
}

type LineDef = (Point, Point);

fn parse_point(input: &str) -> Point {
    let parts: [i32; 2] = input
        .trim()
        .splitn(2, ",")
        .map(|p| p.trim().parse().unwrap())
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap();
    return Point::new(parts[0], parts[1]);
}

fn parse_input(input: String) -> Vec<LineDef> {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            let points: [Point; 2] = line
                .splitn(2, "->")
                .map(parse_point)
                .collect::<Vec<Point>>()
                .try_into()
                .unwrap();
            return (points[0], points[1]);
        })
        .collect();
}

fn range(a: i32, b: i32) -> Range<i32> {
    if a < b {
        return a..b + 1;
    } else {
        return b..a + 1;
    }
}

fn get_points(linedef: LineDef) -> Vec<Point> {
    if linedef.0.x == linedef.1.x {
        return range(linedef.0.y, linedef.1.y)
            .map(|y| Point::new(linedef.0.x, y))
            .collect();
    } else if linedef.0.y == linedef.1.y {
        return range(linedef.0.x, linedef.1.x)
            .map(|x| Point::new(x, linedef.0.y))
            .collect();
    } else {
        panic!("Cannot handle diagonal lines ({:?})", linedef);
    }
}

fn part1(input: String) -> i32 {
    let linedefs = parse_input(input);
    let mut once: HashSet<Point> = HashSet::new();
    let mut more: HashSet<Point> = HashSet::new();
    let mut count = 0;
    for linedef in linedefs {
        if linedef.0.x == linedef.1.x || linedef.0.y == linedef.1.y {
            for point in get_points(linedef) {
                if more.contains(&point) {
                } else if once.contains(&point) {
                    once.remove(&point);
                    more.insert(point);
                    count += 1;
                } else {
                    once.insert(point);
                }
            }
        }
    }
    return count;
}

fn main() {
    run(part1, missing);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![
            (Point::new(0, 9), Point::new(5, 9)),
            (Point::new(8, 0), Point::new(0, 8)),
            (Point::new(9, 4), Point::new(3, 4)),
            (Point::new(2, 2), Point::new(2, 1)),
            (Point::new(7, 0), Point::new(7, 4)),
            (Point::new(6, 4), Point::new(2, 0)),
            (Point::new(0, 9), Point::new(2, 9)),
            (Point::new(3, 4), Point::new(1, 4)),
            (Point::new(0, 0), Point::new(8, 8)),
            (Point::new(5, 5), Point::new(8, 2)),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_get_points() {
        assert_eq!(
            get_points((Point::new(1, 1), Point::new(1, 3))),
            vec![Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)]
        );
        assert_eq!(
            get_points((Point::new(9, 7), Point::new(7, 7))),
            vec![Point::new(7, 7), Point::new(8, 7), Point::new(9, 7)]
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 5);
    }
}
