aoc::setup!(title = "Transparent Origami");

use aoc::point::Point2;

type Grid = Vec<Vec<bool>>;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Axis {
    X,
    Y,
}
impl From<&str> for Axis {
    fn from(value: &str) -> Self {
        match value {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => {
                panic!("Invalid fold axis {value:?}.");
            }
        }
    }
}

fn parse_input(input: &str) -> (Grid, Vec<Axis>) {
    // The numeric portion of the fold instruction doesn't actually matter since the instruction is
    // always fold in half over axis, so we just ignore this and only store what axis to fold on.

    parse!(input =>
        [points split on '\n' with
            { [x as usize] ',' [y as usize] }
            => Point2::new(x, y)
        ]
        "\n\n"
        [folds split on '\n' with
            { "fold along " [axis as Axis] "=" _ }
            => axis
        ]
    );

    let width = points.iter().max_by_key(|point| point.x).unwrap().x + 1;
    let height = points.iter().max_by_key(|point| point.y).unwrap().y + 1;
    let mut grid = (0..height)
        .map(|_| (0..width).map(|_| false).collect::<Vec<bool>>())
        .collect::<Grid>();
    for point in points {
        grid[point.y][point.x] = true;
    }

    (grid, folds)
}

fn do_fold(grid: Grid, axis: Axis) -> Grid {
    if axis == Axis::X {
        let mid = grid[0].len() / 2 + 1;
        grid.into_iter()
            .map(|row| {
                let left = &row[..mid - 1];
                let right = &row[mid..];
                left.iter()
                    .zip(right.iter().rev())
                    .map(|(l, r)| *l || *r)
                    .collect::<Vec<bool>>()
            })
            .collect()
    } else {
        let chunk_height = grid.len() / 2;
        let top = grid.iter().take(chunk_height);
        let bottom = grid.iter().skip(chunk_height + 1).take(chunk_height);
        top.zip(bottom.rev())
            .map::<Vec<bool>, _>(|(t_row, b_row)| {
                t_row
                    .iter()
                    .zip(b_row.iter())
                    .map(|(t, b)| *t || *b)
                    .collect()
            })
            .collect()
    }
}

fn format_grid(grid: &Grid) -> String {
    let mut result = String::new();
    for line in grid {
        for cell in line {
            result += if *cell { "█" } else { " " };
        }
        result += "\n";
    }
    result.pop();
    result
}

pub fn part1(input: &str) -> usize {
    let (mut grid, instructions) = parse_input(input);
    grid = do_fold(grid, instructions.into_iter().next().unwrap());
    grid.into_iter().flatten().filter(|v| *v).count()
}

pub fn part2(input: &str) -> String {
    let (mut grid, instructions) = parse_input(input);
    for axis in instructions {
        grid = do_fold(grid, axis);
    }
    format_grid(&grid)
}

#[cfg(test)]
mod tests {
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
        let expected_grid = vec![
            vec![
                false, false, false, true, false, false, true, false, false, true, false,
            ],
            vec![
                false, false, false, false, true, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                true, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, true, false, false, false, false, true, false, true,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                false, true, false, false, false, false, true, false, true, true, false,
            ],
            vec![
                false, false, false, false, true, false, false, false, false, false, false,
            ],
            vec![
                false, false, false, false, false, false, true, false, false, false, true,
            ],
            vec![
                true, false, false, false, false, false, false, false, false, false, false,
            ],
            vec![
                true, false, true, false, false, false, false, false, false, false, false,
            ],
        ];
        let expected_instructions = vec![Axis::Y, Axis::X];
        let (actual_grid, actual_instructions) = parse_input(&EXAMPLE_INPUT);
        assert_eq!(actual_grid, expected_grid);
        assert_eq!(actual_instructions, expected_instructions);
    }
}
