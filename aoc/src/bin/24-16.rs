aoc::setup!(title = "Reindeer Maze");

use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc::point::{Direction2, Point2};
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

fn find_path(map: &Map, start: &Point2, end: &Point2) -> usize {
    let mut paths = BinaryHeap::new();
    let mut done = HashSet::with_capacity(map[0].len() * map.len() * 4);
    paths.push((0, *start, Direction2::East));
    loop {
        let (score, point, facing) = paths.pop().unwrap();

        if point == *end {
            return -score as usize;
        }

        if !done.insert((point, facing)) {
            continue;
        }

        match facing {
            Direction2::North | Direction2::South => {
                paths.push((score - 1000, point, Direction2::East));
                paths.push((score - 1000, point, Direction2::West));
            }
            Direction2::East | Direction2::West => {
                paths.push((score - 1000, point, Direction2::North));
                paths.push((score - 1000, point, Direction2::South));
            }
        }

        let next = point + facing;
        if map[next.y][next.x] {
            paths.push((score - 1, next, facing));
        }
    }
}

fn make_score_map(
    map: &Map,
    start: &Point2,
    directions: &[Direction2],
) -> HashMap<(Point2, Direction2), usize> {
    let mut paths = BinaryHeap::new();
    for direction in directions {
        paths.push((0, *start, *direction));
    }
    let mut result = HashMap::with_capacity(map[0].len() * map.len() * 4);
    while let Some((score, point, facing)) = paths.pop() {
        if result.contains_key(&(point, facing)) {
            continue;
        }
        result.insert((point, facing), -score as usize);

        match facing {
            Direction2::North | Direction2::South => {
                paths.push((score - 1000, point, Direction2::East));
                paths.push((score - 1000, point, Direction2::West));
            }
            Direction2::East | Direction2::West => {
                paths.push((score - 1000, point, Direction2::North));
                paths.push((score - 1000, point, Direction2::South));
            }
        }

        let next = point + facing;
        if map[next.y][next.x] {
            paths.push((score - 1, next, facing));
        }
    }
    result
}

fn count_tiles_on_best_paths(map: &Map, start: &Point2, end: &Point2) -> usize {
    let bounds = Point2::new(map[0].len(), map.len());
    let non_wall_points: Vec<_> = (1..(bounds.x - 1))
        .flat_map(|x| {
            (1..(bounds.y - 1))
                .filter(|y| map[*y][x])
                .map(|y| Point2::new(x, y))
                .collect::<Vec<_>>()
        })
        .collect();
    let from_start = make_score_map(map, start, &[Direction2::East]);
    let from_end = make_score_map(
        map,
        end,
        &[
            Direction2::North,
            Direction2::East,
            Direction2::South,
            Direction2::West,
        ],
    );
    let best = from_end[&(*start, Direction2::West)];
    non_wall_points
        .into_par_iter()
        .filter(|p| {
            from_start[&(*p, Direction2::North)] + from_end[&(*p, Direction2::South)] == best
                || from_start[&(*p, Direction2::East)] + from_end[&(*p, Direction2::West)] == best
                || from_start[&(*p, Direction2::South)] + from_end[&(*p, Direction2::North)] == best
                || from_start[&(*p, Direction2::West)] + from_end[&(*p, Direction2::East)] == best
        })
        .count()
}

pub fn part1(input: &str) -> usize {
    let (map, start, end) = parse_input(input);
    find_path(&map, &start, &end)
}

pub fn part2(input: &str) -> usize {
    let (map, start, end) = parse_input(input);
    count_tiles_on_best_paths(&map, &start, &end)
}

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 7036, part2 = 45)]
    static EXAMPLE_INPUT_1: &str = "
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    ";

    #[example_input(part1 = 11_048, part2 = 64)]
    static EXAMPLE_INPUT_2: &str = "
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = (
            vec![
                vec![
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
                vec![
                    false, true, true, true, true, true, true, true, false, true, true, true, true,
                    true, false,
                ],
                vec![
                    false, true, false, true, false, false, false, true, false, true, false, false,
                    false, true, false,
                ],
                vec![
                    false, true, true, true, true, true, false, true, false, true, true, true,
                    false, true, false,
                ],
                vec![
                    false, true, false, false, false, true, false, false, false, false, false,
                    true, false, true, false,
                ],
                vec![
                    false, true, false, true, false, true, true, true, true, true, true, true,
                    false, true, false,
                ],
                vec![
                    false, true, false, true, false, false, false, false, false, true, false,
                    false, false, true, false,
                ],
                vec![
                    false, true, true, true, true, true, true, true, true, true, true, true, false,
                    true, false,
                ],
                vec![
                    false, false, false, true, false, true, false, false, false, false, false,
                    true, false, true, false,
                ],
                vec![
                    false, true, true, true, false, true, true, true, true, true, false, true,
                    false, true, false,
                ],
                vec![
                    false, true, false, true, false, true, false, false, false, true, false, true,
                    false, true, false,
                ],
                vec![
                    false, true, true, true, true, true, false, true, true, true, false, true,
                    false, true, false,
                ],
                vec![
                    false, true, false, false, false, true, false, true, false, true, false, true,
                    false, true, false,
                ],
                vec![
                    false, true, true, true, false, true, true, true, true, true, false, true,
                    true, true, false,
                ],
                vec![
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ],
            Point2::new(1, 13),
            Point2::new(13, 1),
        );
        assert_eq!(actual, expected);
    }
}
