use std::collections::HashSet;

use aoc::utils::point::Point2;

type Point = Point2;

#[derive(Debug, PartialEq)]
struct Input {
    start: Point,
    rocks: HashSet<Point>,
}

fn parse_input(input: &str) -> Input {
    let mut rocks = HashSet::new();
    let mut start = None;
    for (y, line) in input.split('\n').enumerate() {
        for (x, chr) in line.char_indices() {
            match chr {
                'S' => {
                    start = Some(Point2::new(x, y));
                }
                '#' => {
                    rocks.insert(Point2::new(x, y));
                }
                '.' => {}
                _ => panic!("Invalid map tile {chr:?}."),
            };
        }
    }
    Input {
        start: start.unwrap(),
        rocks,
    }
}

fn solve(input: &Input, steps: usize) -> usize {
    let mut visited_even = HashSet::new();
    let mut visited_odd = HashSet::new();
    let mut current = HashSet::new();

    visited_even.insert(input.start);
    current.insert(input.start);

    for remaining in (0..steps).rev() {
        let visited = if remaining % 2 == 0 {
            &mut visited_even
        } else {
            &mut visited_odd
        };

        let mut next = HashSet::new();
        for point in current {
            for neighbor in point.neighbours_ortho() {
                if input.rocks.contains(&neighbor) {
                    continue;
                }
                if visited.insert(neighbor) {
                    next.insert(neighbor);
                }
            }
        }
        current = next;
    }
    visited_even.len()
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    solve(&input, 64)
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use common_macros::hash_set;
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

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = Input {
            start: Point::new(5, 5),
            rocks: hash_set![
                Point::new(5, 1),
                Point::new(6, 1),
                Point::new(7, 1),
                Point::new(9, 1),
                Point::new(1, 2),
                Point::new(2, 2),
                Point::new(3, 2),
                Point::new(5, 2),
                Point::new(6, 2),
                Point::new(9, 2),
                Point::new(2, 3),
                Point::new(4, 3),
                Point::new(8, 3),
                Point::new(4, 4),
                Point::new(6, 4),
                Point::new(1, 5),
                Point::new(2, 5),
                Point::new(6, 5),
                Point::new(7, 5),
                Point::new(8, 5),
                Point::new(9, 5),
                Point::new(1, 6),
                Point::new(2, 6),
                Point::new(5, 6),
                Point::new(9, 6),
                Point::new(7, 7),
                Point::new(8, 7),
                Point::new(1, 8),
                Point::new(2, 8),
                Point::new(4, 8),
                Point::new(6, 8),
                Point::new(7, 8),
                Point::new(8, 8),
                Point::new(9, 8),
                Point::new(1, 9),
                Point::new(2, 9),
                Point::new(5, 9),
                Point::new(6, 9),
                Point::new(8, 9),
                Point::new(9, 9),
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_solve() {
        let map = parse_input(&EXAMPLE_INPUT);
        assert_eq!(solve(&map, 6), 16);
    }
}
