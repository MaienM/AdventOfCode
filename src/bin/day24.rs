use std::ops::Range;

use aoc::utils::{parse, point::Point3};

type Point = Point3<f64>;

#[derive(Debug, PartialEq)]
struct Hailstone {
    position: Point,
    velocity: Point,
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    parse!(input => {
        [hailstones split on '\n' with
            { px ", " py ", " pz " @ " vx ", " vy ", " vz }
            => Hailstone {
                position: Point::new(
                    px.trim().parse().unwrap(),
                    py.trim().parse().unwrap(),
                    pz.trim().parse().unwrap(),
                ),
                velocity: Point::new(
                    vx.trim().parse().unwrap(),
                    vy.trim().parse().unwrap(),
                    vz.trim().parse().unwrap(),
                ),
            }
        ]
    } => hailstones)
}

fn calculate_path_intersection_xy(stone_a: &Hailstone, stone_b: &Hailstone) -> Option<Point> {
    let a1 = stone_a.velocity.y;
    let b1 = -stone_a.velocity.x;
    let c1 = stone_a.position.x * a1 + stone_a.position.y * b1;

    let a2 = stone_b.velocity.y;
    let b2 = -stone_b.velocity.x;
    let c2 = stone_b.position.x * a2 + stone_b.position.y * b2;

    let determinant = a1 * b2 - a2 * b1;
    if determinant == 0.0 {
        // Lines are parallel.
        return None;
    }

    Some(Point::new(
        (b2 * c1 - b1 * c2) / determinant,
        (a1 * c2 - a2 * c1) / determinant,
        0.0,
    ))
}

macro_rules! in_past {
    ($vel:expr, $cur:expr, $int:expr) => {
        (if $vel < 0.0 { $int > $cur } else { $int < $cur })
    };
}

fn count_intersections(hailstones: &[Hailstone], range: Range<f64>) -> usize {
    let mut count = 0;
    for (i, stone_a) in hailstones.iter().enumerate() {
        for stone_b in hailstones.iter().skip(i + 1) {
            let Some(intersection) = calculate_path_intersection_xy(stone_a, stone_b) else {
                continue;
            };
            if range.contains(&intersection.x) && range.contains(&intersection.y) {
                if in_past!(stone_a.velocity.x, stone_a.position.x, intersection.x)
                    || in_past!(stone_a.velocity.y, stone_a.position.y, intersection.y)
                    || in_past!(stone_b.velocity.x, stone_b.position.x, intersection.x)
                    || in_past!(stone_b.velocity.y, stone_b.position.y, intersection.y)
                {
                    // In the past.
                    continue;
                }

                count += 1;
            }
        }
    }
    count
}

pub fn part1(input: &str) -> usize {
    let hailstones = parse_input(input);
    count_intersections(&hailstones, 200_000_000_000_000f64..400_000_000_000_000f64)
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input]
    static EXAMPLE_INPUT: &str = "
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Hailstone {
                position: Point::new(19.0, 13.0, 30.0),
                velocity: Point::new(-2.0, 1.0, -2.0),
            },
            Hailstone {
                position: Point::new(18.0, 19.0, 22.0),
                velocity: Point::new(-1.0, -1.0, -2.0),
            },
            Hailstone {
                position: Point::new(20.0, 25.0, 34.0),
                velocity: Point::new(-2.0, -2.0, -4.0),
            },
            Hailstone {
                position: Point::new(12.0, 31.0, 28.0),
                velocity: Point::new(-1.0, -2.0, -1.0),
            },
            Hailstone {
                position: Point::new(20.0, 19.0, 15.0),
                velocity: Point::new(1.0, -5.0, -3.0),
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_count_intersections() {
        assert_eq!(
            count_intersections(&parse_input(&EXAMPLE_INPUT), 7f64..27f64),
            2
        );
    }

    macro_rules! assert_eq_approx {
        ($actual:expr, $expected:expr $(,)?) => {{
            let actual: Option<Point> = $actual;
            let expected: Option<Point> = $expected;
            match (actual, expected) {
                (Some(actual), Some(expected)) => {
                    assert!(
                        (actual.x - expected.x).abs() < 0.005
                            && (actual.y - expected.y).abs() < 0.005,
                        "expected point {:?} to match {:?}",
                        actual,
                        expected,
                    );
                }
                _ => assert_eq!(actual, expected),
            }
        }};
    }

    const EXAMPLE_STONE_1: Hailstone = Hailstone {
        position: Point {
            x: 19.0,
            y: 13.0,
            z: 30.0,
        },
        velocity: Point {
            x: -2.0,
            y: 1.0,
            z: -2.0,
        },
    };
    const EXAMPLE_STONE_2: Hailstone = Hailstone {
        position: Point {
            x: 18.0,
            y: 19.0,
            z: 22.0,
        },
        velocity: Point {
            x: -1.0,
            y: -1.0,
            z: -2.0,
        },
    };
    const EXAMPLE_STONE_3: Hailstone = Hailstone {
        position: Point {
            x: 20.0,
            y: 25.0,
            z: 34.0,
        },
        velocity: Point {
            x: -2.0,
            y: -2.0,
            z: -4.0,
        },
    };
    const EXAMPLE_STONE_4: Hailstone = Hailstone {
        position: Point {
            x: 12.0,
            y: 31.0,
            z: 28.0,
        },
        velocity: Point {
            x: -1.0,
            y: -2.0,
            z: -1.0,
        },
    };
    const EXAMPLE_STONE_5: Hailstone = Hailstone {
        position: Point {
            x: 20.0,
            y: 19.0,
            z: 15.0,
        },
        velocity: Point {
            x: 1.0,
            y: -5.0,
            z: -3.0,
        },
    };

    #[test]
    fn calculate_path_intersection_xy_1_2() {
        assert_eq_approx!(
            super::calculate_path_intersection_xy(&EXAMPLE_STONE_1, &EXAMPLE_STONE_2),
            Some(Point::new(14.333, 15.333, 0.0)),
        );
    }

    #[test]
    fn calculate_path_intersection_xy_1_3() {
        assert_eq_approx!(
            super::calculate_path_intersection_xy(&EXAMPLE_STONE_1, &EXAMPLE_STONE_3),
            Some(Point::new(11.667, 16.667, 0.0)),
        );
    }

    #[test]
    fn calculate_path_intersection_xy_1_4() {
        assert_eq_approx!(
            super::calculate_path_intersection_xy(&EXAMPLE_STONE_1, &EXAMPLE_STONE_4),
            Some(Point::new(6.2, 19.4, 0.0)),
        );
    }

    #[test]
    fn calculate_path_intersection_xy_1_5() {
        assert_eq_approx!(
            super::calculate_path_intersection_xy(&EXAMPLE_STONE_1, &EXAMPLE_STONE_5),
            Some(Point::new(21.444, 11.777, 0.0)),
        );
    }

    #[test]
    fn calculate_path_intersection_xy_2_3() {
        assert_eq_approx!(
            super::calculate_path_intersection_xy(&EXAMPLE_STONE_2, &EXAMPLE_STONE_3),
            None,
        );
    }

    #[test]
    fn calculate_path_intersection_xy_2_4() {
        assert_eq_approx!(
            super::calculate_path_intersection_xy(&EXAMPLE_STONE_2, &EXAMPLE_STONE_4),
            Some(Point::new(-6.0, -5.0, 0.0)),
        );
    }

    #[test]
    fn calculate_path_intersection_xy_2_5() {
        assert_eq_approx!(
            super::calculate_path_intersection_xy(&EXAMPLE_STONE_2, &EXAMPLE_STONE_5),
            Some(Point::new(19.667, 20.667, 0.0)),
        );
    }

    #[test]
    fn calculate_path_intersection_xy_3_4() {
        assert_eq_approx!(
            super::calculate_path_intersection_xy(&EXAMPLE_STONE_3, &EXAMPLE_STONE_4),
            Some(Point::new(-2.0, 3.0, 0.0)),
        );
    }

    #[test]
    fn calculate_path_intersection_xy_3_5() {
        assert_eq_approx!(
            super::calculate_path_intersection_xy(&EXAMPLE_STONE_3, &EXAMPLE_STONE_5),
            Some(Point::new(19.0, 24.0, 0.0)),
        );
    }

    #[test]
    fn calculate_path_intersection_xy_4_5() {
        assert_eq_approx!(
            super::calculate_path_intersection_xy(&EXAMPLE_STONE_4, &EXAMPLE_STONE_5),
            Some(Point::new(16.0, 39.0, 0.0)),
        );
    }
}
