puzzle_runner::register_chapter!(book = "2022", title = "Treetop Tree House");

use std::collections::HashSet;

use puzzle_lib::{
    grid::FullGrid,
    point::{Direction2, Point2},
};

type Grid = FullGrid<u8>;
type Point = Point2<u8>;

fn parse_input(input: &str) -> Grid {
    parse!(input => { [grid cells as u8] } => grid)
}

fn for_line_until(
    grid: &Grid,
    start: Point,
    direction: Direction2,
    predicate: &mut impl FnMut(Point, &u8) -> bool,
) {
    let mut current = start;
    loop {
        let Some(next) = current.checked_add_direction2(direction) else {
            return;
        };
        current = next;

        match grid.get(&current.cast()) {
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
    let mut highest = grid[start.cast()];
    for_line_until(grid, start, direction, &mut |point, height| {
        if height > &highest {
            points.insert(point);
            highest = *height;
        }
        height < &9
    });
}

fn count_visible_from_treehouse(grid: &Grid, start: Point, direction: Direction2) -> usize {
    let treehouse_height = &grid[start.cast()];
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
    let dimensions = Point::new(grid.width() as u8, grid.height() as u8);
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
    let dimensions = Point::new(grid.width() as u8, grid.height() as u8);
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

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
        let expected = [
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]
        .into();
        assert_eq!(actual, expected);
    }
}
