use std::collections::HashSet;

use aoc::grid::{Grid as BaseGrid, Point};
use aoc::runner::*;

type Grid = BaseGrid<i8>;

fn parse_input(input: String) -> Grid {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|chr| chr.to_digit(10).unwrap() as i8)
                .into_iter()
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>()
        .into();
}

fn do_round(grid: &Grid) -> (Grid, i64) {
    let mut new_grid: Grid = grid
        .by_cell()
        .map(|(point, level)| (point, level + 1))
        .collect();

    let mut flashed: HashSet<Point> = HashSet::new();
    loop {
        let mut should_flash: HashSet<Point> = HashSet::new();

        for (point, level) in new_grid.by_cell() {
            if flashed.contains(&point) {
                continue;
            }
            if level > &9 {
                should_flash.insert(point);
            }
        }
        for point in &should_flash {
            flashed.insert(*point);
            for neighbour in new_grid.neighbours(*point, true) {
                new_grid.mutatep(neighbour, |level| level + 1);
            }
        }

        if should_flash.is_empty() {
            break;
        }
    }
    for point in &flashed {
        new_grid.setp(*point, 0);
    }
    return (new_grid, flashed.len() as i64);
}

pub fn part1(input: String) -> i64 {
    let mut grid = parse_input(input);
    let mut flashes = 0_i64;
    for _ in 0..100 {
        let (new_grid, new_flashes) = do_round(&grid);
        grid = new_grid;
        flashes += new_flashes;
    }
    return flashes;
}

pub fn part2(input: String) -> i64 {
    let mut grid = parse_input(input);
    for round in 1.. {
        let (new_grid, new_flashes) = do_round(&grid);
        grid = new_grid;
        if new_flashes == 100 {
            return round;
        }
    }
    panic!("How did you get here?");
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected: Grid = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_step_1() {
        let (actual_grid, actual_steps) = do_round(
            &vec![
                vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
                vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
                vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
                vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
                vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
                vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
                vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
                vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
                vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
                vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
            ]
            .into(),
        );
        assert_eq!(actual_steps, 0);
        assert_eq!(
            actual_grid,
            vec![
                vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
                vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
                vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
                vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
                vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
                vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
                vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
                vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
                vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
                vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
            ]
            .into(),
        );
    }

    #[test]
    fn example_step_2() {
        let (actual_grid, actual_steps) = do_round(
            &vec![
                vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
                vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
                vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
                vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
                vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
                vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
                vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
                vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
                vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
                vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
            ]
            .into(),
        );
        assert_eq!(actual_steps, 35);
        assert_eq!(
            actual_grid,
            vec![
                vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
                vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
                vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
                vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
                vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
                vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
                vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
                vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
                vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
                vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
            ]
            .into(),
        );
    }

    #[test]
    fn example_step_3() {
        let (actual_grid, actual_steps) = do_round(
            &vec![
                vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
                vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
                vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
                vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
                vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
                vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
                vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
                vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
                vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
                vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
            ]
            .into(),
        );
        assert_eq!(actual_steps, 45);
        assert_eq!(
            actual_grid,
            vec![
                vec![0, 0, 5, 0, 9, 0, 0, 8, 6, 6],
                vec![8, 5, 0, 0, 8, 0, 0, 5, 7, 5],
                vec![9, 9, 0, 0, 0, 0, 0, 0, 3, 9],
                vec![9, 7, 0, 0, 0, 0, 0, 0, 4, 1],
                vec![9, 9, 3, 5, 0, 8, 0, 0, 6, 3],
                vec![7, 7, 1, 2, 3, 0, 0, 0, 0, 0],
                vec![7, 9, 1, 1, 2, 5, 0, 0, 0, 9],
                vec![2, 2, 1, 1, 1, 3, 0, 0, 0, 0],
                vec![0, 4, 2, 1, 1, 2, 5, 0, 0, 0],
                vec![0, 0, 2, 1, 1, 1, 9, 0, 0, 0],
            ]
            .into(),
        );
    }

    #[test]
    fn example_step_4() {
        let (actual_grid, actual_steps) = do_round(
            &vec![
                vec![0, 0, 5, 0, 9, 0, 0, 8, 6, 6],
                vec![8, 5, 0, 0, 8, 0, 0, 5, 7, 5],
                vec![9, 9, 0, 0, 0, 0, 0, 0, 3, 9],
                vec![9, 7, 0, 0, 0, 0, 0, 0, 4, 1],
                vec![9, 9, 3, 5, 0, 8, 0, 0, 6, 3],
                vec![7, 7, 1, 2, 3, 0, 0, 0, 0, 0],
                vec![7, 9, 1, 1, 2, 5, 0, 0, 0, 9],
                vec![2, 2, 1, 1, 1, 3, 0, 0, 0, 0],
                vec![0, 4, 2, 1, 1, 2, 5, 0, 0, 0],
                vec![0, 0, 2, 1, 1, 1, 9, 0, 0, 0],
            ]
            .into(),
        );
        assert_eq!(actual_steps, 16);
        assert_eq!(
            actual_grid,
            vec![
                vec![2, 2, 6, 3, 0, 3, 1, 9, 7, 7],
                vec![0, 9, 2, 3, 0, 3, 1, 6, 9, 7],
                vec![0, 0, 3, 2, 2, 2, 1, 1, 5, 0],
                vec![0, 0, 4, 1, 1, 1, 1, 1, 6, 3],
                vec![0, 0, 7, 6, 1, 9, 1, 1, 7, 4],
                vec![0, 0, 5, 3, 4, 1, 1, 1, 2, 2],
                vec![0, 0, 4, 2, 3, 6, 1, 1, 2, 0],
                vec![5, 5, 3, 2, 2, 4, 1, 1, 2, 2],
                vec![1, 5, 3, 2, 2, 4, 7, 2, 1, 1],
                vec![1, 1, 3, 2, 2, 3, 0, 2, 1, 1],
            ]
            .into(),
        );
    }

    #[test]
    fn example_step_5() {
        let (actual_grid, actual_steps) = do_round(
            &vec![
                vec![2, 2, 6, 3, 0, 3, 1, 9, 7, 7],
                vec![0, 9, 2, 3, 0, 3, 1, 6, 9, 7],
                vec![0, 0, 3, 2, 2, 2, 1, 1, 5, 0],
                vec![0, 0, 4, 1, 1, 1, 1, 1, 6, 3],
                vec![0, 0, 7, 6, 1, 9, 1, 1, 7, 4],
                vec![0, 0, 5, 3, 4, 1, 1, 1, 2, 2],
                vec![0, 0, 4, 2, 3, 6, 1, 1, 2, 0],
                vec![5, 5, 3, 2, 2, 4, 1, 1, 2, 2],
                vec![1, 5, 3, 2, 2, 4, 7, 2, 1, 1],
                vec![1, 1, 3, 2, 2, 3, 0, 2, 1, 1],
            ]
            .into(),
        );
        assert_eq!(actual_steps, 8);
        assert_eq!(
            actual_grid,
            vec![
                vec![4, 4, 8, 4, 1, 4, 4, 0, 0, 0],
                vec![2, 0, 4, 4, 1, 4, 4, 0, 0, 0],
                vec![2, 2, 5, 3, 3, 3, 3, 4, 9, 3],
                vec![1, 1, 5, 2, 3, 3, 3, 2, 7, 4],
                vec![1, 1, 8, 7, 3, 0, 3, 2, 8, 5],
                vec![1, 1, 6, 4, 6, 3, 3, 2, 3, 3],
                vec![1, 1, 5, 3, 4, 7, 2, 2, 3, 1],
                vec![6, 6, 4, 3, 3, 5, 2, 2, 3, 3],
                vec![2, 6, 4, 3, 3, 5, 8, 3, 2, 2],
                vec![2, 2, 4, 3, 3, 4, 1, 3, 2, 2],
            ]
            .into(),
        );
    }

    #[test]
    fn example_step_6() {
        let (actual_grid, actual_steps) = do_round(
            &vec![
                vec![4, 4, 8, 4, 1, 4, 4, 0, 0, 0],
                vec![2, 0, 4, 4, 1, 4, 4, 0, 0, 0],
                vec![2, 2, 5, 3, 3, 3, 3, 4, 9, 3],
                vec![1, 1, 5, 2, 3, 3, 3, 2, 7, 4],
                vec![1, 1, 8, 7, 3, 0, 3, 2, 8, 5],
                vec![1, 1, 6, 4, 6, 3, 3, 2, 3, 3],
                vec![1, 1, 5, 3, 4, 7, 2, 2, 3, 1],
                vec![6, 6, 4, 3, 3, 5, 2, 2, 3, 3],
                vec![2, 6, 4, 3, 3, 5, 8, 3, 2, 2],
                vec![2, 2, 4, 3, 3, 4, 1, 3, 2, 2],
            ]
            .into(),
        );
        assert_eq!(actual_steps, 1);
        assert_eq!(
            actual_grid,
            vec![
                vec![5, 5, 9, 5, 2, 5, 5, 1, 1, 1],
                vec![3, 1, 5, 5, 2, 5, 5, 2, 2, 2],
                vec![3, 3, 6, 4, 4, 4, 4, 6, 0, 5],
                vec![2, 2, 6, 3, 4, 4, 4, 4, 9, 6],
                vec![2, 2, 9, 8, 4, 1, 4, 3, 9, 6],
                vec![2, 2, 7, 5, 7, 4, 4, 3, 4, 4],
                vec![2, 2, 6, 4, 5, 8, 3, 3, 4, 2],
                vec![7, 7, 5, 4, 4, 6, 3, 3, 4, 4],
                vec![3, 7, 5, 4, 4, 6, 9, 4, 3, 3],
                vec![3, 3, 5, 4, 4, 5, 2, 4, 3, 3],
            ]
            .into(),
        );
    }

    #[test]
    fn example_step_7() {
        let (actual_grid, actual_steps) = do_round(
            &vec![
                vec![5, 5, 9, 5, 2, 5, 5, 1, 1, 1],
                vec![3, 1, 5, 5, 2, 5, 5, 2, 2, 2],
                vec![3, 3, 6, 4, 4, 4, 4, 6, 0, 5],
                vec![2, 2, 6, 3, 4, 4, 4, 4, 9, 6],
                vec![2, 2, 9, 8, 4, 1, 4, 3, 9, 6],
                vec![2, 2, 7, 5, 7, 4, 4, 3, 4, 4],
                vec![2, 2, 6, 4, 5, 8, 3, 3, 4, 2],
                vec![7, 7, 5, 4, 4, 6, 3, 3, 4, 4],
                vec![3, 7, 5, 4, 4, 6, 9, 4, 3, 3],
                vec![3, 3, 5, 4, 4, 5, 2, 4, 3, 3],
            ]
            .into(),
        );
        assert_eq!(actual_steps, 7);
        assert_eq!(
            actual_grid,
            vec![
                vec![6, 7, 0, 7, 3, 6, 6, 2, 2, 2],
                vec![4, 3, 7, 7, 3, 6, 6, 3, 3, 3],
                vec![4, 4, 7, 5, 5, 5, 5, 8, 2, 7],
                vec![3, 4, 9, 6, 6, 5, 5, 7, 0, 9],
                vec![3, 5, 0, 0, 6, 2, 5, 6, 0, 9],
                vec![3, 5, 0, 9, 9, 5, 5, 5, 6, 6],
                vec![3, 4, 8, 6, 6, 9, 4, 4, 5, 3],
                vec![8, 8, 6, 5, 5, 8, 5, 5, 5, 5],
                vec![4, 8, 6, 5, 5, 8, 0, 6, 4, 4],
                vec![4, 4, 6, 5, 5, 7, 4, 6, 4, 4],
            ]
            .into(),
        );
    }

    #[test]
    fn example_step_8() {
        let (actual_grid, actual_steps) = do_round(
            &vec![
                vec![6, 7, 0, 7, 3, 6, 6, 2, 2, 2],
                vec![4, 3, 7, 7, 3, 6, 6, 3, 3, 3],
                vec![4, 4, 7, 5, 5, 5, 5, 8, 2, 7],
                vec![3, 4, 9, 6, 6, 5, 5, 7, 0, 9],
                vec![3, 5, 0, 0, 6, 2, 5, 6, 0, 9],
                vec![3, 5, 0, 9, 9, 5, 5, 5, 6, 6],
                vec![3, 4, 8, 6, 6, 9, 4, 4, 5, 3],
                vec![8, 8, 6, 5, 5, 8, 5, 5, 5, 5],
                vec![4, 8, 6, 5, 5, 8, 0, 6, 4, 4],
                vec![4, 4, 6, 5, 5, 7, 4, 6, 4, 4],
            ]
            .into(),
        );
        assert_eq!(actual_steps, 24);
        assert_eq!(
            actual_grid,
            vec![
                vec![7, 8, 1, 8, 4, 7, 7, 3, 3, 3],
                vec![5, 4, 8, 8, 4, 7, 7, 4, 4, 4],
                vec![5, 6, 9, 7, 6, 6, 6, 9, 4, 9],
                vec![4, 6, 0, 8, 7, 6, 6, 8, 3, 0],
                vec![4, 7, 3, 4, 9, 4, 6, 7, 3, 0],
                vec![4, 7, 4, 0, 0, 9, 7, 6, 8, 8],
                vec![6, 9, 0, 0, 0, 0, 7, 5, 6, 4],
                vec![0, 0, 0, 0, 0, 0, 9, 6, 6, 6],
                vec![8, 0, 0, 0, 0, 0, 4, 7, 5, 5],
                vec![6, 8, 0, 0, 0, 0, 7, 7, 5, 5],
            ]
            .into(),
        );
    }

    #[test]
    fn example_step_9() {
        let (actual_grid, actual_steps) = do_round(
            &vec![
                vec![7, 8, 1, 8, 4, 7, 7, 3, 3, 3],
                vec![5, 4, 8, 8, 4, 7, 7, 4, 4, 4],
                vec![5, 6, 9, 7, 6, 6, 6, 9, 4, 9],
                vec![4, 6, 0, 8, 7, 6, 6, 8, 3, 0],
                vec![4, 7, 3, 4, 9, 4, 6, 7, 3, 0],
                vec![4, 7, 4, 0, 0, 9, 7, 6, 8, 8],
                vec![6, 9, 0, 0, 0, 0, 7, 5, 6, 4],
                vec![0, 0, 0, 0, 0, 0, 9, 6, 6, 6],
                vec![8, 0, 0, 0, 0, 0, 4, 7, 5, 5],
                vec![6, 8, 0, 0, 0, 0, 7, 7, 5, 5],
            ]
            .into(),
        );
        assert_eq!(actual_steps, 39);
        assert_eq!(
            actual_grid,
            vec![
                vec![9, 0, 6, 0, 0, 0, 0, 6, 4, 4],
                vec![7, 8, 0, 0, 0, 0, 0, 9, 7, 6],
                vec![6, 9, 0, 0, 0, 0, 0, 0, 8, 0],
                vec![5, 8, 4, 0, 0, 0, 0, 0, 8, 2],
                vec![5, 8, 5, 8, 0, 0, 0, 0, 9, 3],
                vec![6, 9, 6, 2, 4, 0, 0, 0, 0, 0],
                vec![8, 0, 2, 1, 2, 5, 0, 0, 0, 9],
                vec![2, 2, 2, 1, 1, 3, 0, 0, 0, 9],
                vec![9, 1, 1, 1, 1, 2, 8, 0, 9, 7],
                vec![7, 9, 1, 1, 1, 1, 9, 9, 7, 6],
            ]
            .into(),
        );
    }

    #[test]
    fn example_step_10() {
        let (actual_grid, actual_steps) = do_round(
            &vec![
                vec![9, 0, 6, 0, 0, 0, 0, 6, 4, 4],
                vec![7, 8, 0, 0, 0, 0, 0, 9, 7, 6],
                vec![6, 9, 0, 0, 0, 0, 0, 0, 8, 0],
                vec![5, 8, 4, 0, 0, 0, 0, 0, 8, 2],
                vec![5, 8, 5, 8, 0, 0, 0, 0, 9, 3],
                vec![6, 9, 6, 2, 4, 0, 0, 0, 0, 0],
                vec![8, 0, 2, 1, 2, 5, 0, 0, 0, 9],
                vec![2, 2, 2, 1, 1, 3, 0, 0, 0, 9],
                vec![9, 1, 1, 1, 1, 2, 8, 0, 9, 7],
                vec![7, 9, 1, 1, 1, 1, 9, 9, 7, 6],
            ]
            .into(),
        );
        assert_eq!(actual_steps, 29);
        assert_eq!(
            actual_grid,
            vec![
                vec![0, 4, 8, 1, 1, 1, 2, 9, 7, 6],
                vec![0, 0, 3, 1, 1, 1, 2, 0, 0, 9],
                vec![0, 0, 4, 1, 1, 1, 2, 5, 0, 4],
                vec![0, 0, 8, 1, 1, 1, 1, 4, 0, 6],
                vec![0, 0, 9, 9, 1, 1, 1, 3, 0, 6],
                vec![0, 0, 9, 3, 5, 1, 1, 2, 3, 3],
                vec![0, 4, 4, 2, 3, 6, 1, 1, 3, 0],
                vec![5, 5, 3, 2, 2, 5, 2, 3, 5, 0],
                vec![0, 5, 3, 2, 2, 5, 0, 6, 0, 0],
                vec![0, 0, 3, 2, 2, 4, 0, 0, 0, 0],
            ]
            .into(),
        );
    }

    #[test]
    fn example_small_1() {
        let (actual_grid, actual_steps) = do_round(
            &vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 9, 9, 9, 1],
                vec![1, 9, 1, 9, 1],
                vec![1, 9, 9, 9, 1],
                vec![1, 1, 1, 1, 1],
            ]
            .into(),
        );
        assert_eq!(actual_steps, 9);
        assert_eq!(
            actual_grid,
            vec![
                vec![3, 4, 5, 4, 3],
                vec![4, 0, 0, 0, 4],
                vec![5, 0, 0, 0, 5],
                vec![4, 0, 0, 0, 4],
                vec![3, 4, 5, 4, 3],
            ]
            .into(),
        );
    }

    #[test]
    fn example_small_2() {
        let (actual_grid, actual_steps) = do_round(
            &vec![
                vec![3, 4, 5, 4, 3],
                vec![4, 0, 0, 0, 4],
                vec![5, 0, 0, 0, 5],
                vec![4, 0, 0, 0, 4],
                vec![3, 4, 5, 4, 3],
            ]
            .into(),
        );
        assert_eq!(actual_steps, 0);
        assert_eq!(
            actual_grid,
            vec![
                vec![4, 5, 6, 5, 4],
                vec![5, 1, 1, 1, 5],
                vec![6, 1, 1, 1, 6],
                vec![5, 1, 1, 1, 5],
                vec![4, 5, 6, 5, 4],
            ]
            .into(),
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 1656);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 195);
    }
}
