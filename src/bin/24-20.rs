use std::collections::{BinaryHeap, HashSet};

use aoc::utils::point::{Direction2, Point2};
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
    let mut done = HashSet::new();
    paths.push((0, *start));
    loop {
        let (score, point) = paths.pop().unwrap();

        if point == *end {
            return -score as usize;
        }

        if !done.insert(point) || !map[point.y][point.x] {
            continue;
        }

        paths.push((score - 1, point + Direction2::East));
        paths.push((score - 1, point + Direction2::West));
        paths.push((score - 1, point + Direction2::North));
        paths.push((score - 1, point + Direction2::South));
    }
}

pub fn part1(input: &str) -> usize {
    let (map, start, end) = parse_input(input);
    let bounds = Point2::new(map[0].len(), map.len());
    let base = find_path(&map, &start, &end);
    (1..(bounds.x - 1))
        .into_par_iter()
        .map(|x| {
            (1..(bounds.y - 1))
                .into_par_iter()
                .filter(|y| {
                    let y = *y;
                    if !map[y][x]
                        && ((map[y - 1][x] && map[y + 1][x]) || (map[y][x - 1] && map[y][x + 1]))
                    {
                        let mut map = map.clone();
                        map[y][x] = true;
                        let with_cheating = find_path(&map, &start, &end);
                        (base - with_cheating) >= 100
                    } else {
                        false
                    }
                })
                .count()
        })
        .sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 1, notest)]
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

    // #[test]
    // fn example_parse() {
    //     let actual = parse_input(&EXAMPLE_INPUT);
    //     let expected = vec![1, 2];
    //     assert_eq!(actual, expected);
    // }
}
