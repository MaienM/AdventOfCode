puzzle_runner::register_chapter!(title = "Like a GIF For Your Yard");

use puzzle_lib::{grid::FullGrid, point::Point2};

type Grid = FullGrid<bool>;

fn parse_input(input: &str) -> Grid {
    parse!(input => {
        [grid cells with |c| c == '#']
    } => grid)
}

#[inline]
fn cycle(grid: &Grid) -> Grid {
    let mut next = grid.map(|_| false);
    for (point, value) in grid.iter_pairs() {
        let neighbours = point
            .neighbours_diag()
            .into_iter()
            .filter(|p| grid.get(p) == Some(&true))
            .count();
        next[*point] = neighbours == 3 || (*value && neighbours == 2);
    }
    next
}

#[register_part(arg = 100)]
fn part1(input: &str, steps: u8) -> usize {
    let mut grid = parse_input(input);
    for _ in 0..steps {
        grid = cycle(&grid);
    }
    grid.into_iter_data().filter(|v| *v).count()
}

#[register_part(arg = 100)]
fn part2(input: &str, steps: u8) -> usize {
    let mut grid = parse_input(input);
    let corners = [
        Point2::new(0, 0),
        Point2::new(0, grid.height() - 1),
        Point2::new(grid.width() - 1, 0),
        Point2::new(grid.width() - 1, grid.height() - 1),
    ];
    for corner in &corners {
        grid[*corner] = true;
    }
    for _ in 0..steps {
        grid = cycle(&grid);
        for corner in &corners {
            grid[*corner] = true;
        }
    }
    grid.into_iter_data().filter(|v| *v).count()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 4, part1::arg = 4, part2 = 17, part2::arg = 5)]
    static EXAMPLE_INPUT: &str = "
        .#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..
    ";
}
