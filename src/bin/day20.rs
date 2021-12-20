use std::collections::HashSet;
use std::fmt::Debug;

#[macro_use]
extern crate derive_new;

use aoc::grid::Grid;
use aoc::runner::*;

type Algorithm = [bool; 512];

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point(i32, i32);
impl Point {
    fn block(&self) -> [Point; 9] {
        return [
            Self(self.0 - 1, self.1 - 1),
            Self(self.0, self.1 - 1),
            Self(self.0 + 1, self.1 - 1),
            Self(self.0 - 1, self.1),
            Self(self.0, self.1),
            Self(self.0 + 1, self.1),
            Self(self.0 - 1, self.1 + 1),
            Self(self.0, self.1 + 1),
            Self(self.0 + 1, self.1 + 1),
        ];
    }
}

#[derive(Debug, Eq, PartialEq, new)]
struct Bounds {
    x: (i32, i32),
    y: (i32, i32),
}
impl Bounds {
    fn grow(&self, amount: i32) -> Self {
        return Self {
            x: (self.x.0 - amount, self.x.1 + amount),
            y: (self.y.0 - amount, self.y.1 + amount),
        };
    }

    fn to_points(&self) -> Vec<Point> {
        let yrange = (self.y.0)..(self.y.1 + 1);
        return yrange
            .into_iter()
            .flat_map::<Vec<Point>, _>(|y| {
                let xrange = (self.x.0)..(self.x.1 + 1);
                return xrange.into_iter().map(|x| Point(x, y)).collect();
            })
            .collect();
    }
}

type LitPoints = HashSet<Point>;

#[derive(new)]
struct State {
    points: LitPoints,
    bounds: Bounds,
    outside_bounds: bool,
}
impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.outside_bounds {
            write!(f, "Darkness all around me.")?;
        }
        for y in (self.bounds.y.0)..(self.bounds.y.1 + 1) {
            for x in (self.bounds.x.0)..(self.bounds.x.1 + 1) {
                if self.points.contains(&Point(x, y)) {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        return write!(f, "\n");
    }
}

fn parse_input(input: String) -> (Algorithm, State) {
    let mut parts = input.trim().splitn(2, "\n\n");
    let algorithm: Algorithm = parts
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap();
    let grid: Grid<bool> = parts
        .next()
        .unwrap()
        .trim()
        .split("\n")
        .map(str::trim)
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect();
    let bounds = Bounds::new((0, grid.width as i32 - 1), (0, grid.height as i32 - 1));
    let lit_points = grid
        .into_by_cell()
        .filter(|(_, v)| *v)
        .map(|(p, _)| Point(p.x as i32, p.y as i32))
        .collect();
    return (algorithm, State::new(lit_points, bounds, false));
}

fn do_step(algorithm: &Algorithm, mut state: State) -> State {
    if algorithm[0] {
        assert!(!algorithm[511]);
    }

    // Grow the bounds to make room for changes at the edges/outsides.
    let new_bounds = state.bounds.grow(1);

    // Fill the new space if all cells outside the bounds are on.
    if state.outside_bounds {
        let old_points = state.bounds.to_points();
        let new_points = state
            .bounds
            .grow(2)
            .to_points()
            .into_iter()
            .filter(|p| !old_points.contains(p));
        for point in new_points {
            state.points.insert(point);
        }
    }

    let mut new_lit_points = LitPoints::new();

    for point in new_bounds.to_points() {
        let mut idx = 0;
        let block_values = point.block().into_iter().map(|p| state.points.contains(&p));
        for (i, v) in block_values.enumerate() {
            if v {
                idx += 2usize.pow((8 - i) as u32);
            }
        }
        if algorithm[idx] {
            new_lit_points.insert(point);
        }
    }

    let new_outside_bounds = if state.outside_bounds {
        algorithm[511]
    } else {
        algorithm[0]
    };

    return State::new(new_lit_points, new_bounds, new_outside_bounds);
}

fn part1(input: String) -> usize {
    let (algorithm, mut state) = parse_input(input);

    state = do_step(&algorithm, state);
    state = do_step(&algorithm, state);
    return state.points.len();
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // This example is garbage since its avoids the core challenge, but its what we were given. Bah.
    const EXAMPLE_INPUT: &'static str = "
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

        #..#.
        #....
        ##..#
        ..#..
        ..###
    ";

    #[test]
    fn example_parse() {
        let (actual_algorithm, actual_state) = parse_input(EXAMPLE_INPUT.to_string());
        let expected_algorithm = [
            false, false, true, false, true, false, false, true, true, true, true, true, false,
            true, false, true, false, true, false, true, true, true, false, true, true, false,
            false, false, false, false, true, true, true, false, true, true, false, true, false,
            false, true, true, true, false, true, true, true, true, false, false, true, true, true,
            true, true, false, false, true, false, false, false, false, true, false, false, true,
            false, false, true, true, false, false, true, true, true, false, false, true, true,
            true, true, true, true, false, true, true, true, false, false, false, true, true, true,
            true, false, false, true, false, false, true, true, true, true, true, false, false,
            true, true, false, false, true, false, true, true, true, true, true, false, false,
            false, true, true, false, true, false, true, false, false, true, false, true, true,
            false, false, true, false, true, false, false, false, false, false, false, true, false,
            true, true, true, false, true, true, true, true, true, true, false, true, true, true,
            false, true, true, true, true, false, false, false, true, false, true, true, false,
            true, true, false, false, true, false, false, true, false, false, true, true, true,
            true, true, false, false, false, false, false, true, false, true, false, false, false,
            false, true, true, true, false, false, true, false, true, true, false, false, false,
            false, false, false, true, false, false, false, false, false, true, false, false, true,
            false, false, true, false, false, true, true, false, false, true, false, false, false,
            true, true, false, true, true, true, true, true, true, false, true, true, true, true,
            false, true, true, true, true, false, true, false, true, false, false, false, true,
            false, false, false, false, false, false, false, true, false, false, true, false, true,
            false, true, false, false, false, true, true, true, true, false, true, true, false,
            true, false, false, false, false, false, false, true, false, false, true, false, false,
            false, true, true, false, true, false, true, true, false, false, true, false, false,
            false, true, true, false, true, false, true, true, false, false, true, true, true,
            false, true, false, false, false, false, false, false, true, false, true, false, false,
            false, false, false, false, false, true, false, true, false, true, false, true, true,
            true, true, false, true, true, true, false, true, true, false, false, false, true,
            false, false, false, false, false, true, true, true, true, false, true, false, false,
            true, false, false, true, false, true, true, false, true, false, false, false, false,
            true, true, false, false, true, false, true, true, true, true, false, false, false,
            false, true, true, false, false, false, true, true, false, false, true, false, false,
            false, true, false, false, false, false, false, false, true, false, true, false, false,
            false, false, false, false, false, true, false, false, false, false, false, false,
            false, true, true, false, false, true, true, true, true, false, false, true, false,
            false, false, true, false, true, false, true, false, false, false, true, true, false,
            false, true, false, true, false, false, true, true, true, false, false, true, true,
            true, true, true, false, false, false, false, false, false, false, false, true, false,
            false, true, true, true, true, false, false, false, false, false, false, true, false,
            false, true,
        ];
        let expected_lit_points = vec![
            Point(0, 0),
            Point(0, 1),
            Point(0, 2),
            Point(1, 2),
            Point(2, 3),
            Point(2, 4),
            Point(3, 0),
            Point(3, 4),
            Point(4, 2),
            Point(4, 4),
        ]
        .into_iter()
        .collect();
        assert_eq!(actual_algorithm, expected_algorithm);
        assert_eq!(actual_state.points, expected_lit_points);
        assert_eq!(actual_state.bounds, Bounds::new((0, 4), (0, 4)));
    }

    #[test]
    fn example_do_step() {
        let algorithm = [
            false, false, true, false, true, false, false, true, true, true, true, true, false,
            true, false, true, false, true, false, true, true, true, false, true, true, false,
            false, false, false, false, true, true, true, false, true, true, false, true, false,
            false, true, true, true, false, true, true, true, true, false, false, true, true, true,
            true, true, false, false, true, false, false, false, false, true, false, false, true,
            false, false, true, true, false, false, true, true, true, false, false, true, true,
            true, true, true, true, false, true, true, true, false, false, false, true, true, true,
            true, false, false, true, false, false, true, true, true, true, true, false, false,
            true, true, false, false, true, false, true, true, true, true, true, false, false,
            false, true, true, false, true, false, true, false, false, true, false, true, true,
            false, false, true, false, true, false, false, false, false, false, false, true, false,
            true, true, true, false, true, true, true, true, true, true, false, true, true, true,
            false, true, true, true, true, false, false, false, true, false, true, true, false,
            true, true, false, false, true, false, false, true, false, false, true, true, true,
            true, true, false, false, false, false, false, true, false, true, false, false, false,
            false, true, true, true, false, false, true, false, true, true, false, false, false,
            false, false, false, true, false, false, false, false, false, true, false, false, true,
            false, false, true, false, false, true, true, false, false, true, false, false, false,
            true, true, false, true, true, true, true, true, true, false, true, true, true, true,
            false, true, true, true, true, false, true, false, true, false, false, false, true,
            false, false, false, false, false, false, false, true, false, false, true, false, true,
            false, true, false, false, false, true, true, true, true, false, true, true, false,
            true, false, false, false, false, false, false, true, false, false, true, false, false,
            false, true, true, false, true, false, true, true, false, false, true, false, false,
            false, true, true, false, true, false, true, true, false, false, true, true, true,
            false, true, false, false, false, false, false, false, true, false, true, false, false,
            false, false, false, false, false, true, false, true, false, true, false, true, true,
            true, true, false, true, true, true, false, true, true, false, false, false, true,
            false, false, false, false, false, true, true, true, true, false, true, false, false,
            true, false, false, true, false, true, true, false, true, false, false, false, false,
            true, true, false, false, true, false, true, true, true, true, false, false, false,
            false, true, true, false, false, false, true, true, false, false, true, false, false,
            false, true, false, false, false, false, false, false, true, false, true, false, false,
            false, false, false, false, false, true, false, false, false, false, false, false,
            false, true, true, false, false, true, true, true, true, false, false, true, false,
            false, false, true, false, true, false, true, false, false, false, true, true, false,
            false, true, false, true, false, false, true, true, true, false, false, true, true,
            true, true, true, false, false, false, false, false, false, false, false, true, false,
            false, true, true, true, true, false, false, false, false, false, false, true, false,
            false, true,
        ];
        let mut state = State::new(
            vec![
                Point(0, 0),
                Point(0, 1),
                Point(0, 2),
                Point(1, 2),
                Point(2, 3),
                Point(2, 4),
                Point(3, 0),
                Point(3, 4),
                Point(4, 2),
                Point(4, 4),
            ]
            .into_iter()
            .collect(),
            Bounds::new((0, 4), (0, 4)),
            false,
        );

        state = do_step(&algorithm, state);
        assert_eq!(
            state.points,
            vec![
                Point(-1, 0),
                Point(-1, 1),
                Point(-1, 2),
                Point(0, -1),
                Point(0, 1),
                Point(0, 2),
                Point(0, 3),
                Point(1, -1),
                Point(1, 2),
                Point(1, 4),
                Point(2, 0),
                Point(2, 1),
                Point(2, 2),
                Point(2, 4),
                Point(2, 5),
                Point(3, -1),
                Point(3, 3),
                Point(4, -1),
                Point(4, 0),
                Point(4, 3),
                Point(4, 5),
                Point(5, 1),
                Point(5, 2),
                Point(5, 4),
            ]
            .into_iter()
            .collect()
        );
        assert!(!state.outside_bounds);

        state = do_step(&algorithm, state);
        assert_eq!(
            state.points,
            vec![
                Point(5, -2),
                Point(-1, -1),
                Point(2, -1),
                Point(4, -1),
                Point(-2, 0),
                Point(0, 0),
                Point(4, 0),
                Point(5, 0),
                Point(6, 0),
                Point(-2, 1),
                Point(2, 1),
                Point(3, 1),
                Point(5, 1),
                Point(-2, 2),
                Point(4, 2),
                Point(6, 2),
                Point(-1, 3),
                Point(1, 3),
                Point(2, 3),
                Point(3, 3),
                Point(4, 3),
                Point(5, 3),
                Point(0, 4),
                Point(2, 4),
                Point(3, 4),
                Point(4, 4),
                Point(5, 4),
                Point(6, 4),
                Point(1, 5),
                Point(2, 5),
                Point(4, 5),
                Point(5, 5),
                Point(2, 6),
                Point(3, 6),
                Point(4, 6),
            ]
            .into_iter()
            .collect()
        );
        assert!(!state.outside_bounds);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 35);
    }
}
