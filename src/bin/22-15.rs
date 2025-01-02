use std::{iter, ops::Range};

use aoc::utils::{parse, point::Point2};

type Point = Point2<isize>;

#[derive(Debug, Eq, PartialEq)]
struct Sensor {
    point: Point,
    range: isize,
}

fn distance(left: &Point, right: &Point) -> isize {
    (left.x - right.x).abs() + (left.y - right.y).abs()
}

fn parse_input(input: &str) -> Vec<Sensor> {
    parse!(input => {
        [lines split on '\n' with
            { "Sensor at x=" [sx as isize] ", y=" [sy as isize] ": closest beacon is at x=" [bx as isize] ", y=" [by as isize] }
            => {
                let point = Point::new(sx, sy);
                let beacon = Point::new(bx, by);
                let range = distance(&point, &beacon);
                Sensor { point, range }
            }
        ]
    } => lines)
}

fn ranges_overlap(left: &Range<isize>, right: &Range<isize>) -> bool {
    left.contains(&right.start)
        || left.contains(&right.end)
        || right.contains(&left.start)
        || right.contains(&left.end)
}

fn count_known_at_y(sensors: Vec<Sensor>, y: isize) -> usize {
    let mut ranges: Vec<Range<isize>> = Vec::new();
    for sensor in sensors {
        let size = sensor.range - (sensor.point.y - y).abs();
        if size < 0 {
            continue;
        }

        let mut range = (sensor.point.x - size)..(sensor.point.x + size);
        let (overlapping, other): (Vec<Range<isize>>, Vec<Range<isize>>) =
            ranges.into_iter().partition(|r| ranges_overlap(&range, r));
        ranges = other;
        if !overlapping.is_empty() {
            // Range overlaps with one or more existing ranges, so merge them all into a single range.
            let start = overlapping
                .iter()
                .chain(iter::once(&range))
                .map(|r| r.start)
                .min()
                .unwrap();
            let end = overlapping
                .iter()
                .chain(iter::once(&range))
                .map(|r| r.end)
                .max()
                .unwrap();
            range = start..end;
        }

        ranges.push(range);
    }
    ranges.into_iter().map(|r| r.len()).sum()
}

pub fn part1(input: &str) -> usize {
    let sensors = parse_input(input);
    count_known_at_y(sensors, 2_000_000)
}

fn get_beacon(sensors: &Vec<Sensor>, range: isize) -> Point {
    for sensor in sensors {
        // Consider all points that are _just_ outside the range of this sensor.
        for x in (sensor.point.x - sensor.range - 1).max(0)
            ..=(sensor.point.x + sensor.range + 1).min(range)
        {
            let yoffset = sensor.range - (sensor.point.x - x).abs() + 1;
            let points = [
                Point::new(x, sensor.point.y - yoffset),
                Point::new(x, sensor.point.y + yoffset),
            ];
            'points: for point in points {
                if point.y < 0 || point.y > range {
                    continue;
                }
                for sensor in sensors {
                    if distance(&sensor.point, &point) <= sensor.range {
                        continue 'points;
                    }
                }
                return point;
            }
        }
    }
    Point::new(0, 0)
}

pub fn part2(input: &str) -> isize {
    let sensors = parse_input(input);
    let point = get_beacon(&sensors, 4_000_000);
    point.x * 4_000_000 + point.y
}

aoc_runner::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input]
    static EXAMPLE_INPUT: &str = "
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Sensor {
                point: Point::new(2, 18),
                range: 7,
            },
            Sensor {
                point: Point::new(9, 16),
                range: 1,
            },
            Sensor {
                point: Point::new(13, 2),
                range: 3,
            },
            Sensor {
                point: Point::new(12, 14),
                range: 4,
            },
            Sensor {
                point: Point::new(10, 20),
                range: 4,
            },
            Sensor {
                point: Point::new(14, 17),
                range: 5,
            },
            Sensor {
                point: Point::new(8, 7),
                range: 9,
            },
            Sensor {
                point: Point::new(2, 0),
                range: 10,
            },
            Sensor {
                point: Point::new(0, 11),
                range: 3,
            },
            Sensor {
                point: Point::new(20, 14),
                range: 8,
            },
            Sensor {
                point: Point::new(17, 20),
                range: 6,
            },
            Sensor {
                point: Point::new(16, 7),
                range: 5,
            },
            Sensor {
                point: Point::new(14, 3),
                range: 1,
            },
            Sensor {
                point: Point::new(20, 1),
                range: 7,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_count_known_at_y() {
        let sensors = parse_input(&EXAMPLE_INPUT);
        assert_eq!(count_known_at_y(sensors, 10), 26);
    }

    #[test]
    fn example_get_beacon() {
        let sensors = parse_input(&EXAMPLE_INPUT);
        assert_eq!(get_beacon(&sensors, 20), Point::new(14, 11));
    }
}
