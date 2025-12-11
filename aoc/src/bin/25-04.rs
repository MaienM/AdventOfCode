puzzle_runner::register_chapter!(book = 2025, title = "Printing Department");

use puzzle_lib::{grid::FullGrid, point::Point2};

type Grid = FullGrid<bool>;

fn parse_input(input: &str) -> Grid {
    parse!(input => {
        [grid cells with |c| c == '@']
    } => grid)
}

#[inline]
fn can_remove(grid: &Grid, (p, v): (&Point2, &bool)) -> bool {
    *v && p
        .neighbours_diag()
        .into_iter()
        .filter(|n| grid.get(n).is_some_and(|nv| *nv))
        .count()
        < 4
}

#[register_part]
fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    grid.iter_pairs()
        .filter(|(p, v)| can_remove(&grid, (*p, *v)))
        .count()
}

#[register_part]
fn part2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut removed = 0;
    loop {
        let to_remove: Vec<_> = grid
            .iter_pairs()
            .filter_map(|(p, v)| {
                if can_remove(&grid, (p, v)) {
                    Some(*p)
                } else {
                    None
                }
            })
            .collect();
        if to_remove.is_empty() {
            break;
        }
        removed += to_remove.len();
        for point in to_remove {
            grid[point] = false;
        }
    }
    removed
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 13, part2 = 43)]
    static EXAMPLE_INPUT: &str = "
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    ";
}
