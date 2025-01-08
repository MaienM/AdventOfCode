puzzle_lib::setup!(title = "Smoke Basin");

use std::collections::HashSet;

use puzzle_lib::point::Point2;

type Grid = Vec<Vec<u8>>;
type Basin = HashSet<Point2>;

fn parse_input(input: &str) -> Grid {
    parse!(input => {
        [grid split on '\n' with [chars as u8]]
    } => grid)
}

fn get_low_points(grid: &Grid) -> Vec<Point2> {
    (0..grid[0].len())
        .cartesian_product(0..grid.len())
        .map(|(x, y)| Point2::new(x, y))
        .filter(|point| {
            let height = grid[point.y][point.x];
            point.neighbours_ortho().into_iter().all(|n| {
                grid.get(n.y)
                    .and_then(|row| row.get(n.x))
                    .is_none_or(|h| *h > height)
            })
        })
        .collect()
}

fn expand_basin(grid: &Grid, basin: &mut Basin, point: Point2) {
    if basin.contains(&point)
        || grid
            .get(point.y)
            .and_then(|row| row.get(point.x))
            .is_none_or(|h| *h == 9)
    {
        return;
    }
    basin.insert(point);
    for neighbour in point.neighbours_ortho() {
        expand_basin(grid, basin, neighbour);
    }
}

pub fn part1(input: &str) -> u32 {
    let grid = parse_input(input);
    get_low_points(&grid)
        .into_iter()
        .map(|point| u32::from(grid[point.y][point.x] + 1))
        .sum()
}

pub fn part2(input: &str) -> u32 {
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
        let expected = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(actual, expected);
    }
}
