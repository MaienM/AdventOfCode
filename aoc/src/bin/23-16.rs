puzzle_runner::register_chapter!(book = 2023, title = "The Floor Will Be Lava");

use std::{collections::HashSet, sync::Arc};

use puzzle_lib::{
    grid::FullGrid,
    point::{Direction2, Point2},
};

type Point = Point2;
type Direction = Direction2;

#[derive(Debug, PartialEq)]
enum Tile {
    MirrorUpRight,
    MirrorUpLeft,
    SplitterHorizontal,
    SplitterVertical,
    None,
}

fn parse_input(input: &str) -> FullGrid<Tile> {
    parse!(input => {
        [grid cells match {
            '|' => Tile::SplitterVertical,
            '-' => Tile::SplitterHorizontal,
            '/' => Tile::MirrorUpRight,
            '\\' => Tile::MirrorUpLeft,
            '.' => Tile::None,
        }]
    } => grid)
}

fn track_beams(
    grid: &FullGrid<Tile>,
    bounds: &Point,
    energized: &mut FullGrid<bool>,
    processed: &mut HashSet<(Point, Direction)>,
    start: Point,
    direction: Direction,
) {
    let is_new = processed.insert((start, direction));
    if !is_new {
        return;
    }

    let mut point = start;
    while point.x < bounds.x && point.y < bounds.y {
        energized[point] = true;

        match grid[point] {
            Tile::MirrorUpRight => {
                let direction = match direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                };
                track_beams(
                    grid,
                    bounds,
                    energized,
                    processed,
                    point.wrapping_add_direction2(direction),
                    direction,
                );
                return;
            }
            Tile::MirrorUpLeft => {
                let direction = match direction {
                    Direction::North => Direction::West,
                    Direction::West => Direction::North,
                    Direction::South => Direction::East,
                    Direction::East => Direction::South,
                };
                track_beams(
                    grid,
                    bounds,
                    energized,
                    processed,
                    point.wrapping_add_direction2(direction),
                    direction,
                );
                return;
            }
            Tile::SplitterHorizontal
                if matches!(direction, Direction::North | Direction::South) =>
            {
                track_beams(grid, bounds, energized, processed, point, Direction::East);
                track_beams(grid, bounds, energized, processed, point, Direction::West);
                return;
            }
            Tile::SplitterVertical if matches!(direction, Direction::East | Direction::West) => {
                track_beams(grid, bounds, energized, processed, point, Direction::North);
                track_beams(grid, bounds, energized, processed, point, Direction::South);
                return;
            }
            _ => {}
        }

        point = point.wrapping_add_direction2(direction);
    }
}

fn solve_from_position(
    grid: &FullGrid<Tile>,
    bounds: &Point,
    start: Point,
    direction: Direction,
) -> usize {
    let mut results: FullGrid<bool> = (0..bounds.y)
        .map(|_| (0..bounds.x).map(|_| false))
        .collect();
    track_beams(
        grid,
        bounds,
        &mut results,
        &mut HashSet::new(),
        start,
        direction,
    );
    results.into_iter_data().filter(|v| *v).count()
}

#[register_part]
fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let bounds = Point::new(grid.width(), grid.height());
    solve_from_position(&grid, &bounds, Point::new(0, 0), Direction::East)
}

#[register_part]
fn part2(input: &str) -> usize {
    let grid = Arc::new(parse_input(input));
    let bounds = Arc::new(Point::new(grid.width(), grid.height()));

    let mut options = Vec::new();
    options.extend((0..bounds.x).map(|x| (Point::new(x, 0), Direction::South)));
    options.extend((0..bounds.x).map(|x| (Point::new(x, bounds.y), Direction::North)));
    options.extend((0..bounds.y).map(|y| (Point::new(0, y), Direction::East)));
    options.extend((0..bounds.y).map(|y| (Point::new(bounds.x, y), Direction::West)));
    options
        .into_par_iter()
        .map(|(point, direction)| solve_from_position(&grid, &bounds, point, direction))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 46, part2 = 51)]
    static EXAMPLE_INPUT: &str = r#"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
    "#;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = [
            [
                Tile::None,
                Tile::SplitterVertical,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::MirrorUpLeft,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
            ],
            [
                Tile::SplitterVertical,
                Tile::None,
                Tile::SplitterHorizontal,
                Tile::None,
                Tile::MirrorUpLeft,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
            ],
            [
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::SplitterVertical,
                Tile::SplitterHorizontal,
                Tile::None,
                Tile::None,
                Tile::None,
            ],
            [
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::SplitterVertical,
                Tile::None,
            ],
            [
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
            ],
            [
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::MirrorUpLeft,
            ],
            [
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::MirrorUpRight,
                Tile::None,
                Tile::MirrorUpLeft,
                Tile::MirrorUpLeft,
                Tile::None,
                Tile::None,
            ],
            [
                Tile::None,
                Tile::SplitterHorizontal,
                Tile::None,
                Tile::SplitterHorizontal,
                Tile::MirrorUpRight,
                Tile::None,
                Tile::None,
                Tile::SplitterVertical,
                Tile::None,
                Tile::None,
            ],
            [
                Tile::None,
                Tile::SplitterVertical,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::SplitterHorizontal,
                Tile::SplitterVertical,
                Tile::None,
                Tile::MirrorUpLeft,
            ],
            [
                Tile::None,
                Tile::None,
                Tile::MirrorUpRight,
                Tile::MirrorUpRight,
                Tile::None,
                Tile::SplitterVertical,
                Tile::None,
                Tile::None,
                Tile::None,
                Tile::None,
            ],
        ]
        .into();
        assert_eq!(actual, expected);
    }
}
