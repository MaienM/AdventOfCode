use std::collections::{BinaryHeap, HashSet};

use aoc::utils::{parse, point::Point2};

fn parse_input(input: &str) -> Vec<Point2> {
    parse!(input => {
        [coordinates split on '\n' with
            { [x as usize] ',' [y as usize] }
            => Point2::new(x, y)
        ]
    } => coordinates)
}

fn run(walls: &[Point2], end: Point2) -> usize {
    let mut paths = BinaryHeap::new();
    let mut done = HashSet::new();
    paths.push((0, Point2::new(0, 0)));
    loop {
        let (score, point) = paths.pop().unwrap();

        if point == end {
            return -score as usize;
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

pub fn part1(input: &str) -> usize {
    let coordinates = parse_input(input);
    run(&coordinates[..1024], Point2::new(70, 70))
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 22, notest)]
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
        let coordinates = parse_input(&EXAMPLE_INPUT);
        let actual = run(&coordinates[..12], Point2::new(6, 6)).to_string();
        assert_eq!(actual, EXAMPLE_INPUT.part1.unwrap());
    }
}
