use std::collections::HashSet;

use aoc::utils::point::{Direction2, Point2};

type Point = Point2<usize>;

fn parse_input(input: &str) -> (Point, HashSet<Point>, Point) {
    let mut guard = None;
    let mut obstacles = HashSet::new();
    let bounds = Point::new(
        input.split('\n').next().unwrap().len(),
        input.split('\n').count(),
    );
    for (y, line) in input.split('\n').enumerate() {
        for (x, char) in line.char_indices() {
            match char {
                '^' => {
                    guard = Some(Point::new(x, y));
                }
                '#' => {
                    obstacles.insert(Point::new(x, y));
                }
                _ => {}
            }
        }
    }
    (guard.unwrap(), obstacles, bounds)
}

fn get_path_out(guard: &Point, obstacles: &HashSet<Point>, bounds: &Point) -> HashSet<Point> {
    let mut visited = HashSet::new();
    visited.insert(*guard);
    let mut guard = (*guard, Direction2::North);
    while let Some(nextpoint) = guard.0.checked_add_direction(guard.1, &1) {
        if obstacles.contains(&nextpoint) {
            guard.1 = match guard.1 {
                Direction2::North => Direction2::East,
                Direction2::East => Direction2::South,
                Direction2::South => Direction2::West,
                Direction2::West => Direction2::North,
            };
            continue;
        }

        if nextpoint.x >= bounds.x || nextpoint.y >= bounds.y {
            break;
        }
        visited.insert(nextpoint);
        guard.0 = nextpoint;
    }
    visited
}

fn check_loop(
    guard: &Point,
    obstacles: &HashSet<Point>,
    extra_obstacle: &Point,
    bounds: &Point,
) -> bool {
    let mut bumped = HashSet::new();
    let mut guard = (*guard, Direction2::North);
    while let Some(nextpoint) = guard.0.checked_add_direction(guard.1, &1) {
        if obstacles.contains(&nextpoint) || &nextpoint == extra_obstacle {
            if !bumped.insert((nextpoint, guard.1)) {
                return true;
            }
            guard.1 = match guard.1 {
                Direction2::North => Direction2::East,
                Direction2::East => Direction2::South,
                Direction2::South => Direction2::West,
                Direction2::West => Direction2::North,
            };
            continue;
        }

        if nextpoint.x > bounds.x || nextpoint.y > bounds.y {
            return false;
        }
        guard.0 = nextpoint;
    }
    false
}

pub fn part1(input: &str) -> usize {
    let (guard, obstacles, bounds) = parse_input(input);
    let visited = get_path_out(&guard, &obstacles, &bounds);
    visited.len()
}

pub fn part2(input: &str) -> usize {
    let (guard, obstacles, bounds) = parse_input(input);
    let visited = get_path_out(&guard, &obstacles, &bounds);
    let mut loops = 0;
    for point in visited {
        if point == guard {
            continue;
        }
        if check_loop(&guard, &obstacles, &point, &bounds) {
            loops += 1;
        }
    }
    loops
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 41, part2 = 6)]
    static EXAMPLE_INPUT: &str = "
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = (
            Point::new(4, 6),
            vec![
                Point::new(4, 0),
                Point::new(9, 1),
                Point::new(2, 3),
                Point::new(7, 4),
                Point::new(1, 6),
                Point::new(8, 7),
                Point::new(0, 8),
                Point::new(6, 9),
            ]
            .into_iter()
            .collect(),
            Point::new(10, 10),
        );
        assert_eq!(actual, expected);
    }
}
