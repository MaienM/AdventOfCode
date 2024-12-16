use std::collections::{BinaryHeap, HashSet};

use aoc::utils::point::{Direction2, Point2};

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
    let mut done = HashSet::new();
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

pub fn part1(input: &str) -> usize {
    let (map, start, end) = parse_input(input);
    find_path(&map, &start, &end)
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 7036)]
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

    #[example_input(part1 = 11_048)]
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

    // #[test]
    // fn example_parse() {
    //     let actual = parse_input(&EXAMPLE_INPUT);
    //     let expected = vec![1, 2];
    //     assert_eq!(actual, expected);
    // }
}
