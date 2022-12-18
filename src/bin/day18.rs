use aoc::counter::Counter;
use aoc::runner::*;
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point(i8, i8, i8);
impl Point {
    pub fn neighbours(&self) -> [Self; 6] {
        return [
            Point(self.0 + 1, self.1, self.2),
            Point(self.0 - 1, self.1, self.2),
            Point(self.0, self.1 + 1, self.2),
            Point(self.0, self.1 - 1, self.2),
            Point(self.0, self.1, self.2 + 1),
            Point(self.0, self.1, self.2 - 1),
        ];
    }
}

fn parse_input(input: String) -> Vec<Point> {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            let mut parts = line
                .trim()
                .splitn(3, ',')
                .map(str::parse)
                .map(Result::unwrap);
            return Point(
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            );
        })
        .collect();
}

pub fn part1(input: String) -> usize {
    let points = parse_input(input);
    let mut neighbour_counts = HashMap::new();
    for point in points.iter() {
        for neighbour in point.neighbours() {
            neighbour_counts.count(neighbour, 1);
        }
    }
    for point in points.iter() {
        neighbour_counts.remove(point);
    }
    return neighbour_counts.into_values().sum();
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![
            Point(2, 2, 2),
            Point(1, 2, 2),
            Point(3, 2, 2),
            Point(2, 1, 2),
            Point(2, 3, 2),
            Point(2, 2, 1),
            Point(2, 2, 3),
            Point(2, 2, 4),
            Point(2, 2, 6),
            Point(1, 2, 5),
            Point(3, 2, 5),
            Point(2, 1, 5),
            Point(2, 3, 5),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 64);
    }
}
