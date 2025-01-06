use std::collections::HashSet;

use aoc::grid::{Grid as BaseGrid, Point};
use aoc::runner::*;

type Grid = BaseGrid<i8>;
type Basin = HashSet<Point>;

fn parse_input(input: String) -> Grid {
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
        .collect::<Grid>();
}

fn get_low_points(grid: &Grid) -> Vec<Point> {
    let mut results: Vec<Point> = Vec::new();
    'points: for (point, value) in grid.by_cell() {
        for neighbour in grid.neighbours(point, false) {
            if grid.getp(neighbour).unwrap() <= value {
                continue 'points;
            }
        }
        results.push(point);
    }
    return results;
}

fn expand_basin(grid: &Grid, basin: &mut Basin, point: Point) {
    if basin.contains(&point) || *grid.getp(point).unwrap() == 9 {
        return;
    }
    basin.insert(point);
    for neighbour in grid.neighbours(point, false) {
        expand_basin(grid, basin, neighbour);
    }
}

pub fn part1(input: String) -> u32 {
    let grid = parse_input(input);
    return get_low_points(&grid)
        .into_iter()
        .map(|point| (grid.getp(point).unwrap() + 1) as u32)
        .sum();
}

pub fn part2(input: String) -> u32 {
    let grid = parse_input(input);
    let mut basin_sizes = get_low_points(&grid)
        .into_iter()
        .map(|point| {
            let mut basin = Basin::new();
            expand_basin(&grid, &mut basin, point);
            return basin.len();
        })
        .collect::<Vec<usize>>();
    basin_sizes.sort_unstable();
    return (basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap())
        as u32;
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 15);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 1134);
    }
}
