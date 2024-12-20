use std::collections::{BinaryHeap, HashSet};

use aoc::utils::point::Point2;
use rayon::prelude::*;

type Map = Vec<Vec<bool>>;

fn parse_input(input: &str) -> (Map, Point2, Point2) {
    let mut start = None;
    let mut end = None;
    let map = input
        .split('\n')
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, c)| match c {
                    '#' => false,
                    '.' => true,
                    'S' => {
                        start = Some(Point2::new(x, y));
                        true
                    }
                    'E' => {
                        end = Some(Point2::new(x, y));
                        true
                    }
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    (map, start.unwrap(), end.unwrap())
}

fn make_step_map(map: &Map, start: &Point2) -> Vec<Vec<usize>> {
    let mut paths = BinaryHeap::new();
    let mut visited = HashSet::new();
    paths.push((0, *start));
    let mut result: Vec<Vec<usize>> = map
        .iter()
        .map(|row| row.iter().map(|_| 0).collect())
        .collect();
    while let Some((steps, point)) = paths.pop() {
        if !map[point.y][point.x] || !visited.insert(point) {
            continue;
        }

        result[point.y][point.x] = -steps as usize;

        for neigh in point.neighbours_ortho() {
            paths.push((steps - 1, neigh));
        }
    }
    result
}

fn find_cheat_paths(
    map: &Map,
    start: &Point2,
    end: &Point2,
    cheat: usize,
    min_save: usize,
) -> usize {
    let bounds = Point2::new(map[0].len(), map.len());
    let non_wall_points: Vec<_> = (1..(bounds.x - 1))
        .flat_map(|x| {
            (1..(bounds.y - 1))
                .filter(|y| map[*y][x])
                .map(|y| Point2::new(x, y))
                .collect::<Vec<_>>()
        })
        .collect();
    let from_start = make_step_map(map, start);
    let from_end = make_step_map(map, end);
    let threshold = from_start[end.y][end.x] - min_save;
    non_wall_points
        .par_iter()
        .map(|point1| {
            non_wall_points
                .iter()
                .filter(|point2| {
                    let distance = point1.abs_diff(point2).sum();
                    distance <= cheat
                        && from_start[point1.y][point1.x] + distance + from_end[point2.y][point2.x]
                            <= threshold
                })
                .count()
        })
        .sum()
}

fn part1impl(input: &str, min_save: usize) -> usize {
    let (map, start, end) = parse_input(input);
    find_cheat_paths(&map, &start, &end, 2, min_save)
}

pub fn part1(input: &str) -> usize {
    part1impl(input, 100)
}

fn part2impl(input: &str, min_save: usize) -> usize {
    let (map, start, end) = parse_input(input);
    find_cheat_paths(&map, &start, &end, 20, min_save)
}

pub fn part2(input: &str) -> usize {
    part2impl(input, 100)
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 44, part2 = 285, notest)]
    static EXAMPLE_INPUT: &str = "
        ###############
        #...#...#.....#
        #.#.#.#.#.###.#
        #S#...#.#.#...#
        #######.#.#.###
        #######.#.#...#
        #######.#.###.#
        ###..E#...#...#
        ###.#######.###
        #...###...#...#
        #.#####.#.###.#
        #.#...#.#.#...#
        #.#.#.#.#.#.###
        #...#...#...###
        ###############
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = (
            vec![
                vec![
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
                vec![
                    false, true, true, true, false, true, true, true, false, true, true, true,
                    true, true, false,
                ],
                vec![
                    false, true, false, true, false, true, false, true, false, true, false, false,
                    false, true, false,
                ],
                vec![
                    false, true, false, true, true, true, false, true, false, true, false, true,
                    true, true, false,
                ],
                vec![
                    false, false, false, false, false, false, false, true, false, true, false,
                    true, false, false, false,
                ],
                vec![
                    false, false, false, false, false, false, false, true, false, true, false,
                    true, true, true, false,
                ],
                vec![
                    false, false, false, false, false, false, false, true, false, true, false,
                    false, false, true, false,
                ],
                vec![
                    false, false, false, true, true, true, false, true, true, true, false, true,
                    true, true, false,
                ],
                vec![
                    false, false, false, true, false, false, false, false, false, false, false,
                    true, false, false, false,
                ],
                vec![
                    false, true, true, true, false, false, false, true, true, true, false, true,
                    true, true, false,
                ],
                vec![
                    false, true, false, false, false, false, false, true, false, true, false,
                    false, false, true, false,
                ],
                vec![
                    false, true, false, true, true, true, false, true, false, true, false, true,
                    true, true, false,
                ],
                vec![
                    false, true, false, true, false, true, false, true, false, true, false, true,
                    false, false, false,
                ],
                vec![
                    false, true, true, true, false, true, true, true, false, true, true, true,
                    false, false, false,
                ],
                vec![
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ],
            Point2::new(1, 3),
            Point2::new(5, 7),
        );
        vec![1, 2];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_test_1() {
        let actual = part1impl(&EXAMPLE_INPUT, 1).to_string();
        assert_eq!(actual, EXAMPLE_INPUT.part1.unwrap());
    }

    #[test]
    fn example_test_2() {
        let actual = part2impl(&EXAMPLE_INPUT, 50).to_string();
        assert_eq!(actual, EXAMPLE_INPUT.part2.unwrap());
    }
}
