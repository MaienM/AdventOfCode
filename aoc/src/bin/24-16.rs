puzzle_runner::register_chapter!(book = "2024", title = "Reindeer Maze");

use std::collections::{BinaryHeap, HashMap, HashSet};

use puzzle_lib::{
    grid::FullGrid,
    point::{Direction2, Point2},
};

type Grid = FullGrid<bool>;

fn parse_input(input: &str) -> (Grid, Point2, Point2) {
    parse!(input => {
        [grid cells match {
            '#' => (false),
            '.' => (true),
            'S' => index into start => (true),
            'E' => index into end => (true),
        }]
    } => (grid, start, end))
}

fn find_path(grid: &Grid, start: &Point2, end: &Point2) -> usize {
    let mut paths = BinaryHeap::new();
    let mut done = HashSet::with_capacity(grid.width() * grid.height() * 4);
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
        if grid[next] {
            paths.push((score - 1, next, facing));
        }
    }
}

fn make_score_map(
    grid: &Grid,
    start: &Point2,
    directions: &[Direction2],
) -> HashMap<(Point2, Direction2), usize> {
    let mut paths = BinaryHeap::new();
    for direction in directions {
        paths.push((0, *start, *direction));
    }
    let mut result = HashMap::with_capacity(grid.width() * grid.height() * 4);
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
        if grid[next] {
            paths.push((score - 1, next, facing));
        }
    }
    result
}

fn count_tiles_on_best_paths(grid: &Grid, start: &Point2, end: &Point2) -> usize {
    let non_wall_points: Vec<_> = grid
        .iter_pairs()
        .filter_map(|(point, cell)| if *cell { Some(*point) } else { None })
        .collect();
    let from_start = make_score_map(grid, start, &[Direction2::East]);
    let from_end = make_score_map(
        grid,
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

#[register_part]
fn part1(input: &str) -> usize {
    let (grid, start, end) = parse_input(input);
    find_path(&grid, &start, &end)
}

#[register_part]
fn part2(input: &str) -> usize {
    let (grid, start, end) = parse_input(input);
    count_tiles_on_best_paths(&grid, &start, &end)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

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
            [
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
                [
                    false, true, true, true, true, true, true, true, false, true, true, true, true,
                    true, false,
                ],
                [
                    false, true, false, true, false, false, false, true, false, true, false, false,
                    false, true, false,
                ],
                [
                    false, true, true, true, true, true, false, true, false, true, true, true,
                    false, true, false,
                ],
                [
                    false, true, false, false, false, true, false, false, false, false, false,
                    true, false, true, false,
                ],
                [
                    false, true, false, true, false, true, true, true, true, true, true, true,
                    false, true, false,
                ],
                [
                    false, true, false, true, false, false, false, false, false, true, false,
                    false, false, true, false,
                ],
                [
                    false, true, true, true, true, true, true, true, true, true, true, true, false,
                    true, false,
                ],
                [
                    false, false, false, true, false, true, false, false, false, false, false,
                    true, false, true, false,
                ],
                [
                    false, true, true, true, false, true, true, true, true, true, false, true,
                    false, true, false,
                ],
                [
                    false, true, false, true, false, true, false, false, false, true, false, true,
                    false, true, false,
                ],
                [
                    false, true, true, true, true, true, false, true, true, true, false, true,
                    false, true, false,
                ],
                [
                    false, true, false, false, false, true, false, true, false, true, false, true,
                    false, true, false,
                ],
                [
                    false, true, true, true, false, true, true, true, true, true, false, true,
                    true, true, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false,
                ],
            ]
            .into(),
            Point2::new(1, 13),
            Point2::new(13, 1),
        );
        assert_eq!(actual, expected);
    }
}
