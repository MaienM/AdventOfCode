puzzle_runner::register_chapter!(title = "Resonant Collinearity");

use std::collections::HashMap;

use puzzle_lib::{
    grid::{PointCollectionInsertResult, SparsePointSet},
    point::Point2,
};

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
        input.split('\n').next().unwrap().len() as isize - 1,
        input.split('\n').count() as isize - 1,
    );
    for (char, point) in nodes {
        map.entry(char).or_default().push(point);
    }
    (map, bounds)
}

#[register_part]
fn part1(input: &str) -> usize {
    let (map, bounds) = parse_input(input);
    let mut antinodes = SparsePointSet::default()
        .with_boundaries((Point2::new(0, 0)..=bounds).into())
        .unwrap();
    for nodes in map.into_values() {
        for (node1, node2) in nodes.into_iter().tuple_combinations() {
            let diff = node1 - node2;
            antinodes.insert(node1 + diff);
            antinodes.insert(node2 - diff);
        }
    }
    antinodes.iter_points().count()
}

#[register_part]
fn part2(input: &str) -> usize {
    let (map, bounds) = parse_input(input);
    let mut antinodes = SparsePointSet::default()
        .with_boundaries((Point2::new(0, 0)..=bounds).into())
        .unwrap();
    for nodes in map.into_values() {
        for (node1, node2) in nodes.into_iter().tuple_combinations() {
            let diff = node1 - node2;

            let mut node = node1;
            while antinodes.insert(node) != PointCollectionInsertResult::OutOfBounds {
                node += diff;
            }

            let mut node = node2;
            while antinodes.insert(node) != PointCollectionInsertResult::OutOfBounds {
                node -= diff;
            }
        }
    }
    antinodes.iter_points().count()
}

#[cfg(test)]
mod tests {
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 14, part2 = 34)]
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

    #[example_input(part2 = 9)]
    static EXAMPLE_INPUT_2: &str = "
        T.........
        ...T......
        .T........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
        ..........
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
            Point::new(11, 11),
        );
        assert_eq!(actual, expected);
    }
}
