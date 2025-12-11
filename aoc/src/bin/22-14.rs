puzzle_runner::register_chapter!(book = 2022, title = "Regolith Reservoir");

use std::collections::HashSet;

use puzzle_lib::point::Point2;

type Point = Point2<isize>;

const DROP_POINT: Point = Point { x: 500, y: 0 };
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

fn parse_input(input: &str) -> Points {
    let mut result = Points::new();
    parse!(input =>
        [
            _lines split on '\n' with
                { [points split on " -> " into iterator with
                    { [x as isize] ',' [y as isize] }
                    => Point::new(x, y)
                ] }
                => {
                    let mut start = points.next().unwrap();
                    for end in points {
                        collect_points_on_line(&mut result, start, end);
                        start = end;
                    }
                }
        ]
    );
    result
}

#[derive(Debug, Eq, PartialEq)]
enum Sand {
    FellIntoVoid,
    AtRest,
}

fn sand_fill(points: &mut Points, current: Point, void_start: isize) -> Sand {
    if current.y > void_start {
        return Sand::FellIntoVoid;
    } else if points.contains(&current) {
        return Sand::AtRest;
    }
    for move_ in MOVES {
        let next = current + move_;
        match sand_fill(points, next, void_start) {
            Sand::FellIntoVoid => return Sand::FellIntoVoid,
            Sand::AtRest => {
                points.insert(next);
            }
        }
    }
    Sand::AtRest
}

#[register_part]
fn part1(input: &str) -> usize {
    let mut points = parse_input(input);
    let void_start = points.iter().map(|p| p.y).max().unwrap();
    let size_start = points.len();
    assert_eq!(
        sand_fill(&mut points, DROP_POINT, void_start),
        Sand::FellIntoVoid
    );
    points.len() - size_start
}

#[register_part]
fn part2(input: &str) -> usize {
    let mut points = parse_input(input);
    let floor = points.iter().map(|p| p.y).max().unwrap() + 2;
    for x in -(floor + 1)..=floor {
        points.insert(Point::new(DROP_POINT.x + x, floor));
    }
    let size_start = points.len();
    assert_eq!(sand_fill(&mut points, DROP_POINT, floor + 1), Sand::AtRest);
    points.len() - size_start + 1
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 24, part2 = 93)]
    static EXAMPLE_INPUT: &str = "
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = Points::from([
            Point::new(498, 4),
            Point::new(498, 5),
            Point::new(498, 6),
            Point::new(497, 6),
            Point::new(496, 6),
            Point::new(503, 4),
            Point::new(502, 4),
            Point::new(502, 5),
            Point::new(502, 6),
            Point::new(502, 7),
            Point::new(502, 8),
            Point::new(502, 9),
            Point::new(501, 9),
            Point::new(500, 9),
            Point::new(499, 9),
            Point::new(498, 9),
            Point::new(497, 9),
            Point::new(496, 9),
            Point::new(495, 9),
            Point::new(494, 9),
        ]);
        assert_eq!(actual, expected);
    }
}
