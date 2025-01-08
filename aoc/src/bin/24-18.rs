aoc::setup!(title = "RAM Run");

use std::collections::{BinaryHeap, HashSet};

use aoc::point::Point2;

fn parse_input(input: &str) -> Vec<Point2> {
    parse!(input => {
        [coordinates split on '\n' with
            { [x as usize] ',' [y as usize] }
            => Point2::new(x, y)
        ]
    } => coordinates)
}

fn run(walls: &[Point2], end: Point2) -> Option<usize> {
    let mut paths = BinaryHeap::new();
    let mut done = HashSet::new();
    paths.push((0, Point2::new(0, 0)));
    loop {
        let (score, point) = paths.pop()?;

        if point == end {
            return Some(-score as usize);
        }

        if walls.contains(&point) || point.x > end.x || point.y > end.y {
            continue;
        }
        if !done.insert(point) {
            continue;
        }

        paths.extend(point.neighbours_ortho().into_iter().map(|p| (score - 1, p)));
    }
}

fn part1impl(input: &str, count: usize, end: Point2) -> usize {
    let coordinates = parse_input(input);
    run(&coordinates[..count], end).unwrap()
}

pub fn part1(input: &str) -> usize {
    part1impl(input, 1024, Point2::new(70, 70))
}

fn part2impl(input: &str, end: Point2) -> String {
    let coordinates = parse_input(input);
    let idx = (0..coordinates.len()).partition_point(|v| run(&coordinates[..v], end).is_none());
    let point = coordinates[idx.unwrap() - 1];
    format!("{},{}", point.x, point.y)
}

pub fn part2(input: &str) -> String {
    part2impl(input, Point2::new(70, 70))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 22, part2 = "6,1", notest)]
    static EXAMPLE_INPUT: &str = "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    ";

    #[test]
    fn example_test_1() {
        let actual = part1impl(&EXAMPLE_INPUT, 12, Point2::new(6, 6)).to_string();
        assert_eq!(actual, EXAMPLE_INPUT.part1.unwrap());
    }

    #[test]
    fn example_test_2() {
        let actual = part2impl(&EXAMPLE_INPUT, Point2::new(6, 6));
        assert_eq!(actual, EXAMPLE_INPUT.part2.unwrap());
    }
}
