puzzle_runner::register_chapter!(book = "2022", title = "Hill Climbing Algorithm");

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use derive_new::new;
use puzzle_lib::{grid::FullGrid, point::Point2};

type Grid = FullGrid<u8>;
type Point = Point2;

fn parse_input(input: &str) -> (Grid, Point, Point) {
    parse!(input => {
        [grid cells match {
            'S' => index into start => 0,
            'E' => index into end => 25,
            c => c as u8 - b'a',
        }]
    } => (grid, start, end))
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
        Some(self.cmp(other))
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
    paths.push(PartialPath::new(0, grid[start], start));
    loop {
        let current = paths.pop().unwrap();
        for point in current.point.neighbours_ortho() {
            if visited.contains(&point) {
                continue;
            }

            let Some(height) = grid.get(&point) else {
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

#[register_part]
fn part1(input: &str) -> u16 {
    let (grid, start, end) = parse_input(input);
    pathfind(
        &grid,
        start,
        |height, current| height <= current + 1,
        |point, _| point == end,
    )
}

#[register_part]
fn part2(input: &str) -> u16 {
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
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

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
            Grid::from([
                [0, 0, 1, 16, 15, 14, 13, 12],
                [0, 1, 2, 17, 24, 23, 23, 11],
                [0, 2, 2, 18, 25, 25, 23, 10],
                [0, 2, 2, 19, 20, 21, 22, 9],
                [0, 1, 3, 4, 5, 6, 7, 8],
            ]),
            Point::new(0, 0),
            Point::new(5, 2),
        );
        assert_eq!(actual, expected);
    }
}
