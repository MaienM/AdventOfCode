puzzle_runner::register_chapter!(book = "2024", title = "Ceres Search");

use puzzle_lib::{
    grid::FullGrid,
    point::{Direction2X, Point2},
};

type Grid = FullGrid<char>;

fn parse_input(input: &str) -> Grid {
    parse!(input => { [grid cells] } => grid)
}

#[inline]
fn test_mas_from(grid: &Grid, start: Point2, direction: Direction2X) -> bool {
    test_mas(grid, start + direction, direction)
}

fn test_mas(grid: &Grid, start: Point2, direction: Direction2X) -> bool {
    grid[start] == 'M' && grid[start + direction] == 'A' && grid[start + direction * 2] == 'S'
}

fn test_xmas_all_directions(grid: &Grid, dimensions: &Point2, start: Point2) -> usize {
    [
        start.x >= 3 && test_mas_from(grid, start, Direction2X::West),
        start.x >= 3 && start.y >= 3 && test_mas_from(grid, start, Direction2X::NorthWest),
        start.x >= 3
            && dimensions.y - start.y >= 4
            && test_mas_from(grid, start, Direction2X::SouthWest),
        (dimensions.x - start.x) >= 4 && test_mas_from(grid, start, Direction2X::East),
        (dimensions.x - start.x) >= 4
            && start.y >= 3
            && test_mas_from(grid, start, Direction2X::NorthEast),
        (dimensions.x - start.x) >= 4
            && dimensions.y - start.y >= 4
            && test_mas_from(grid, start, Direction2X::SouthEast),
        start.y >= 3 && test_mas_from(grid, start, Direction2X::North),
        dimensions.y - start.y >= 4 && test_mas_from(grid, start, Direction2X::South),
    ]
    .into_iter()
    .filter(|v| *v)
    .count()
}

fn test_x_mas(grid: &Grid, tl: Point2) -> bool {
    (test_mas(grid, tl, Direction2X::SouthEast)
        || test_mas(
            grid,
            Point2::new(tl.x + 2, tl.y + 2),
            Direction2X::NorthWest,
        ))
        && (test_mas(grid, Point2::new(tl.x + 2, tl.y), Direction2X::SouthWest)
            || test_mas(grid, Point2::new(tl.x, tl.y + 2), Direction2X::NorthEast))
}

pub fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let dimensions = Point2::new(grid.width(), grid.height());
    let mut result = 0;
    for y in 0..dimensions.y {
        for x in 0..dimensions.x {
            let point = Point2::new(x, y);
            if grid[point] == 'X' {
                result += test_xmas_all_directions(&grid, &dimensions, point);
            }
        }
    }
    result
}

pub fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let dimensions = Point2::new(grid.width(), grid.height());
    let mut result = 0;
    for y in 0..(dimensions.y - 2) {
        for x in 0..(dimensions.x - 2) {
            if test_x_mas(&grid, Point2::new(x, y)) {
                result += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 18, part2 = 9)]
    static EXAMPLE_INPUT: &str = "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = [
            ['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            ['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            ['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            ['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            ['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            ['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            ['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            ['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            ['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            ['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ]
        .into();
        assert_eq!(actual, expected);
    }
}
