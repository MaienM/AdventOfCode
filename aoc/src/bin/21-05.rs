puzzle_lib::setup!(title = "Hydrothermal Venture");
use std::{collections::HashSet, ops::RangeInclusive};

use puzzle_lib::point::Point2;

type Point = Point2<i16>;
type LineDef = (Point, Point);

fn parse_input(input: &str) -> Vec<LineDef> {
    parse!(input => {
        [linedefs split on '\n' with
            { [x1 as i16] ',' [y1 as i16] " -> " [x2 as i16] ',' [y2 as i16] }
            => (Point::new(x1, y1), Point::new(x2, y2))
        ]
    } => linedefs)
}

fn range(a: i16, b: i16) -> RangeInclusive<i16> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn get_points(linedef: LineDef) -> Vec<Point> {
    if linedef.0.x == linedef.1.x {
        range(linedef.0.y, linedef.1.y)
            .map(|y| Point::new(linedef.0.x, y))
            .collect()
    } else if linedef.0.y == linedef.1.y {
        range(linedef.0.x, linedef.1.x)
            .map(|x| Point::new(x, linedef.0.y))
            .collect()
    } else if (linedef.0.x - linedef.1.x).abs() == (linedef.0.y - linedef.1.y).abs() {
        let xmul = (linedef.1.x - linedef.0.x) / (linedef.1.x - linedef.0.x).abs();
        let ymul = (linedef.1.y - linedef.0.y) / (linedef.1.y - linedef.0.y).abs();
        range(0, (linedef.0.x - linedef.1.x).abs())
            .map(|i| Point::new(linedef.0.x + i * xmul, linedef.0.y + i * ymul))
            .collect()
    } else {
        panic!("Cannot handle diagonal lines at a non-45 degree angle ({linedef:?})",);
    }
}

fn count_overlapping_points(linedefs: Vec<LineDef>) -> i16 {
    let mut once: HashSet<Point> = HashSet::new();
    let mut more: HashSet<Point> = HashSet::new();
    let mut count = 0;
    for linedef in linedefs {
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
    count
}

pub fn part1(input: &str) -> i16 {
    let linedefs = parse_input(input)
        .into_iter()
        .filter(|linedef| linedef.0.x == linedef.1.x || linedef.0.y == linedef.1.y)
        .collect();
    count_overlapping_points(linedefs)
}

pub fn part2(input: &str) -> i16 {
    let linedefs = parse_input(input);
    count_overlapping_points(linedefs)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 5, part2 = 12)]
    static EXAMPLE_INPUT: &str = "
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
        let actual = parse_input(&EXAMPLE_INPUT);
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
        assert_eq!(
            get_points((Point::new(1, 1), Point::new(3, 3))),
            vec![Point::new(1, 1), Point::new(2, 2), Point::new(3, 3)]
        );
        assert_eq!(
            get_points((Point::new(9, 7), Point::new(7, 9))),
            vec![Point::new(9, 7), Point::new(8, 8), Point::new(7, 9)]
        );
    }
}
