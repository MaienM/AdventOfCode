use std::collections::HashSet;

use aoc::runner::*;
use aoc::grid::Grid as BaseGrid;
use aoc::grid::Point;

type Grid = BaseGrid<u8>; 

fn parse_input(input: String) -> Grid {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            line.trim()
                .chars()
                .map(|chr| chr.to_digit(10).unwrap() as u8)
                .into_iter()
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
        .into();
}

fn find_visible(grid: &Grid, points: &mut HashSet<Point>, start: Point, offset: (isize, isize)) {
    let mut highest = grid.getp(start).unwrap();
    let mut current = start;
    loop {
        current = Point::new((current.x as isize + offset.0) as usize, (current.y as isize + offset.1) as usize);
        match grid.getp(current) {
            Some(height) => {
                if height > highest {
                    points.insert(current);
                    highest = height;

                    if height == &9 {
                        return;
                    }
                }
            },
            None => return,
        }
    }
}

pub fn part1(input: String) -> usize {
    let grid = parse_input(input);

    let mut visible = HashSet::new();
    visible.insert(Point::new(0, 0));
    visible.insert(Point::new(0, grid.height - 1));
    visible.insert(Point::new(grid.width - 1, 0));
    visible.insert(Point::new(grid.width - 1, grid.height - 1));

    for x in 0..grid.width {
        let north = Point::new(x, 0);
        visible.insert(north);
        find_visible(&grid, &mut visible, north, (0, 1));

        let south = Point::new(x, grid.height - 1);
        visible.insert(south);
        find_visible(&grid, &mut visible, south, (0, -1));
    }

    for y in 0..grid.height {
        let west = Point::new(0, y);
        visible.insert(west);
        find_visible(&grid, &mut visible, west, (1, 0));

        let east = Point::new(grid.width - 1, y);
        visible.insert(east);
        find_visible(&grid, &mut visible, east, (-1, 0));
    }

    return visible.len();
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        30373
        25512
        65332
        33549
        35390
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ].into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 21);
    }
}
