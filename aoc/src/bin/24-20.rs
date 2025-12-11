puzzle_runner::register_chapter!(book = "2024", title = "Race Condition");

use std::collections::BinaryHeap;

use puzzle_lib::{grid::FullGrid, point::Point2};

type Grid = FullGrid<bool>;
type StepGrid = FullGrid<usize>;

fn parse_input(input: &str) -> (Grid, Point2, Point2) {
    parse!(input => {
        [grid cells match {
            '#' => (false),
            '.' => (true),
            'S' => index into start => (true),
            'E' => index into end => (true),
        }]
    } => (grid, start, end))
}

fn calculate_step_grid(grid: &Grid, start: &Point2) -> StepGrid {
    let mut paths = BinaryHeap::new();
    paths.push((0, *start));
    let mut result = grid.map(|_| 0);
    while let Some((steps, point)) = paths.pop() {
        if !grid[point] || result[point] > 0 {
            continue;
        }

        result[point] = -steps as usize;

        for neigh in point.neighbours_ortho() {
            paths.push((steps - 1, neigh));
        }
    }
    result[*start] = 0;
    result
}

fn find_cheat_paths(
    grid: &Grid,
    start: &Point2,
    end: &Point2,
    cheat: usize,
    min_save: usize,
) -> usize {
    let non_wall_points: Vec<_> = grid
        .iter_pairs()
        .filter_map(|(point, cell)| if *cell { Some(*point) } else { None })
        .collect();
    let from_start = calculate_step_grid(grid, start);
    let from_end = calculate_step_grid(grid, end);
    let threshold = from_start[*end] - min_save;
    non_wall_points
        .par_iter()
        .map(|point1| {
            non_wall_points
                .iter()
                .filter(|point2| {
                    let distance = point1.abs_diff(point2).sum();
                    distance <= cheat
                        && from_start[*point1] + distance + from_end[**point2] <= threshold
                })
                .count()
        })
        .sum()
}

fn part1impl(input: &str, min_save: usize) -> usize {
    let (grid, start, end) = parse_input(input);
    find_cheat_paths(&grid, &start, &end, 2, min_save)
}

#[register_part]
fn part1(input: &str) -> usize {
    part1impl(input, 100)
}

fn part2impl(input: &str, min_save: usize) -> usize {
    let (grid, start, end) = parse_input(input);
    find_cheat_paths(&grid, &start, &end, 20, min_save)
}

#[register_part]
fn part2(input: &str) -> usize {
    part2impl(input, 100)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 44, part2 = 285, notest)]
    static EXAMPLE_INPUT: &str = "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = (
            [
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
                [
                    false, true, true, true, false, true, true, true, false, true, true, true,
                    true, true, false,
                ],
                [
                    false, true, false, true, false, true, false, true, false, true, false, false,
                    false, true, false,
                ],
                [
                    false, true, false, true, true, true, false, true, false, true, false, true,
                    true, true, false,
                ],
                [
                    false, false, false, false, false, false, false, true, false, true, false,
                    true, false, false, false,
                ],
                [
                    false, false, false, false, false, false, false, true, false, true, false,
                    true, true, true, false,
                ],
                [
                    false, false, false, false, false, false, false, true, false, true, false,
                    false, false, true, false,
                ],
                [
                    false, false, false, true, true, true, false, true, true, true, false, true,
                    true, true, false,
                ],
                [
                    false, false, false, true, false, false, false, false, false, false, false,
                    true, false, false, false,
                ],
                [
                    false, true, true, true, false, false, false, true, true, true, false, true,
                    true, true, false,
                ],
                [
                    false, true, false, false, false, false, false, true, false, true, false,
                    false, false, true, false,
                ],
                [
                    false, true, false, true, true, true, false, true, false, true, false, true,
                    true, true, false,
                ],
                [
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, false, false,
                ],
                [
                    false, true, true, true, false, true, true, true, false, true, true, true,
                    false, false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ]
            .into(),
            Point2::new(1, 3),
            Point2::new(5, 7),
        );
        vec![1, 2];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_test_1() {
        let actual = part1impl(&EXAMPLE_INPUT, 1).to_string();
        assert_eq!(actual, EXAMPLE_INPUT.parts[&1]);
    }

    #[test]
    fn example_test_2() {
        let actual = part2impl(&EXAMPLE_INPUT, 50).to_string();
        assert_eq!(actual, EXAMPLE_INPUT.parts[&2]);
    }
}
