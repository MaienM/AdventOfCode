use std::cmp::Ordering;

use aoc::utils::{ext::iter::IterExt, parse, point::Point2};

type Point = Point2<isize>;

#[derive(Debug, PartialEq, Eq)]
struct Robot {
    position: Point,
    velocity: Point,
}

fn parse_input(input: &str) -> Vec<Robot> {
    parse!(input => {
        [robots split on '\n' with
            { "p=" [sx as isize] ',' [sy as isize] " v=" [vx as isize] ',' [vy as isize] }
            => Robot { position: Point::new(sx, sy), velocity: Point::new(vx, vy) }
        ]
    } => robots)
}

fn simulate(robots: &[Robot], bounds: &Point, seconds: isize) -> Vec<Point> {
    robots
        .iter()
        .map(|robot| {
            Point::new(
                (robot.position.x + robot.velocity.x * seconds + bounds.x * seconds) % bounds.x,
                (robot.position.y + robot.velocity.y * seconds + bounds.y * seconds) % bounds.y,
            )
        })
        .collect()
}

fn safety_score(positions: &[Point], bounds: &Point) -> usize {
    let xtresh = bounds.x / 2;
    let ytresh = bounds.y / 2;
    let quadrants = positions
        .iter()
        .filter_map(
            |position| match (position.x.cmp(&xtresh), position.y.cmp(&ytresh)) {
                (Ordering::Less, Ordering::Less) => Some(0),
                (Ordering::Less, Ordering::Greater) => Some(1),
                (Ordering::Greater, Ordering::Less) => Some(2),
                (Ordering::Greater, Ordering::Greater) => Some(3),
                _ => None,
            },
        )
        .count_occurences();
    println!("{quadrants:?}");
    quadrants.get(&0).unwrap_or(&0)
        * quadrants.get(&1).unwrap_or(&0)
        * quadrants.get(&2).unwrap_or(&0)
        * quadrants.get(&3).unwrap_or(&0)
}

pub fn solve(input: &str, bounds: Point, seconds: isize) -> usize {
    let robots = parse_input(input);
    let positions = simulate(&robots, &bounds, seconds);
    safety_score(&positions, &bounds)
}

pub fn part1(input: &str) -> usize {
    solve(input, Point::new(101, 103), 100)
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 12, notest)]
    static EXAMPLE_INPUT: &str = "
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    ";

    #[test]
    fn example_simulate() {
        let robots = vec![Robot {
            position: Point::new(2, 4),
            velocity: Point::new(2, -3),
        }];
        let bounds = Point::new(11, 7);

        let expected = vec![Point::new(4, 1)];
        let actual = simulate(&robots, &bounds, 1);
        assert_eq!(actual, expected);

        let expected = vec![Point::new(6, 5)];
        let actual = simulate(&robots, &bounds, 2);
        assert_eq!(actual, expected);

        let expected = vec![Point::new(8, 2)];
        let actual = simulate(&robots, &bounds, 3);
        assert_eq!(actual, expected);

        let expected = vec![Point::new(10, 6)];
        let actual = simulate(&robots, &bounds, 4);
        assert_eq!(actual, expected);

        let expected = vec![Point::new(1, 3)];
        let actual = simulate(&robots, &bounds, 5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        let actual = solve(&EXAMPLE_INPUT, Point::new(11, 7), 100);
        let expected = 12;
        assert_eq!(actual, expected);
    }

    // #[test]
    // fn example_parse() {
    //     let actual = parse_input(&EXAMPLE_INPUT);
    //     let expected = vec![1, 2];
    //     assert_eq!(actual, expected);
    // }
}
