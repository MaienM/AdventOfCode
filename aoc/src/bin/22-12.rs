aoc::setup!(title = "Hill Climbing Algorithm");

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use aoc::point::Point2;
use derive_new::new;

type Grid = Vec<Vec<u8>>;
type Point = Point2;

fn parse_input(input: &str) -> (Grid, Point, Point) {
    let placeholder_start = 100;
    let placeholder_end = 101;

    parse!(input =>
        [grid split on '\n' with
            [chars with |c| match c {
                'S' => placeholder_start,
                'E' => placeholder_end,
                c => c as u8 - b'a',
            } ]
        ]
    );

    let mut start = Option::None;
    let mut end = Option::None;
    for (y, row) in grid.iter_mut().enumerate() {
        for (x, value) in row.iter_mut().enumerate() {
            if value == &placeholder_start {
                *value = 0;
                start = Option::Some(Point::new(x, y));
            } else if value == &placeholder_end {
                *value = 25;
                end = Option::Some(Point::new(x, y));
            }
        }
    }

    (grid, start.unwrap(), end.unwrap())
}

#[derive(Debug, Eq, PartialEq, new)]
struct PartialPath {
    steps: u16,
    height: u8,
    point: Point,
}
// Sorting comparisons are inverted since we always want the smallest item from the max-heap.
impl PartialOrd for PartialPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.steps.cmp(&self.steps))
    }
}
impl Ord for PartialPath {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

fn pathfind(
    grid: &Grid,
    start: Point,
    predicate_valid: fn(u8, u8) -> bool,
    predicate_done: impl Fn(Point, u8) -> bool,
) -> u16 {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut paths: BinaryHeap<PartialPath> = BinaryHeap::new();
    paths.push(PartialPath::new(0, grid[start.y][start.x], start));
    loop {
        let current = paths.pop().unwrap();
        for point in current.point.neighbours_ortho() {
            if visited.contains(&point) {
                continue;
            }

            let Some(height) = grid.get(point.y).and_then(|row| row.get(point.x)) else {
                continue;
            };
            if predicate_valid(*height, current.height) {
                if predicate_done(point, *height) {
                    return current.steps + 1;
                }

                visited.insert(point);
                paths.push(PartialPath::new(current.steps + 1, *height, point));
            }
        }
    }
}

pub fn part1(input: &str) -> u16 {
    let (grid, start, end) = parse_input(input);
    pathfind(
        &grid,
        start,
        |height, current| height <= current + 1,
        |point, _| point == end,
    )
}

pub fn part2(input: &str) -> u16 {
    let (grid, _, end) = parse_input(input);
    pathfind(
        &grid,
        end,
        |height, current| current <= height + 1,
        |_, height| height == 0,
    )
}

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 31, part2 = 29)]
    static EXAMPLE_INPUT: &str = "
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = (
            Grid::from(vec![
                vec![0, 0, 1, 16, 15, 14, 13, 12],
                vec![0, 1, 2, 17, 24, 23, 23, 11],
                vec![0, 2, 2, 18, 25, 25, 23, 10],
                vec![0, 2, 2, 19, 20, 21, 22, 9],
                vec![0, 1, 3, 4, 5, 6, 7, 8],
            ]),
            Point::new(0, 0),
            Point::new(5, 2),
        );
        assert_eq!(actual, expected);
    }
}
