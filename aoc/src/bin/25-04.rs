puzzle_lib::setup!(title = "Printing Department");

use puzzle_lib::grid::FullGrid;

type Grid = FullGrid<bool>;

fn parse_input(input: &str) -> Grid {
    parse!(input => {
        [grid cells with |c| c == '@']
    } => grid)
}

pub fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    grid.iter_pairs()
        .filter(|(p, v)| {
            **v && p
                .neighbours_diag()
                .into_iter()
                .filter(|n| grid.get(n).is_some_and(|nv| *nv))
                .count()
                < 4
        })
        .count()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 13)]
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
