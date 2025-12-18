puzzle_runner::register_chapter!(title = "Boiling Boulders");

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point(i8, i8, i8);
impl Point {
    pub fn neighbours(&self) -> [Self; 6] {
        [
            Point(self.0 + 1, self.1, self.2),
            Point(self.0 - 1, self.1, self.2),
            Point(self.0, self.1 + 1, self.2),
            Point(self.0, self.1 - 1, self.2),
            Point(self.0, self.1, self.2 + 1),
            Point(self.0, self.1, self.2 - 1),
        ]
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.0.abs() + other.1.abs() + other.2.abs())
            .cmp(&(self.0.abs() + self.1.abs() + self.2.abs()))
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    parse!(input => {
        [points split on '\n' with 
            { [coords split on ',' as i8] }
            => Point(coords[0], coords[1], coords[2])
        ]
    } => points)
}

enum Air {
    Cooling(HashSet<Point>),
    Bubble(HashSet<Point>),
}

fn check_air(point: &Point, points: &[Point]) -> Air {
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
    Air::Bubble(visited)
}

fn get_counts(points: &[Point]) -> HashMap<Point, u16> {
    let mut neighbour_counts = HashMap::new();
    for point in points {
        for neighbour in point.neighbours() {
            neighbour_counts.increment_one(neighbour);
        }
    }
    for point in points {
        neighbour_counts.remove(point);
    }
    neighbour_counts
}

#[register_part]
fn part1(input: &str) -> u16 {
    let points = parse_input(input);
    let neighbour_counts = get_counts(&points);
    neighbour_counts.into_values().sum()
}

#[register_part]
fn part2(input: &str) -> u16 {
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
    cooling
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 64, part2 = 58)]
    static EXAMPLE_INPUT: &str = "
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
        let actual = parse_input(&EXAMPLE_INPUT);
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
}
