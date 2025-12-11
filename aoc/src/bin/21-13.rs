puzzle_runner::register_chapter!(book = 2021, title = "Transparent Origami");

use puzzle_lib::{grid::SparsePointSet, point::Point2};

type Grid = SparsePointSet<u16>;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Axis {
    X,
    Y,
}

type Fold = (Axis, u16);

fn parse_input(input: &str) -> (Grid, Vec<Fold>) {
    parse!(input => {
        [grid split on '\n' into (Grid) with
            { [x as u16] ',' [y as u16] }
            => Point2::new(x, y)
        ]
        "\n\n"
        [folds split on '\n' with
            {
                "fold along "
                [axis match {
                    "x" => Axis::X,
                    "y" => Axis::Y,
                }]
                "="
                [num as u16]
            }
            => (axis, num)
        ]
    } => (grid, folds))
}

fn do_fold(grid: Grid, fold: Fold) -> Grid {
    match fold {
        (Axis::X, fold) => grid
            .into_iter_points()
            .map(|p| {
                if p.x > fold {
                    Point2::new(fold - (p.x - fold), p.y)
                } else {
                    p
                }
            })
            .collect(),
        (Axis::Y, fold) => grid
            .into_iter_points()
            .map(|p| {
                if p.y > fold {
                    Point2::new(p.x, fold - (p.y - fold))
                } else {
                    p
                }
            })
            .collect(),
    }
}

fn format_grid(grid: &Grid) -> String {
    let mut result = String::new();
    let bounds = Point2::new(
        grid.iter_points().max_by_key(|p| p.x).unwrap().x,
        grid.iter_points().max_by_key(|p| p.y).unwrap().y,
    );
    for y in 0..=bounds.y {
        for x in 0..=bounds.x {
            result += if grid.contains_point(&Point2::new(x, y)) {
                "█"
            } else {
                " "
            };
        }
        result += "\n";
    }
    result.pop();
    result
}

#[register_part]
fn part1(input: &str) -> usize {
    let (mut grid, instructions) = parse_input(input);
    grid = do_fold(grid, instructions.into_iter().next().unwrap());
    grid.into_iter_points().count()
}

#[register_part]
fn part2(input: &str) -> String {
    let (mut grid, instructions) = parse_input(input);
    for axis in instructions {
        grid = do_fold(grid, axis);
    }
    format_grid(&grid)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use common_macros::hash_set;
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(
        part1 = 17,
        part2 = "
            █████
            █   █
            █   █
            █   █
            █████
        "
    )]
    static EXAMPLE_INPUT: &str = "
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    ";

    #[test]
    fn example_parse() {
        let expected_grid = hash_set![
            Point2::new(6, 10),
            Point2::new(0, 14),
            Point2::new(9, 10),
            Point2::new(0, 3),
            Point2::new(10, 4),
            Point2::new(4, 11),
            Point2::new(6, 0),
            Point2::new(6, 12),
            Point2::new(4, 1),
            Point2::new(0, 13),
            Point2::new(10, 12),
            Point2::new(3, 4),
            Point2::new(3, 0),
            Point2::new(8, 4),
            Point2::new(1, 10),
            Point2::new(2, 14),
            Point2::new(8, 10),
            Point2::new(9, 0),
        ];
        let expected_instructions = vec![(Axis::Y, 7), (Axis::X, 5)];
        let (actual_grid, actual_instructions) = parse_input(&EXAMPLE_INPUT);
        assert_eq!(
            actual_grid.into_iter_points().collect::<HashSet<_>>(),
            expected_grid
        );
        assert_eq!(actual_instructions, expected_instructions);
    }
}
