aoc::setup!(title = "Guard Gallivant");

use std::collections::HashSet;

use aoc::point::{Direction2, Point2};

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

#[inline]
fn get_next_direction(direction: Direction2) -> Direction2 {
    match direction {
        Direction2::North => Direction2::East,
        Direction2::East => Direction2::South,
        Direction2::South => Direction2::West,
        Direction2::West => Direction2::North,
    }
}

fn get_next_obstacle<'a>(
    guard: &Point,
    direction: Direction2,
    obstacles: &'a [&'a Point],
    extra_obstacle: Option<&'a &'a Point>,
) -> Option<&'a &'a Point> {
    match direction {
        Direction2::North => obstacles
            .iter()
            .chain(extra_obstacle)
            .filter(|p| p.x == guard.x && p.y < guard.y)
            .max_by_key(|p| p.y),
        Direction2::South => obstacles
            .iter()
            .chain(extra_obstacle)
            .filter(|p| p.x == guard.x && p.y > guard.y)
            .min_by_key(|p| p.y),
        Direction2::East => obstacles
            .iter()
            .chain(extra_obstacle)
            .filter(|p| p.y == guard.y && p.x > guard.x)
            .min_by_key(|p| p.x),
        Direction2::West => obstacles
            .iter()
            .chain(extra_obstacle)
            .filter(|p| p.y == guard.y && p.x < guard.x)
            .max_by_key(|p| p.x),
    }
}

fn add_path_to_visited(
    visited: &mut HashSet<Point>,
    guard: &Point,
    direction: Direction2,
    obstacle: &Point,
) {
    match direction {
        Direction2::North => {
            visited.extend(((obstacle.y + 1)..=guard.y).map(|y| Point::new(guard.x, y)));
        }
        Direction2::South => {
            visited.extend((guard.y..obstacle.y).map(|y| Point::new(guard.x, y)));
        }
        Direction2::West => {
            visited.extend(((obstacle.x + 1)..=guard.x).map(|x| Point::new(x, guard.y)));
        }
        Direction2::East => {
            visited.extend((guard.x..obstacle.x).map(|x| Point::new(x, guard.y)));
        }
    }
}

fn get_path_out(mut guard: Point, obstacles: &HashSet<Point>, bounds: &Point) -> HashSet<Point> {
    let mut visited = HashSet::new();
    visited.insert(guard);
    let obstacles: Vec<_> = obstacles.iter().collect();
    let mut direction = Direction2::North;

    while let Some(obstacle) = get_next_obstacle(&guard, direction, &obstacles, None) {
        add_path_to_visited(&mut visited, &guard, direction, obstacle);
        guard = **obstacle - direction;
        direction = get_next_direction(direction);
    }

    let edge = match direction {
        Direction2::North | Direction2::West => {
            let edge = Point::new(0, 0);
            // The add_path_to_visited doesn't include the 'obstacle', but since we're using unsigned integers for the coordinated we can't position an obstacle past the edge like that, so we just add it to the visited list directly here.
            visited.insert(edge);
            edge
        }
        Direction2::East | Direction2::South => *bounds,
    };
    add_path_to_visited(&mut visited, &guard, direction, &edge);

    visited
}

fn check_loop(mut guard: Point, obstacles: &HashSet<Point>, extra_obstacle: &Point) -> bool {
    let mut bumped = HashSet::new();
    let mut direction = Direction2::North;
    let obstacles: Vec<_> = obstacles.iter().collect();
    while let Some(obstacle) =
        get_next_obstacle(&guard, direction, &obstacles, Some(&extra_obstacle))
    {
        if !bumped.insert((obstacle, direction)) {
            return true;
        }
        guard = **obstacle - direction;
        direction = get_next_direction(direction);
    }
    false
}

pub fn part1(input: &str) -> usize {
    let (guard, obstacles, bounds) = parse_input(input);
    get_path_out(guard, &obstacles, &bounds).len()
}

pub fn part2(input: &str) -> usize {
    let (guard, obstacles, bounds) = parse_input(input);
    get_path_out(guard, &obstacles, &bounds)
        .into_par_iter()
        .filter(|point| check_loop(guard, &obstacles, point))
        .count()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

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
