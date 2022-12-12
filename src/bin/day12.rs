use aoc::grid::Grid as BaseGrid;
use aoc::grid::Point;
use aoc::runner::*;
use derive_new::new;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

type Grid = BaseGrid<u8>;

fn parse_input(input: String) -> (Grid, Point, Point) {
    let mut start = Option::None;
    let mut end = Option::None;
    let grid: Vec<Vec<u8>> = input
        .trim()
        .split("\n")
        .map(str::trim)
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => 100,
                    'E' => 101,
                    c => c as u8 - 'a' as u8,
                })
                .collect()
        })
        .collect();
    let mut grid = Grid::from(grid);
    for (point, value) in grid.mut_by_cell() {
        if value == &100 {
            *value = 0;
            start = Option::Some(point);
        } else if value == &101 {
            *value = 25;
            end = Option::Some(point);
        }
    }
    return (grid, start.unwrap(), end.unwrap());
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

pub fn part1(input: String) -> u16 {
    let (grid, start, end) = parse_input(input);
    let mut visited: HashSet<Point> = HashSet::new();
    let mut paths: BinaryHeap<PartialPath> = BinaryHeap::new();
    paths.push(PartialPath::new(0, 0, start));
    loop {
        let current = paths.pop().unwrap();
        for point in grid.neighbours(current.point, false) {
            if visited.contains(&point) {
                continue;
            }

            let height = *grid.getp(point).unwrap();
            if height <= current.height + 1 {
                if point == end {
                    return current.steps + 1;
                }

                visited.insert(point);
                paths.push(PartialPath::new(current.steps + 1, height, point));
            }
        }
    }
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
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

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 31);
    }
}
