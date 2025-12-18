puzzle_runner::register_chapter!(title = "Smoke Basin");

use std::collections::HashSet;

use puzzle_lib::{grid::FullGrid, point::Point2};

type Grid = FullGrid<u8>;
type Basin = HashSet<Point2>;

fn parse_input(input: &str) -> Grid {
    parse!(input => { [grid cells as u8] } => grid)
}

fn get_low_points(grid: &Grid) -> Vec<Point2> {
    grid.iter_pairs()
        .filter(|(point, height)| {
            grid.get_filter_many(point.neighbours_ortho().iter())
                .all(|(_, h)| h > *height)
        })
        .map(|(point, _)| *point)
        .collect()
}

fn expand_basin(grid: &Grid, basin: &mut Basin, point: Point2) {
    if basin.contains(&point) || grid.get(&point).is_none_or(|h| *h == 9) {
        return;
    }
    basin.insert(point);
    for neighbour in point.neighbours_ortho() {
        expand_basin(grid, basin, neighbour);
    }
}

#[register_part]
fn part1(input: &str) -> u32 {
    let grid = parse_input(input);
    get_low_points(&grid)
        .into_iter()
        .map(|point| u32::from(grid[point] + 1))
        .sum()
}

#[register_part]
fn part2(input: &str) -> u32 {
    let grid = parse_input(input);
    let mut basin_sizes = get_low_points(&grid)
        .into_iter()
        .map(|point| {
            let mut basin = Basin::new();
            expand_basin(&grid, &mut basin, point);
            basin.len()
        })
        .collect::<Vec<usize>>();
    basin_sizes.sort_unstable();
    (basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap()) as u32
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 15, part2 = 1134)]
    static EXAMPLE_INPUT: &str = "
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = [
            [2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            [3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            [9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            [8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            [9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]
        .into();
        assert_eq!(actual, expected);
    }
}
