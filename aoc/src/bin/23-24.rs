puzzle_runner::register_chapter!(book = 2023, title = "Never Tell Me The Odds");

use std::ops::Range;

use puzzle_lib::{matrix::Matrix, point::Point3};

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

#[register_part]
fn part1(input: &str) -> usize {
    let hailstones = parse_input(input);
    count_intersections(&hailstones, 200_000_000_000_000f64..400_000_000_000_000f64)
}

#[register_part]
fn part2(input: &str) -> usize {
    let hailstones = parse_input(input);

    // Let Xn, Yn, Zn be the starting position of stone N, and XVn, YVn and ZVn be the velocity of the same stone. Let N = 0 for the thrown stone, and N = 1.. for the hailstones from the input file. Let Tn be the time when stone N intersects with the thrown stone.
    //
    // This gives us the following equations:
    //
    // X1 + XV1T1 = X0 + XV0T1
    // Y1 + YV1T1 = Y0 + YV0T1
    //
    // Rewrite to an equation for T1:
    //
    // X1 - X0 + T1(XV1 - XV0)
    // T1(XV1 - XV0) = X0 - X1
    // T1 = (X0 - X1) / (XV1 - XV0)
    //
    // We can do the same for Y, yielding two different equations for T1. We can combine these to eliminate T1 from the equations entirely:
    //
    // (X0 - X1) / (XV1 - XV0) = (Y0 - Y1) / (YV1 - YV0)
    // (X0 - X1)(YV1 - YV0) - (Y0 - Y1)(XV1 - XV0) = 0
    //
    // We can do the same thing for N = 2, and we can combine these. This results in a number of quadratic unknowns appearing on both sides allowing us to eliminate these.
    //
    // (X0 - X1)(YV1 - YV0) - (Y0 - Y1)(XV1 - XV0) = (X0 - X2)(YV2 - YV0) - (Y0 - Y2)(XV2 - XV0)
    // X0YV1 - X0YV0 - X1YV1 + X1YV0 - Y0XV1 + Y0XV0 + Y1XV1 - Y1XV0 = X0YV2 - X0YV0 - X2YV2 + X2YV0 - Y0XV2 + Y0XV0 + Y2XV2 - Y2XV0
    // X0YV1 - X1YV1 + X1YV0 - Y0XV1 + Y1XV1 - Y1XV0 = X0YV2 - X2YV2 + X2YV0 - Y0XV2 + Y2XV2 - Y2XV0
    //
    // This is now a linear equation. Lets rewrite it a bit further to be easier to put into a matrix:
    //
    // X0YV1 - X1YV1 + X1YV0 - Y0XV1 + Y1XV1 - Y1XV0 - X0YV2 + X2YV2 - X2YV0 + Y0XV2 - Y2XV2 + Y2XV0 = 0
    // X0(YV1 - YV2) + Y0(XV2 - XV1) + XV0(Y2 - Y1) + YV0(X1 - X2) = X1YV1 - Y1XV1 - X2YV2 + Y2XV2
    //
    // This gives us 4 unknowns (X0, Y0, XV0, YV0) with known factors, using (X, Y) of hailstones (1, 2). We can do the same for (X, Z) and (Y, Z) for the same hailstones, and then also for hailstones (1, 3). This gives us 6 linear equations for the 6 unknowns (X0, Y0, Z0, XV0, YV0, ZV0), which should be sufficient. We'll put these equations into an augmented matrix [X0, Y0, Z0, XV0, YV0, ZV0, C] and use Gauss-Jordan elimination to solve them.

    let matrix = Matrix::new([
        // (X, Y) for (1, 2).
        [
            hailstones[0].velocity.y - hailstones[1].velocity.y,
            hailstones[1].velocity.x - hailstones[0].velocity.x,
            0.0,
            hailstones[1].position.y - hailstones[0].position.y,
            hailstones[0].position.x - hailstones[1].position.x,
            0.0,
            hailstones[0].position.x * hailstones[0].velocity.y
                - hailstones[0].position.y * hailstones[0].velocity.x
                - hailstones[1].position.x * hailstones[1].velocity.y
                + hailstones[1].position.y * hailstones[1].velocity.x,
        ],
        // (X, Z) for (1, 2).
        [
            hailstones[0].velocity.z - hailstones[1].velocity.z,
            0.0,
            hailstones[1].velocity.x - hailstones[0].velocity.x,
            hailstones[1].position.z - hailstones[0].position.z,
            0.0,
            hailstones[0].position.x - hailstones[1].position.x,
            hailstones[0].position.x * hailstones[0].velocity.z
                - hailstones[0].position.z * hailstones[0].velocity.x
                - hailstones[1].position.x * hailstones[1].velocity.z
                + hailstones[1].position.z * hailstones[1].velocity.x,
        ],
        // (Y, Z) for (1, 2).
        [
            0.0,
            hailstones[0].velocity.z - hailstones[1].velocity.z,
            hailstones[1].velocity.y - hailstones[0].velocity.y,
            0.0,
            hailstones[1].position.z - hailstones[0].position.z,
            hailstones[0].position.y - hailstones[1].position.y,
            hailstones[0].position.y * hailstones[0].velocity.z
                - hailstones[0].position.z * hailstones[0].velocity.y
                - hailstones[1].position.y * hailstones[1].velocity.z
                + hailstones[1].position.z * hailstones[1].velocity.y,
        ],
        // (X, Y) for (1, 3).
        [
            hailstones[0].velocity.y - hailstones[2].velocity.y,
            hailstones[2].velocity.x - hailstones[0].velocity.x,
            0.0,
            hailstones[2].position.y - hailstones[0].position.y,
            hailstones[0].position.x - hailstones[2].position.x,
            0.0,
            hailstones[0].position.x * hailstones[0].velocity.y
                - hailstones[0].position.y * hailstones[0].velocity.x
                - hailstones[2].position.x * hailstones[2].velocity.y
                + hailstones[2].position.y * hailstones[2].velocity.x,
        ],
        // (X, Z) for (1, 3).
        [
            hailstones[0].velocity.z - hailstones[2].velocity.z,
            0.0,
            hailstones[2].velocity.x - hailstones[0].velocity.x,
            hailstones[2].position.z - hailstones[0].position.z,
            0.0,
            hailstones[0].position.x - hailstones[2].position.x,
            hailstones[0].position.x * hailstones[0].velocity.z
                - hailstones[0].position.z * hailstones[0].velocity.x
                - hailstones[2].position.x * hailstones[2].velocity.z
                + hailstones[2].position.z * hailstones[2].velocity.x,
        ],
        // (Y, Z) for (1, 3).
        [
            0.0,
            hailstones[0].velocity.z - hailstones[2].velocity.z,
            hailstones[2].velocity.y - hailstones[0].velocity.y,
            0.0,
            hailstones[2].position.z - hailstones[0].position.z,
            hailstones[0].position.y - hailstones[2].position.y,
            hailstones[0].position.y * hailstones[0].velocity.z
                - hailstones[0].position.z * hailstones[0].velocity.y
                - hailstones[2].position.y * hailstones[2].velocity.z
                + hailstones[2].position.z * hailstones[2].velocity.y,
        ],
    ]);
    let result = matrix.gauss_jordan_elimination().unwrap();
    let stone = Hailstone {
        position: Point::new(result[0], result[1], result[2]),
        velocity: Point::new(result[3], result[4], result[5]),
    };

    stone.position.x.round() as usize
        + stone.position.y.round() as usize
        + stone.position.z.round() as usize
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

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
