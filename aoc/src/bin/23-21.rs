puzzle_runner::register_chapter!(book = "2023", title = "Step Counter");

use std::collections::HashSet;

use puzzle_lib::{grid::FullGrid, point::Point2};

type Point = Point2<usize>;
type PointUnbound = Point2<isize>;

#[derive(Debug, PartialEq)]
enum Tile {
    Rock,
    Plot,
}

type Grid = FullGrid<Tile>;

fn parse_input(input: &str) -> (Grid, Point2) {
    parse!(input => {
        [grid cells match {
            'S' => index into start => Tile::Plot,
            '.' => Tile::Plot,
            '#' => Tile::Rock,
        }]
    } => (grid, start))
}

fn wrap_point(point: &PointUnbound, bounds: &Point) -> Point {
    Point2::new(
        (point.x + ((point.x.unsigned_abs() / bounds.x + 1) * bounds.x) as isize) as usize
            % bounds.x,
        (point.y + ((point.y.unsigned_abs() / bounds.y + 1) * bounds.y) as isize) as usize
            % bounds.y,
    )
}

fn solve_naive<const N: usize>(grid: &Grid, start: &Point, targets: [usize; N]) -> [usize; N] {
    let bounds = grid.area().1;
    let start: PointUnbound = start.try_cast().unwrap();

    let mut visited_even = HashSet::new();
    let mut visited_odd = HashSet::new();
    visited_even.insert(start);

    let mut current = HashSet::new();
    current.insert(start);

    let mut targetidx = 0;
    let mut results = [0; N];

    for steps in 1.. {
        let visited = if steps % 2 == 0 {
            &mut visited_even
        } else {
            &mut visited_odd
        };

        let mut next = HashSet::new();
        for point in current {
            for neighbor in point.neighbours_ortho() {
                let wrapped = wrap_point(&neighbor, &bounds);
                if grid[wrapped] == Tile::Rock {
                    continue;
                }
                if visited.insert(neighbor) {
                    next.insert(neighbor);
                }
            }
        }
        current = next;

        if steps == targets[targetidx] {
            results[targetidx] = visited.len();
            targetidx += 1;
            if targetidx == N {
                break;
            }
        }
    }
    results
}

fn solve(grid: &Grid, start: &Point, steps: usize) -> usize {
    let bounds = grid.area().1;

    if steps < bounds.x * 6 {
        return solve_naive(grid, start, [steps])[0];
    }

    // There is a consistent growth pattern we can use to calculate the result. To find this pattern we need the first 3 points.
    let remainder = steps % bounds.x;
    let times = steps / bounds.x;
    let sequence = solve_naive(
        grid,
        start,
        [remainder, remainder + bounds.x, remainder + bounds.x * 2],
    );

    // The difference between two results are not consistent, but the difference between these differences are, so calculate this.
    let diffofdiffs = (sequence[2] - sequence[1]) - (sequence[1] - sequence[0]);
    let mut diff = sequence[2] - sequence[1];
    let mut result = sequence[2];
    for _ in 2..times {
        diff += diffofdiffs;
        result += diff;
    }
    result
}

pub fn part1(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    solve(&grid, &start, 64)
}

pub fn part2(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    solve(&grid, &start, 26_501_365)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input]
    static EXAMPLE_INPUT: &str = "
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    ";

    #[allow(clippy::too_many_lines)]
    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = (
            Grid::from([
                [
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                ],
                [
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Plot,
                ],
                [
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Plot,
                ],
                [
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Plot,
                ],
                [
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                ],
                [
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Plot,
                ],
                [
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Plot,
                ],
                [
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Plot,
                ],
                [
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Plot,
                ],
                [
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Plot,
                    Tile::Rock,
                    Tile::Rock,
                    Tile::Plot,
                ],
                [
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                    Tile::Plot,
                ],
            ]),
            Point::new(5, 5),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_solve_naive_6() {
        let (grid, start) = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&grid, &start, [6]), [16]);
    }

    #[test]
    fn example_solve_naive_10() {
        let (grid, start) = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&grid, &start, [10]), [50]);
    }

    #[test]
    fn example_solve_naive_50() {
        let (grid, start) = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&grid, &start, [50]), [1594]);
    }

    #[test]
    fn example_solve_naive_100() {
        let (grid, start) = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&grid, &start, [100]), [6536]);
    }

    #[test]
    #[ignore = "slow"]
    fn example_solve_naive_500() {
        let (grid, start) = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&grid, &start, [500]), [167_004]);
    }

    #[test]
    #[ignore = "slow"]
    fn example_solve_naive_1000() {
        let (grid, start) = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&grid, &start, [1000]), [668_697]);
    }

    #[test]
    #[ignore = "slow"]
    fn example_solve_naive_5000() {
        let (grid, start) = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&grid, &start, [5000]), [16_733_044]);
    }

    #[test]
    fn wrap_point() {
        assert_eq!(
            super::wrap_point(&PointUnbound::new(-2, -616), &Point::new(10, 10)),
            Point::new(8, 4)
        );
        assert_eq!(
            super::wrap_point(&PointUnbound::new(4, 8), &Point::new(10, 10)),
            Point::new(4, 8)
        );
        assert_eq!(
            super::wrap_point(&PointUnbound::new(492, 812), &Point::new(10, 10)),
            Point::new(2, 2)
        );
    }
}
