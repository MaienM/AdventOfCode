puzzle_runner::register_chapter!(book = 2024, title = "RAM Run");

use std::collections::{BinaryHeap, HashSet};

use puzzle_lib::point::Point2;

fn parse_input(input: &str) -> Vec<Point2> {
    parse!(input => {
        [coordinates split on '\n' with
            { [x as usize] ',' [y as usize] }
            => Point2::new(x, y)
        ]
    } => coordinates)
}

fn run(walls: &[Point2], end: Point2) -> Option<usize> {
    let mut paths = BinaryHeap::new();
    let mut done = HashSet::new();
    paths.push((0, Point2::new(0, 0)));
    loop {
        let (score, point) = paths.pop()?;

        if point == end {
            return Some(-score as usize);
        }

        if walls.contains(&point) || point.x > end.x || point.y > end.y {
            continue;
        }
        if !done.insert(point) {
            continue;
        }

        paths.extend(point.neighbours_ortho().into_iter().map(|p| (score - 1, p)));
    }
}

#[register_part(arg = (1024, Point2::new(70, 70)))]
fn part1(input: &str, (count, end): (usize, Point2)) -> usize {
    let coordinates = parse_input(input);
    run(&coordinates[..count], end).unwrap()
}

#[register_part(arg = Point2::new(70, 70))]
fn part2(input: &str, end: Point2) -> String {
    let coordinates = parse_input(input);
    let idx = (0..coordinates.len()).partition_point(|v| run(&coordinates[..v], end).is_none());
    let point = coordinates[idx.unwrap() - 1];
    format!("{},{}", point.x, point.y)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(
        part1 = 22,
        part1::arg = (12, Point2::new(6, 6)),
        part2 = "6,1",
        part2::arg = Point2::new(6, 6),
    )]
    static EXAMPLE_INPUT: &str = "
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    ";
}
