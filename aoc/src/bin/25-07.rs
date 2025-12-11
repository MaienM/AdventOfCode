puzzle_runner::register_chapter!(book = 2025, title = "Laboratories");

use puzzle_lib::{grid::FullGrid, point::Point2};

type Grid = FullGrid<bool>;

fn parse_input(input: &str) -> (Grid, Point2) {
    parse!(input => {
        [grid cells match {
            'S' => index into start => (false),
            '.' => (false),
            '^' => (true),
        }]
    } => (grid, start))
}

#[register_part]
fn part1(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    let mut split = 0;
    let mut beams: Vec<_> = (0..grid.width()).map(|_| false).collect();
    beams[start.x] = true;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if beams[x] && grid[Point2::new(x, y)] {
                split += 1;
                beams[x - 1] = true;
                beams[x] = false;
                beams[x + 1] = true;
            }
        }
    }
    split
}

#[register_part]
fn part2(input: &str) -> usize {
    let (grid, start) = parse_input(input);
    let mut beams: Vec<_> = (0..grid.width()).map(|_| 0).collect();
    beams[start.x] = 1;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let count = beams[x];
            if count > 0 && grid[Point2::new(x, y)] {
                beams[x - 1] += count;
                beams[x] = 0;
                beams[x + 1] += count;
            }
        }
    }
    beams.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 21, part2 = 40)]
    static EXAMPLE_INPUT: &str = "
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    ";
}
