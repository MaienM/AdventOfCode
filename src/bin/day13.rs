use aoc::{
    grid::{Grid as BaseGrid, Point},
    *,
};

type Grid = BaseGrid<bool>;
#[derive(Debug, PartialEq)]
enum FoldAxis {
    X,
    Y,
}
type Instruction = FoldAxis;

fn parse_input(input: String) -> (Grid, Vec<FoldAxis>) {
    let lines = input
        .trim()
        .split("\n")
        .map(str::trim)
        .collect::<Vec<&str>>();
    let mut split = lines.split(|line| line.is_empty());
    let grid_lines = split.next().unwrap();
    let instructions_lines = split.next().unwrap();

    let points = grid_lines
        .into_iter()
        .map(|line| {
            let mut parts = line.splitn(2, ",").map(str::parse).map(Result::unwrap);
            return Point::new(parts.next().unwrap(), parts.next().unwrap());
        })
        .collect::<Vec<Point>>();
    let width = points.iter().max_by_key(|point| point.x).unwrap().x + 1;
    let height = points.iter().max_by_key(|point| point.y).unwrap().y + 1;
    let mut grid = (0..height)
        .map(|_| (0..width).map(|_| false).collect::<Vec<bool>>())
        .collect::<Grid>();
    for point in points {
        grid.setp(point, true);
    }

    let instructions = instructions_lines
        .into_iter()
        .map(|line| {
            assert!(line.starts_with("fold along "));
            let axis = match line.chars().nth(11) {
                Some('x') => FoldAxis::X,
                Some('y') => FoldAxis::Y,
                _ => {
                    panic!("Invalid fold axis.");
                }
            };
            // The numeric portion doesn't actually matter since the instruction is always fold in half over axis.
            // let num = line[13..].parse().unwrap();
            return axis;
        })
        .collect::<Vec<Instruction>>();

    return (grid, instructions);
}

fn do_fold(grid: Grid, instruction: Instruction) -> Grid {
    if instruction == FoldAxis::X {
        let mid = grid.width / 2 + 1;
        return grid
            .into_iter()
            .map(|row| {
                let left = &row[..mid - 1];
                let right = &row[mid..];
                return left
                    .into_iter()
                    .zip(right.into_iter().rev())
                    .map(|(l, r)| *l || *r)
                    .collect::<Vec<bool>>();
            })
            .collect();
    } else {
        let chunk_height = grid.height / 2;
        let top = grid.iter().take(chunk_height);
        let bottom = grid.iter().skip(chunk_height + 1).take(chunk_height);
        // println!("{:?} -> {:?} | {:?}", grid, top, bottom);
        return top
            .zip(bottom.rev())
            .map::<Vec<bool>, _>(|(trow, brow)| {
                trow.into_iter()
                    .zip(brow.into_iter())
                    .map(|(t, b)| *t || *b)
                    .collect()
            })
            .collect();
    }
}

fn part1(input: String) -> i64 {
    let (mut grid, instructions) = parse_input(input);
    grid = do_fold(grid, instructions.into_iter().next().unwrap());
    return grid.into_by_cell().filter(|(_, value)| *value).count() as i64;
}

fn main() {
    run(part1, missing);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "
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
        ]
        .into();
        let expected_instructions = vec![FoldAxis::Y, FoldAxis::X];
        let (actual_grid, actual_instructions) = parse_input(EXAMPLE_INPUT.to_string());
        assert_eq!(actual_grid, expected_grid);
        assert_eq!(actual_instructions, expected_instructions);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 17);
    }
}
