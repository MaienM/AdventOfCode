puzzle_runner::register_chapter!(title = "Parabolic Reflector Dish");

use std::{
    collections::HashMap,
    hash::{BuildHasher, RandomState},
};

use puzzle_lib::{grid::FullGrid, point::Point2};

#[derive(Debug, Hash, PartialEq)]
enum Cell {
    RoundRock,
    CubeRock,
    Empty,
}

type Grid = FullGrid<Cell>;

fn parse_input(input: &str) -> Grid {
    parse!(input => { [grid cells match {
        'O' => Cell::RoundRock,
        '#' => Cell::CubeRock,
        '.' => Cell::Empty,
    }] } => grid)
}

fn grid_set_range<I>(grid: &mut Grid, points: I)
where
    I: Iterator<Item = Point2<usize>>,
{
    for (_, cell) in grid.get_many_mut(points) {
        *cell.unwrap() = Cell::RoundRock;
    }
}

fn slide_north(grid: &mut Grid) {
    for x in 0..grid.width() {
        let mut rolling = 0;
        for y in (0..grid.height()).rev() {
            match grid[Point2::new(x, y)] {
                Cell::RoundRock => {
                    rolling += 1;
                    grid[Point2::new(x, y)] = Cell::Empty;
                }
                Cell::CubeRock => {
                    grid_set_range(grid, (0..rolling).map(|i| Point2::new(x, y + i + 1)));
                    rolling = 0;
                }
                Cell::Empty => {}
            }
        }
        grid_set_range(grid, (0..rolling).map(|i| Point2::new(x, i)));
    }
}

fn slide_south(grid: &mut Grid) {
    for x in 0..grid.width() {
        let mut rolling = 0;
        for y in 0..grid.height() {
            match grid[Point2::new(x, y)] {
                Cell::RoundRock => {
                    rolling += 1;
                    grid[Point2::new(x, y)] = Cell::Empty;
                }
                Cell::CubeRock => {
                    grid_set_range(grid, (0..rolling).map(|i| Point2::new(x, y - i - 1)));
                    rolling = 0;
                }
                Cell::Empty => {}
            }
        }
        let height = grid.height();
        grid_set_range(grid, (0..rolling).map(|i| Point2::new(x, height - i - 1)));
    }
}

fn slide_east(grid: &mut Grid) {
    for y in 0..grid.height() {
        let mut rolling = 0;
        for x in 0..grid.width() {
            match grid[Point2::new(x, y)] {
                Cell::RoundRock => {
                    rolling += 1;
                    grid[Point2::new(x, y)] = Cell::Empty;
                }
                Cell::CubeRock => {
                    grid_set_range(grid, (0..rolling).map(|i| Point2::new(x - i - 1, y)));
                    rolling = 0;
                }
                Cell::Empty => {}
            }
        }
        let width = grid.width();
        grid_set_range(grid, (0..rolling).map(|i| Point2::new(width - i - 1, y)));
    }
}

fn slide_west(grid: &mut Grid) {
    for y in 0..grid.height() {
        let mut rolling = 0;
        for x in (0..grid.width()).rev() {
            match grid[Point2::new(x, y)] {
                Cell::RoundRock => {
                    rolling += 1;
                    grid[Point2::new(x, y)] = Cell::Empty;
                }
                Cell::CubeRock => {
                    grid_set_range(grid, (0..rolling).map(|i| Point2::new(x + i + 1, y)));
                    rolling = 0;
                }
                Cell::Empty => {}
            }
        }
        grid_set_range(grid, (0..rolling).map(|i| Point2::new(i, y)));
    }
}

fn cycle(grid: &mut Grid) {
    slide_north(grid);
    slide_west(grid);
    slide_south(grid);
    slide_east(grid);
}

fn calculate_load(grid: &Grid) -> usize {
    let height = grid.height();
    grid.iter_rows()
        .enumerate()
        .map(|(y, row)| (height - y) * row.filter(|cell| cell == &&Cell::RoundRock).count())
        .sum()
}

#[register_part]
fn part1(input: &str) -> usize {
    let mut grid = parse_input(input);
    slide_north(&mut grid);
    calculate_load(&grid)
}

#[register_part]
fn part2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let hasher = RandomState::new();
    let mut cache = HashMap::new();
    cache.insert(hasher.hash_one(&grid), (0, 0));
    let cycles = 1_000_000_000;
    for i in 0..cycles {
        cycle(&mut grid);
        let hash = hasher.hash_one(&grid);
        let Some((first, _)) = cache.get(&hash) else {
            cache.insert(hash, (i, calculate_load(&grid)));
            continue;
        };
        let first = *first;

        // We're in a loop, so figure out which loop member we will end on and use that.
        let remaining = cycles - i - 1;
        let loop_size = i - first;
        let steps = remaining % loop_size;
        return cache
            .into_iter()
            .find_map(|(_, (iteration, load))| {
                if iteration == first + steps {
                    Some(load)
                } else {
                    None
                }
            })
            .unwrap();
    }
    never!();
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 136, part2 = 64)]
    static EXAMPLE_INPUT: &str = "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    ";

    #[test]
    #[allow(clippy::too_many_lines)]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = [
            [
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::RoundRock,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            [
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
            ],
            [
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::CubeRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        ]
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn slide_north() {
        let mut grid = parse_input(&EXAMPLE_INPUT);
        super::slide_north(&mut grid);
        let expected = parse_input(
            "
                OOOO.#.O..
                OO..#....#
                OO..O##..O
                O..#.OO...
                ........#.
                ..#....#.#
                ..O..#.O.O
                ..O.......
                #....###..
                #....#....
            "
            .replace("\n                ", "\n")
            .trim(),
        );
        assert_eq!(grid, expected);
    }

    #[test]
    fn slide_east() {
        let mut grid = parse_input(&EXAMPLE_INPUT);
        super::slide_east(&mut grid);
        let expected = parse_input(
            "
                ....O#....
                .OOO#....#
                .....##...
                .OO#....OO
                ......OO#.
                .O#...O#.#
                ....O#..OO
                .........O
                #....###..
                #..OO#....
            "
            .replace("\n                ", "\n")
            .trim(),
        );
        assert_eq!(grid, expected);
    }

    #[test]
    fn slide_south() {
        let mut grid = parse_input(&EXAMPLE_INPUT);
        super::slide_south(&mut grid);
        let expected = parse_input(
            "
                .....#....
                ....#....#
                ...O.##...
                ...#......
                O.O....O#O
                O.#..O.#.#
                O....#....
                OO....OO..
                #OO..###..
                #OO.O#...O
            "
            .replace("\n                ", "\n")
            .trim(),
        );
        assert_eq!(grid, expected);
    }

    #[test]
    fn slide_west() {
        let mut grid = parse_input(&EXAMPLE_INPUT);
        super::slide_west(&mut grid);
        let expected = parse_input(
            "
                O....#....
                OOO.#....#
                .....##...
                OO.#OO....
                OO......#.
                O.#O...#.#
                O....#OO..
                O.........
                #....###..
                #OO..#....
            "
            .replace("\n                ", "\n")
            .trim(),
        );
        assert_eq!(grid, expected);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn cycle() {
        let mut grid = [
            [
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::RoundRock,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            [
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
            ],
            [
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::CubeRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        ]
        .into();

        let expected = [
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::CubeRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
            ],
            [
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        ]
        .into();
        super::cycle(&mut grid);
        assert_eq!(grid, expected);

        let expected = [
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::CubeRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
            ],
            [
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::CubeRock,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
        ]
        .into();
        super::cycle(&mut grid);
        assert_eq!(grid, expected);

        let expected = [
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
            ],
            [
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::CubeRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            [
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
            ],
            [
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::RoundRock,
            ],
            [
                Cell::CubeRock,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
        ]
        .into();
        super::cycle(&mut grid);
        assert_eq!(grid, expected);
    }
}
