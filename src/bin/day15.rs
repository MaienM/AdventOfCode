use aoc::grid::Point as BasePoint;
use aoc::runner::*;
use std::collections::HashSet;

type Point = BasePoint<isize>;

#[derive(Debug, Eq, PartialEq)]
struct Sensor {
    point: Point,
    range: isize,
}

fn parse_input(input: String) -> Vec<Sensor> {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            let mut words = line.trim().split(" ");

            let x = words
                .nth(2)
                .unwrap()
                .strip_prefix("x=")
                .unwrap()
                .strip_suffix(',')
                .unwrap()
                .parse()
                .unwrap();
            let y = words
                .next()
                .unwrap()
                .strip_prefix("y=")
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse()
                .unwrap();
            let point = Point::new(x, y);

            let x: isize = words
                .nth(4)
                .unwrap()
                .strip_prefix("x=")
                .unwrap()
                .strip_suffix(',')
                .unwrap()
                .parse()
                .unwrap();
            let y: isize = words
                .next()
                .unwrap()
                .strip_prefix("y=")
                .unwrap()
                .parse()
                .unwrap();
            let range = (point.x - x).abs() + (point.y - y).abs();

            return Sensor { point, range };
        })
        .collect();
}

fn count_known_at_y(sensors: Vec<Sensor>, y: isize) -> usize {
    // let relevant_sensors = sensors
    //     .into_iter()
    //     .filter(|sensor|  <= sensor.range);
    let mut points: HashSet<isize> = HashSet::new();
    for sensor in sensors {
        let size = sensor.range - (sensor.point.y - y).abs();
        if size < 0 {
            continue;
        }
        for x in (sensor.point.x - size)..(sensor.point.x + size) {
            points.insert(x);
        }
    }
    return points.len();
}

pub fn part1(input: String) -> usize {
    let sensors = parse_input(input);
    return count_known_at_y(sensors, 2_000_000);
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
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
        let actual = parse_input(EXAMPLE_INPUT.to_string());
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
        let sensors = parse_input(EXAMPLE_INPUT.to_string());
        assert_eq!(count_known_at_y(sensors, 10), 26);
    }
}