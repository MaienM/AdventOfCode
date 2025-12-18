puzzle_runner::register_chapter!(title = "Point of Incidence");

type Grid = Vec<Vec<bool>>;

fn parse_input(input: &str) -> Vec<Grid> {
    parse!(input => {
        [grids split on "\n\n" with [
            split on '\n' with [chars with |c| c == '#']]
        ]
    } => grids)
}

fn rotate(grid: &Grid) -> Grid {
    (0..grid[0].len())
        .map(|i| grid.iter().map(|row| row[i]).collect())
        .collect()
}

fn find_reflection_row(grid: &Grid, with_smudge: bool) -> Option<usize> {
    let len = grid.len();
    'row: for i in 0..(len - 1) {
        let mut found_smudge = !with_smudge;
        for o in 0..=i.min(len - i - 2) {
            let left = &grid[i - o];
            let right = &grid[i + o + 1];
            let diff = (0..left.len()).find(|i| left[*i] != right[*i]);
            match diff {
                None => {}
                Some(idx) if !found_smudge => {
                    found_smudge = true;
                    if ((idx + 1)..left.len()).any(|i| left[i] != right[i]) {
                        continue 'row;
                    }
                }
                Some(_) => continue 'row,
            }
        }
        if found_smudge {
            return Some(i);
        }
    }
    None
}

fn solve(input: &str, with_smudge: bool) -> usize {
    let grids = parse_input(input);
    grids
        .into_iter()
        .map(|grid| {
            if let Some(row) = find_reflection_row(&grid, with_smudge) {
                (row + 1) * 100
            } else {
                let grid = rotate(&grid);
                let column = find_reflection_row(&grid, with_smudge).unwrap();
                column + 1
            }
        })
        .sum()
}

#[register_part]
fn part1(input: &str) -> usize {
    solve(input, false)
}

#[register_part]
fn part2(input: &str) -> usize {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 405, part2 = 400)]
    static EXAMPLE_INPUT: &str = "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            vec![
                vec![true, false, true, true, false, false, true, true, false],
                vec![false, false, true, false, true, true, false, true, false],
                vec![true, true, false, false, false, false, false, false, true],
                vec![true, true, false, false, false, false, false, false, true],
                vec![false, false, true, false, true, true, false, true, false],
                vec![false, false, true, true, false, false, true, true, false],
                vec![true, false, true, false, true, true, false, true, false],
            ],
            vec![
                vec![true, false, false, false, true, true, false, false, true],
                vec![true, false, false, false, false, true, false, false, true],
                vec![false, false, true, true, false, false, true, true, true],
                vec![true, true, true, true, true, false, true, true, false],
                vec![true, true, true, true, true, false, true, true, false],
                vec![false, false, true, true, false, false, true, true, true],
                vec![true, false, false, false, false, true, false, false, true],
            ],
        ];
        assert_eq!(actual, expected);
    }
}
