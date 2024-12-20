use std::cmp::Ordering;

use aoc::utils::{ext::iter::IterExt, parse, point::Point2};

type Point = Point2<isize>;

#[derive(Debug, PartialEq, Eq, Clone)]
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

fn simulate(robots: &mut [Robot], bounds: &Point, seconds: isize) {
    for robot in robots {
        robot.position.x =
            (robot.position.x + robot.velocity.x * seconds + bounds.x * seconds) % bounds.x;
        robot.position.y =
            (robot.position.y + robot.velocity.y * seconds + bounds.y * seconds) % bounds.y;
    }
}

fn safety_score(robots: &[Robot], bounds: &Point) -> usize {
    let xtresh = bounds.x / 2;
    let ytresh = bounds.y / 2;
    let quadrants = robots
        .iter()
        .filter_map(
            |robot| match (robot.position.x.cmp(&xtresh), robot.position.y.cmp(&ytresh)) {
                (Ordering::Less, Ordering::Less) => Some(0),
                (Ordering::Less, Ordering::Greater) => Some(1),
                (Ordering::Greater, Ordering::Less) => Some(2),
                (Ordering::Greater, Ordering::Greater) => Some(3),
                _ => None,
            },
        )
        .count_occurences();
    quadrants.get(&0).unwrap_or(&0)
        * quadrants.get(&1).unwrap_or(&0)
        * quadrants.get(&2).unwrap_or(&0)
        * quadrants.get(&3).unwrap_or(&0)
}

pub fn solve(input: &str, bounds: Point, seconds: isize) -> usize {
    let mut robots = parse_input(input);
    simulate(&mut robots, &bounds, seconds);
    safety_score(&robots, &bounds)
}

pub fn part1(input: &str) -> usize {
    solve(input, Point::new(101, 103), 100)
}

pub fn part2(input: &str) -> usize {
    let mut robots = parse_input(input);
    let bounds = Point::new(101, 103);
    let mut elapsed = 0;
    while robots.iter().map(|r| r.position).count_occurences().len() != robots.len() {
        simulate(&mut robots, &bounds, 1);
        elapsed += 1;
    }

    /*
    for y in 0..bounds.y {
        for x in 0..bounds.x {
            if robots
                .iter()
                .any(|r| r.position.x == x && r.position.y == y)
            {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    */

    elapsed
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
        let initial_robots = vec![Robot {
            position: Point::new(2, 4),
            velocity: Point::new(2, -3),
        }];
        let bounds = Point::new(11, 7);

        let expected = Point::new(4, 1);
        let mut robots = initial_robots.clone();
        simulate(&mut robots, &bounds, 1);
        assert_eq!(robots[0].position, expected);

        let expected = Point::new(6, 5);
        let mut robots = initial_robots.clone();
        simulate(&mut robots, &bounds, 2);
        assert_eq!(robots[0].position, expected);

        let expected = Point::new(8, 2);
        let mut robots = initial_robots.clone();
        simulate(&mut robots, &bounds, 3);
        assert_eq!(robots[0].position, expected);

        let expected = Point::new(10, 6);
        let mut robots = initial_robots.clone();
        simulate(&mut robots, &bounds, 4);
        assert_eq!(robots[0].position, expected);

        let expected = Point::new(1, 3);
        let mut robots = initial_robots.clone();
        simulate(&mut robots, &bounds, 5);
        assert_eq!(robots[0].position, expected);
    }

    #[test]
    fn example_part1() {
        let actual = solve(&EXAMPLE_INPUT, Point::new(11, 7), 100);
        let expected = 12;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Robot {
                position: Point::new(0, 4),
                velocity: Point::new(3, -3),
            },
            Robot {
                position: Point::new(6, 3),
                velocity: Point::new(-1, -3),
            },
            Robot {
                position: Point::new(10, 3),
                velocity: Point::new(-1, 2),
            },
            Robot {
                position: Point::new(2, 0),
                velocity: Point::new(2, -1),
            },
            Robot {
                position: Point::new(0, 0),
                velocity: Point::new(1, 3),
            },
            Robot {
                position: Point::new(3, 0),
                velocity: Point::new(-2, -2),
            },
            Robot {
                position: Point::new(7, 6),
                velocity: Point::new(-1, -3),
            },
            Robot {
                position: Point::new(3, 0),
                velocity: Point::new(-1, -2),
            },
            Robot {
                position: Point::new(9, 3),
                velocity: Point::new(2, 3),
            },
            Robot {
                position: Point::new(7, 3),
                velocity: Point::new(-1, 2),
            },
            Robot {
                position: Point::new(2, 4),
                velocity: Point::new(2, -3),
            },
            Robot {
                position: Point::new(9, 5),
                velocity: Point::new(-3, -3),
            },
        ];
        assert_eq!(actual, expected);
    }
}
