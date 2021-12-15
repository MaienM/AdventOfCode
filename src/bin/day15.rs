use std::collections::BinaryHeap;

use aoc::grid::{Grid as BaseGrid, Point};
use aoc::runner::*;

#[derive(Debug, Eq, PartialEq)]
struct Cell {
    pub point: Point,
    pub min_path_cost: i16,
}
impl Cell {
    pub fn new(point: Point, min_path_cost: i16) -> Self {
        return Self {
            point,
            min_path_cost,
        };
    }
}

// BinaryHeap is a max-heap, but we always want the smallest value, so we invert the ordering.
impl PartialOrd for Cell {
    fn lt(&self, other: &Self) -> bool {
        return other.min_path_cost.lt(&self.min_path_cost);
    }

    fn le(&self, other: &Self) -> bool {
        return other.min_path_cost.le(&self.min_path_cost);
    }

    fn gt(&self, other: &Self) -> bool {
        return other.min_path_cost.gt(&self.min_path_cost);
    }

    fn ge(&self, other: &Self) -> bool {
        return other.min_path_cost.ge(&self.min_path_cost);
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return other.min_path_cost.partial_cmp(&self.min_path_cost);
    }
}
impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.min_path_cost.cmp(&self.min_path_cost);
    }
}

type InputGrid = BaseGrid<i8>;
type ResultGrid = BaseGrid<i16>;

fn parse_input(input: String) -> InputGrid {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            return line
                .trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<i8>>();
        })
        .collect();
}

fn calculate_min_path_cost(grid: InputGrid, starting_point: Point) -> ResultGrid {
    let mut min_cost_grid: BaseGrid<Option<i16>> = grid.by_cell().map(|(p, _)| (p, None)).collect();
    let mut heap: BinaryHeap<Cell> = BinaryHeap::new();
    min_cost_grid.setp(starting_point, Some(0));
    heap.push(Cell::new(starting_point, 0));

    while !heap.is_empty() {
        let cell = heap.pop().unwrap();
        let next = grid
            .neighbours(cell.point, false)
            .into_iter()
            .filter(|p| min_cost_grid.getp(*p).unwrap().is_none())
            .map(|p| (p, grid.getp(p)))
            .min_by_key(|(_, c)| *c);
        if next.is_none() {
            continue;
        }
        let (next_point, next_cost) = next.unwrap();
        let cost = cell.min_path_cost + *next_cost.unwrap() as i16;
        min_cost_grid.setp(next_point, Some(cost));
        heap.push(cell);
        heap.push(Cell::new(next_point, cost));
    }

    return min_cost_grid
        .into_by_cell()
        .map(|(p, v)| (p, v.unwrap()))
        .collect();
}

fn part1(input: String) -> i16 {
    let grid = parse_input(input);
    let starting_point = Point::new(0, 0);
    let (w, h) = (grid.width, grid.height);
    let grid = calculate_min_path_cost(grid, starting_point);
    return *grid.get(w - 1, h - 1).unwrap();
}

fn main() {
    run(part1, missing::<i8>);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected: BaseGrid<i8> = vec![
            vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
            vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
            vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
            vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
            vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
            vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
            vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
            vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
            vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
            vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
        ]
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 40);
    }
}
