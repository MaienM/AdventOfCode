use std::collections::{HashMap, HashSet};

use aoc::utils::point::Point2;

type Point = Point2<isize>;

fn parse_input(input: &str) -> (HashMap<char, Vec<Point>>, Point) {
    let mut map: HashMap<char, Vec<Point>> = HashMap::new();
    let nodes = input.split('\n').enumerate().flat_map(|(y, line)| {
        line.char_indices()
            .filter(|(_, c)| *c != '.')
            .map(|(x, c)| (c, Point::new(x as isize, y as isize)))
            .collect::<Vec<_>>()
    });
    let bounds = Point::new(
        input.split('\n').next().unwrap().len() as isize,
        input.split('\n').count() as isize,
    );
    for (char, point) in nodes {
        map.entry(char).or_default().push(point);
    }
    (map, bounds)
}

pub fn part1(input: &str) -> usize {
    let (map, bounds) = parse_input(input);
    let mut antinodes = HashSet::new();
    for nodes in map.into_values() {
        for node1 in &nodes {
            for node2 in &nodes {
                if node1 == node2 {
                    continue;
                }
                let diff = *node1 - *node2;
                antinodes.insert(*node1 + diff);
                antinodes.insert(*node2 - diff);
            }
        }
    }
    antinodes
        .into_iter()
        .filter(|an| 0 <= an.x && an.x < bounds.x && 0 <= an.y && an.y < bounds.y)
        .count()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 14)]
    static EXAMPLE_INPUT: &str = "
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = (
            hash_map![
                '0' => vec![
                    Point::new(8, 1),
                    Point::new(5, 2),
                    Point::new(7, 3),
                    Point::new(4, 4),
                ],
                'A' => vec![
                    Point::new(6, 5),
                    Point::new(8, 8),
                    Point::new(9, 9),
                ],
            ],
            Point::new(12, 12),
        );
        assert_eq!(actual, expected);
    }
}
