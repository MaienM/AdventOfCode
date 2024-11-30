use std::{collections::HashSet, ops::RangeInclusive};

use aoc::utils::{
    abs_diff, parse,
    point::{Point2, Point3},
};

type Point = Point3<usize>;

fn parse_input(input: &str) -> Vec<(Point, Point)> {
    parse!(input => {
        [bricks split on '\n' with
            { [x1 as usize] ',' [y1 as usize] ',' [z1 as usize] '~' [x2 as usize] ',' [y2 as usize] ',' [z2 as usize] }
            => (Point::new(x1, y1, z1), Point::new(x2, y2, z2))
        ]
    } => bricks)
}

fn range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a > b {
        b..=a
    } else {
        a..=b
    }
}

pub fn part1(input: &str) -> usize {
    let mut bricks = parse_input(input);
    bricks.sort_by_key(|b| usize::min(b.0.z, b.1.z));
    let bounds = Point2::new(
        bricks
            .iter()
            .map(|b| usize::max(b.0.x, b.1.x))
            .max()
            .unwrap()
            + 1,
        bricks
            .iter()
            .map(|b| usize::max(b.0.y, b.1.y))
            .max()
            .unwrap()
            + 1,
    );
    let mut can_be_disintegrated: Vec<_> = (0..=bricks.len()).map(|_| true).collect();
    let mut map: Vec<Vec<_>> = (0..bounds.y)
        .map(|_| (0..bounds.x).map(|_| (0, 0)).collect())
        .collect();
    for (idx, (p1, p2)) in bricks.into_iter().enumerate() {
        // We'll treat the ground as brick ID 0, and offset the rest by one to compensate.
        let id = idx + 1;

        // Find resting z level.
        let mut resting_z = 0;
        for x in range(p1.x, p2.x) {
            for y in range(p1.y, p2.y) {
                resting_z = usize::max(resting_z, map[y][x].0);
            }
        }
        let top_z = resting_z + abs_diff(p1.z, p2.z) + 1;

        // Update map & find out what brick(s) we're resting on.
        let mut resting_on = HashSet::new();
        for x in range(p1.x, p2.x) {
            for y in range(p1.y, p2.y) {
                let (base_z, base_id) = &map[y][x];
                if *base_z == resting_z {
                    resting_on.insert(*base_id);
                }

                map[y][x] = (top_z, id);
            }
        }

        // Mark the brick we're resting on as cannot-be-disintegrated if there's only a single one.
        if resting_on.len() == 1 {
            for id in resting_on {
                can_be_disintegrated[id] = false;
            }
        }
    }

    can_be_disintegrated.into_iter().filter(|v| *v).count()
}

pub fn part2(input: &str) -> usize {
    let mut bricks = parse_input(input);
    bricks.sort_by_key(|b| usize::min(b.0.z, b.1.z));
    let bounds = Point2::new(
        bricks
            .iter()
            .map(|b| usize::max(b.0.x, b.1.x))
            .max()
            .unwrap()
            + 1,
        bricks
            .iter()
            .map(|b| usize::max(b.0.y, b.1.y))
            .max()
            .unwrap()
            + 1,
    );
    let mut map: Vec<Vec<_>> = (0..bounds.y)
        .map(|_| (0..bounds.x).map(|_| (0, 0)).collect())
        .collect();
    let mut foundations: Vec<HashSet<usize>> = Vec::new();
    foundations.push(HashSet::new());
    for (idx, (p1, p2)) in bricks.into_iter().enumerate() {
        // We'll treat the ground as brick ID 0, and offset the rest by one to compensate.
        let id = idx + 1;

        // Find resting z level.
        let mut resting_z = 0;
        for x in range(p1.x, p2.x) {
            for y in range(p1.y, p2.y) {
                resting_z = usize::max(resting_z, map[y][x].0);
            }
        }
        let top_z = resting_z + abs_diff(p1.z, p2.z) + 1;

        // Update map & find out what brick(s) we're resting on.
        let mut resting_on = HashSet::new();
        for x in range(p1.x, p2.x) {
            for y in range(p1.y, p2.y) {
                let (base_z, base_id) = &map[y][x];
                if *base_z == resting_z {
                    resting_on.insert(*base_id);
                }

                map[y][x] = (top_z, id);
            }
        }

        // Mark the bricks we're resting on as foundations.
        foundations.push(resting_on);
    }

    let mut sum = 0;
    for i in 1..foundations.len() {
        let mut gone = HashSet::from([i]);
        for (j, foundations_j) in foundations.iter().enumerate().skip(i + 1) {
            if foundations_j.difference(&gone).all(|_| false) {
                gone.insert(j);
            }
        }
        sum += gone.len() - 1;
    }
    sum
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 5, part2 = 7)]
    static EXAMPLE_INPUT: &str = "
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            (Point::new(1, 0, 1), Point::new(1, 2, 1)),
            (Point::new(0, 0, 2), Point::new(2, 0, 2)),
            (Point::new(0, 2, 3), Point::new(2, 2, 3)),
            (Point::new(0, 0, 4), Point::new(0, 2, 4)),
            (Point::new(2, 0, 5), Point::new(2, 2, 5)),
            (Point::new(0, 1, 6), Point::new(2, 1, 6)),
            (Point::new(1, 1, 8), Point::new(1, 1, 9)),
        ];
        assert_eq!(actual, expected);
    }
}
