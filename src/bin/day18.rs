use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use aoc::counter::Counter;
use aoc::runner::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        return (other.0.abs() + other.1.abs() + other.2.abs())
            .cmp(&(self.0.abs() + self.1.abs() + self.2.abs()));
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

enum Air {
    Cooling(HashSet<Point>),
    Bubble(HashSet<Point>),
}

fn check_air<'a>(point: &Point, points: &'a Vec<Point>) -> Air {
    let mut paths = BinaryHeap::new();
    let mut visited = HashSet::new();
    paths.push(point.clone());
    while !paths.is_empty() {
        let point = paths.pop().unwrap();
        if point == Point(0, 0, 0) {
            return Air::Cooling(visited);
        }
        for neighbour in point.neighbours() {
            if points.contains(&neighbour) || visited.contains(&neighbour) {
                continue;
            }
            paths.push(neighbour.clone());
            visited.insert(neighbour);
        }
    }
    return Air::Bubble(visited);
}

fn get_counts(points: &Vec<Point>) -> HashMap<Point, u16> {
    let mut neighbour_counts = HashMap::new();
    for point in points.iter() {
        for neighbour in point.neighbours() {
            neighbour_counts.count(neighbour, 1);
        }
    }
    for point in points.iter() {
        neighbour_counts.remove(point);
    }
    return neighbour_counts;
}

pub fn part1(input: String) -> u16 {
    let points = parse_input(input);
    let neighbour_counts = get_counts(&points);
    return neighbour_counts.into_values().sum();
}

pub fn part2(input: String) -> u16 {
    let points = parse_input(input);
    let mut neighbour_counts = get_counts(&points);
    let mut cooling = 0;
    while !neighbour_counts.is_empty() {
        let point = neighbour_counts.keys().next().unwrap().clone();
        let count = neighbour_counts.remove(&point).unwrap();
        match check_air(&point, &points) {
            Air::Cooling(air_points) => {
                cooling += count;
                for air_point in air_points {
                    cooling += neighbour_counts.remove(&air_point).unwrap_or(0);
                }
            }
            Air::Bubble(bubble_points) => {
                for bubble_point in bubble_points {
                    neighbour_counts.remove(&bubble_point);
                }
            }
        }
    }
    return cooling;
}

fn main() {
    run(part1, part2);
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

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 58);
    }
}
