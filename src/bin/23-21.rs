use std::collections::HashSet;

use aoc::utils::point::Point2;

type PointUnbound = Point2<isize>;
type PointBound = Point2<usize>;

#[derive(Debug, PartialEq)]
enum Tile {
    Rock,
    Plot,
}

#[derive(Debug, PartialEq)]
struct Input {
    start: PointUnbound,
    bounds: PointBound,
    tiles: Vec<Vec<Tile>>,
}

fn parse_input(input: &str) -> Input {
    let mut start = None;
    let mut tiles = Vec::new();
    for (y, line) in input.split('\n').enumerate() {
        let mut line_tiles = Vec::new();
        for (x, chr) in line.char_indices() {
            match chr {
                'S' => {
                    start = Some(Point2::new(x as isize, y as isize));
                    line_tiles.push(Tile::Plot);
                }
                '.' => {
                    line_tiles.push(Tile::Plot);
                }
                '#' => {
                    line_tiles.push(Tile::Rock);
                }
                _ => panic!("Invalid tile {chr:?}."),
            };
        }
        tiles.push(line_tiles);
    }
    Input {
        start: start.unwrap(),
        bounds: Point2::new(tiles[0].len(), tiles.len()),
        tiles,
    }
}

fn wrap_point(point: &PointUnbound, bounds: &PointBound) -> PointBound {
    Point2::new(
        (point.x + ((point.x.unsigned_abs() / bounds.x + 1) * bounds.x) as isize) as usize
            % bounds.x,
        (point.y + ((point.y.unsigned_abs() / bounds.y + 1) * bounds.y) as isize) as usize
            % bounds.y,
    )
}

fn solve_naive<const N: usize>(input: &Input, targets: [usize; N]) -> [usize; N] {
    let mut visited_even = HashSet::new();
    let mut visited_odd = HashSet::new();
    visited_even.insert(input.start);

    let mut current = HashSet::new();
    current.insert(input.start);

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
                let wrapped = wrap_point(&neighbor, &input.bounds);
                if input.tiles[wrapped.y][wrapped.x] == Tile::Rock {
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

fn solve(input: &Input, steps: usize) -> usize {
    if steps < input.bounds.x * 6 {
        return solve_naive(input, [steps])[0];
    }

    // There is a consistent growth pattern we can use to calculate the result. To find this pattern we need the first 3 points.
    let remainder = steps % input.bounds.x;
    let times = steps / input.bounds.x;
    let sequence = solve_naive(
        input,
        [
            remainder,
            remainder + input.bounds.x,
            remainder + input.bounds.x * 2,
        ],
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
    let input = parse_input(input);
    solve(&input, 64)
}

pub fn part2(input: &str) -> usize {
    let input = parse_input(input);
    solve(&input, 26_501_365)
}

aoc_runner::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use pretty_assertions::assert_eq;

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
        let expected = Input {
            start: PointUnbound::new(5, 5),
            bounds: PointBound::new(11, 11),
            tiles: vec![
                vec![
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
                vec![
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
                vec![
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
                vec![
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
                vec![
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
                vec![
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
                vec![
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
                vec![
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
                vec![
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
                vec![
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
                vec![
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
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_solve_naive_6() {
        let map = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&map, [6]), [16]);
    }

    #[test]
    fn example_solve_naive_10() {
        let map = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&map, [10]), [50]);
    }

    #[test]
    fn example_solve_naive_50() {
        let map = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&map, [50]), [1594]);
    }

    #[test]
    fn example_solve_naive_100() {
        let map = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&map, [100]), [6536]);
    }

    #[test]
    #[ignore = "slow"]
    fn example_solve_naive_500() {
        let map = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&map, [500]), [167_004]);
    }

    #[test]
    #[ignore = "slow"]
    fn example_solve_naive_1000() {
        let map = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&map, [1000]), [668_697]);
    }

    #[test]
    #[ignore = "slow"]
    fn example_solve_naive_5000() {
        let map = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve_naive(&map, [5000]), [16_733_044]);
    }

    #[test]
    fn wrap_point() {
        assert_eq!(
            super::wrap_point(&PointUnbound::new(-2, -616), &PointBound::new(10, 10)),
            PointBound::new(8, 4)
        );
        assert_eq!(
            super::wrap_point(&PointUnbound::new(4, 8), &PointBound::new(10, 10)),
            PointBound::new(4, 8)
        );
        assert_eq!(
            super::wrap_point(&PointUnbound::new(492, 812), &PointBound::new(10, 10)),
            PointBound::new(2, 2)
        );
    }
}
