puzzle_runner::register_chapter!(book = 2015, title = "Like a GIF For Your Yard");

use puzzle_lib::grid::FullGrid;

type Grid = FullGrid<bool>;

fn parse_input(input: &str) -> Grid {
    parse!(input => {
        [grid cells with |c| c == '#']
    } => grid)
}

#[register_part(arg = 100)]
fn part1(input: &str, steps: u8) -> usize {
    let mut grid = parse_input(input);
    for _ in 0..steps {
        let mut next = grid.map(|_| false);
        for (point, value) in grid.iter_pairs() {
            let neighbours = point
                .neighbours_diag()
                .into_iter()
                .filter(|p| grid.get(p) == Some(&true))
                .count();
            next[*point] = neighbours == 3 || (*value && neighbours == 2);
        }
        grid = next;
    }
    grid.into_iter_data().filter(|v| *v).count()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 4, part1::arg = 4)]
    static EXAMPLE_INPUT: &str = "
        .#.#.#
        ...##.
        #....#
        ..#...
        #.#..#
        ####..
    ";
}
