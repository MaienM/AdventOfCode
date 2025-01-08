puzzle_lib::setup!(title = "Hoof It");

use std::collections::HashSet;

use puzzle_lib::point::Point2;

type Map = Vec<Vec<u8>>;

fn parse_height(input: char) -> u8 {
    if input == '.' {
        10
    } else {
        input.to_digit(10).unwrap() as u8
    }
}

fn parse_input(input: &str) -> (Map, Point2) {
    parse!(input => [map split on '\n' with [chars with parse_height]]);
    let bounds = Point2::new(map[0].len(), map.len());
    (map, bounds)
}

fn find_reachable_summits(
    map: &Map,
    current: &Point2,
    target_height: u8,
    found: &mut HashSet<Point2>,
) {
    if target_height == 10 {
        found.insert(*current);
        return;
    }

    for point in current.neighbours_ortho() {
        let Some(height) = map.get(point.y).and_then(|r| r.get(point.x)) else {
            continue;
        };
        if *height == target_height {
            find_reachable_summits(map, &point, target_height + 1, found);
        }
    }
}

fn find_trails_to_summits(map: &Map, current: &Point2, target_height: u8) -> usize {
    if target_height == 10 {
        return 1;
    }

    current
        .neighbours_ortho()
        .iter()
        .map(|point| {
            let Some(height) = map.get(point.y).and_then(|r| r.get(point.x)) else {
                return 0;
            };
            if *height == target_height {
                find_trails_to_summits(map, point, target_height + 1)
            } else {
                0
            }
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    let (map, bounds) = parse_input(input);
    (0..bounds.y)
        .flat_map(|y| (0..bounds.x).map(|x| Point2::new(x, y)).collect::<Vec<_>>())
        .filter(|start| map[start.y][start.x] == 0)
        .par_bridge()
        .map(|start| {
            let mut found = HashSet::new();
            find_reachable_summits(&map, &start, 1, &mut found);
            found.len()
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (map, bounds) = parse_input(input);
    (0..bounds.y)
        .flat_map(|y| (0..bounds.x).map(|x| Point2::new(x, y)).collect::<Vec<_>>())
        .filter(|start| map[start.y][start.x] == 0)
        .par_bridge()
        .map(|start| find_trails_to_summits(&map, &start, 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 2)]
    static EXAMPLE_INPUT_1_1: &str = "
        ...0...
        ...1...
        ...2...
        6543456
        7.....7
        8.....8
        9.....9
    ";

    #[example_input(part1 = 4)]
    static EXAMPLE_INPUT_1_2: &str = "
        ..90..9
        ...1.98
        ...2..7
        6543456
        765.987
        876....
        987....
    ";

    #[example_input(part1 = 3)]
    static EXAMPLE_INPUT_1_3: &str = "
        10..9..
        2...8..
        3...7..
        4567654
        ...8..3
        ...9..2
        .....01
    ";

    #[example_input(part2 = 3)]
    static EXAMPLE_INPUT_2_1: &str = "
        .....0.
        ..4321.
        ..5..2.
        ..6543.
        ..7..4.
        ..8765.
        ..9....
    ";

    #[example_input(part2 = 13)]
    static EXAMPLE_INPUT_2_2: &str = "
        ..90..9
        ...1.98
        ...2..7
        6543456
        765.987
        876....
        987....
    ";

    #[example_input(part2 = 227)]
    static EXAMPLE_INPUT_2_3: &str = "
        012345
        123456
        234567
        345678
        4.6789
        56789.
    ";

    #[example_input(part1 = 36, part2 = 81)]
    static EXAMPLE_INPUT: &str = "
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = (
            vec![
                vec![8, 9, 0, 1, 0, 1, 2, 3],
                vec![7, 8, 1, 2, 1, 8, 7, 4],
                vec![8, 7, 4, 3, 0, 9, 6, 5],
                vec![9, 6, 5, 4, 9, 8, 7, 4],
                vec![4, 5, 6, 7, 8, 9, 0, 3],
                vec![3, 2, 0, 1, 9, 0, 1, 2],
                vec![0, 1, 3, 2, 9, 8, 0, 1],
                vec![1, 0, 4, 5, 6, 7, 3, 2],
            ],
            Point2::new(8, 8),
        );
        assert_eq!(actual, expected);
    }
}
