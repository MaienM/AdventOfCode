aoc::setup!(title = "Ceres Search");

use aoc::point::Point2;

type Point = Point2<isize>;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    parse!(input => {
        [chars split on '\n' with [chars]]
    } => chars)
}

fn test_mas_from(chars: &[Vec<char>], start: Point, offset: Point) -> bool {
    test_mas(chars, start + offset, offset)
}

fn test_mas(chars: &[Vec<char>], start: Point, offset: Point) -> bool {
    chars[start.y as usize][start.x as usize] == 'M'
        && chars[(start.y + offset.y) as usize][(start.x + offset.x) as usize] == 'A'
        && chars[(start.y + offset.y * 2) as usize][(start.x + offset.x * 2) as usize] == 'S'
}

fn test_xmas_all_directions(chars: &[Vec<char>], dimensions: &Point, start: Point) -> usize {
    [
        start.x >= 3 && test_mas_from(chars, start, Point::new(-1, 0)),
        start.x >= 3 && start.y >= 3 && test_mas_from(chars, start, Point::new(-1, -1)),
        start.x >= 3
            && dimensions.y - start.y >= 4
            && test_mas_from(chars, start, Point::new(-1, 1)),
        (dimensions.x - start.x) >= 4 && test_mas_from(chars, start, Point::new(1, 0)),
        (dimensions.x - start.x) >= 4
            && start.y >= 3
            && test_mas_from(chars, start, Point::new(1, -1)),
        (dimensions.x - start.x) >= 4
            && dimensions.y - start.y >= 4
            && test_mas_from(chars, start, Point::new(1, 1)),
        start.y >= 3 && test_mas_from(chars, start, Point::new(0, -1)),
        dimensions.y - start.y >= 4 && test_mas_from(chars, start, Point::new(0, 1)),
    ]
    .into_iter()
    .filter(|v| *v)
    .count()
}

fn test_x_mas(chars: &[Vec<char>], tl: Point) -> bool {
    (test_mas(chars, tl, Point::new(1, 1))
        || test_mas(chars, Point::new(tl.x + 2, tl.y + 2), Point::new(-1, -1)))
        && (test_mas(chars, Point::new(tl.x + 2, tl.y), Point::new(-1, 1))
            || test_mas(chars, Point::new(tl.x, tl.y + 2), Point::new(1, -1)))
}

pub fn part1(input: &str) -> usize {
    let chars = parse_input(input);
    let dimensions = Point::new(chars[0].len() as isize, chars[0].len() as isize);
    let mut result = 0;
    for y in 0..dimensions.y {
        for x in 0..dimensions.x {
            if chars[y as usize][x as usize] == 'X' {
                result += test_xmas_all_directions(&chars, &dimensions, Point::new(x, y));
            }
        }
    }
    result
}

pub fn part2(input: &str) -> usize {
    let chars = parse_input(input);
    let dimensions = Point::new(chars[0].len() as isize, chars[0].len() as isize);
    let mut result = 0;
    for y in 0..(dimensions.y - 2) {
        for x in 0..(dimensions.x - 2) {
            if test_x_mas(&chars, Point::new(x, y)) {
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
        let expected = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        assert_eq!(actual, expected);
    }
}
