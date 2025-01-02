use std::collections::HashSet;

use aoc::utils::{
    parse,
    point::{Direction2, Point2},
};

type Grid = Vec<Vec<u8>>;
type Point = Point2<u8>;

fn parse_input(input: &str) -> Grid {
    parse!(input => { [rows split on '\n' with [chars as u8]] } => rows)
}

fn for_line_until(
    grid: &Grid,
    start: Point,
    direction: Direction2,
    predicate: &mut impl FnMut(Point, &u8) -> bool,
) {
    let mut current = start;
    loop {
        let Some(next) = current.checked_add_direction(direction, &1) else {
            return;
        };
        current = next;

        match grid
            .get(current.y as usize)
            .and_then(|r| r.get(current.x as usize))
        {
            Some(height) => {
                if !predicate(current, height) {
                    return;
                }
            }
            None => return,
        }
    }
}

fn find_visible_from_edge(
    grid: &Grid,
    points: &mut HashSet<Point>,
    start: Point,
    direction: Direction2,
) {
    let mut highest = grid[start.y as usize][start.x as usize];
    for_line_until(grid, start, direction, &mut |point, height| {
        if height > &highest {
            points.insert(point);
            highest = *height;
        }
        height < &9
    });
}

fn count_visible_from_treehouse(grid: &Grid, start: Point, direction: Direction2) -> usize {
    let treehouse_height = &grid[start.y as usize][start.x as usize];
    let mut count = 0;
    for_line_until(grid, start, direction, &mut |_, height| {
        count += 1;
        height < treehouse_height
    });
    count
}

pub fn part1(input: &str) -> usize {
    let grid = parse_input(input);

    let mut visible = HashSet::new();
    let dimensions = Point::new(grid[0].len() as u8, grid.len() as u8);
    visible.insert(Point::new(0, 0));
    visible.insert(Point::new(0, dimensions.y - 1));
    visible.insert(Point::new(dimensions.x - 1, 0));
    visible.insert(Point::new(dimensions.x - 1, dimensions.y - 1));

    for x in 0..dimensions.x {
        let north = Point::new(x, 0);
        visible.insert(north);
        find_visible_from_edge(&grid, &mut visible, north, Direction2::South);

        let south = Point::new(x, dimensions.y - 1);
        visible.insert(south);
        find_visible_from_edge(&grid, &mut visible, south, Direction2::North);
    }

    for y in 0..dimensions.y {
        let west = Point::new(0, y);
        visible.insert(west);
        find_visible_from_edge(&grid, &mut visible, west, Direction2::East);

        let east = Point::new(dimensions.x - 1, y);
        visible.insert(east);
        find_visible_from_edge(&grid, &mut visible, east, Direction2::West);
    }

    visible.len()
}

pub fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let dimensions = Point::new(grid[0].len() as u8, grid.len() as u8);
    (0..dimensions.x)
        .map(|x| {
            (0..dimensions.y)
                .map(|y| {
                    let point = Point::new(x, y);
                    let mut score = count_visible_from_treehouse(&grid, point, Direction2::North);
                    if score > 0 {
                        score *= count_visible_from_treehouse(&grid, point, Direction2::East);
                    }
                    if score > 0 {
                        score *= count_visible_from_treehouse(&grid, point, Direction2::South);
                    }
                    if score > 0 {
                        score *= count_visible_from_treehouse(&grid, point, Direction2::West);
                    }
                    score
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

aoc_runner::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 21, part2 = 8)]
    static EXAMPLE_INPUT: &str = "
        30373
        25512
        65332
        33549
        35390
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(actual, expected);
    }
}
