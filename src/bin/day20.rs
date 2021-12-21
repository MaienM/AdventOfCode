use std::fmt::Debug;
use std::ops::RangeInclusive;

use aoc::grid::Grid;
use aoc::runner::*;
use derive_new::new;

type Algorithm = [bool; 512];

#[derive(Debug, Eq, PartialEq)]
struct LitPoints<const SIZE: usize>([[bool; SIZE]; SIZE]);
impl<const SIZE: usize> LitPoints<SIZE> {
    fn new() -> Self {
        return Self([[false; SIZE]; SIZE]);
    }

    fn get(&self, x: usize, y: usize) -> bool {
        return self.0[y][x];
    }
    fn set(&mut self, x: usize, y: usize, value: bool) {
        self.0[y][x] = value;
    }

    fn get_block(&self, x: usize, y: usize) -> [bool; 9] {
        return [
            self.get(x - 1, y - 1),
            self.get(x, y - 1),
            self.get(x + 1, y - 1),
            self.get(x - 1, y),
            self.get(x, y),
            self.get(x + 1, y),
            self.get(x - 1, y + 1),
            self.get(x, y + 1),
            self.get(x + 1, y + 1),
        ];
    }

    fn count_lit(&self) -> usize {
        let mut count = 0;
        for row in self.0 {
            for cell in row {
                if cell {
                    count += 1;
                }
            }
        }
        return count;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Point(usize, usize);

#[derive(Debug, Eq, PartialEq, new)]
struct Bounds {
    x: (usize, usize),
    y: (usize, usize),
}
impl Bounds {
    fn grow(&self, amount: usize) -> Self {
        return Self {
            x: (self.x.0 - amount, self.x.1 + amount),
            y: (self.y.0 - amount, self.y.1 + amount),
        };
    }

    fn xrange(&self) -> RangeInclusive<usize> {
        return (self.x.0)..=(self.x.1);
    }

    fn yrange(&self) -> RangeInclusive<usize> {
        return (self.y.0)..=(self.y.1);
    }
}

#[derive(new)]
struct State<const SIZE: usize> {
    points: LitPoints<SIZE>,
    bounds: Bounds,
    outside_bounds: bool,
}
impl<const SIZE: usize> Debug for State<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.outside_bounds {
            write!(f, "Darkness all around me.")?;
        }
        for y in (self.bounds.y.0)..=(self.bounds.y.1) {
            for x in (self.bounds.x.0)..=(self.bounds.x.1) {
                if self.points.get(x, y) {
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

fn parse_input<const SIZE: usize>(input: String) -> (Algorithm, State<SIZE>) {
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

    let xoffset_point = (SIZE - grid.width) / 2;
    let yoffset_point = (SIZE - grid.height) / 2;
    let bounds = Bounds::new(
        (xoffset_point, xoffset_point + grid.width as usize - 1),
        (yoffset_point, yoffset_point + grid.height as usize - 1),
    );
    let mut lit_points: LitPoints<SIZE> = LitPoints::new();
    for (point, value) in grid.into_by_cell() {
        lit_points.set(
            xoffset_point + point.x as usize,
            yoffset_point + point.y as usize,
            value,
        );
    }
    return (algorithm, State::new(lit_points, bounds, false));
}

fn do_step<const SIZE: usize>(algorithm: &Algorithm, mut state: State<SIZE>) -> State<SIZE> {
    if algorithm[0] {
        assert!(!algorithm[511]);
    }

    // Grow the bounds to make room for changes at the edges/outsides.
    let new_bounds = state.bounds.grow(1);

    // Fill the new space if all cells outside the bounds are on.
    if state.outside_bounds {
        let fill_bounds = new_bounds.grow(1);

        for x in fill_bounds.xrange() {
            state.points.set(x, fill_bounds.y.0, true);
            state.points.set(x, fill_bounds.y.0 + 1, true);
            state.points.set(x, fill_bounds.y.1 - 1, true);
            state.points.set(x, fill_bounds.y.1, true);
        }
        for y in fill_bounds.yrange() {
            state.points.set(fill_bounds.x.0, y, true);
            state.points.set(fill_bounds.x.0 + 1, y, true);
            state.points.set(fill_bounds.x.1 - 1, y, true);
            state.points.set(fill_bounds.x.1, y, true);
        }
    }

    let mut new_lit_points: LitPoints<SIZE> = LitPoints::new();

    for x in new_bounds.xrange() {
        for y in new_bounds.yrange() {
            let mut idx = 0;
            let block_values = state.points.get_block(x, y);
            for (i, v) in block_values.into_iter().enumerate() {
                if v {
                    idx += 2usize.pow((8 - i) as u32);
                }
            }
            if algorithm[idx] {
                new_lit_points.set(x, y, true);
            }
        }
    }

    let new_outside_bounds = if state.outside_bounds {
        algorithm[511]
    } else {
        algorithm[0]
    };

    return State::new(new_lit_points, new_bounds, new_outside_bounds);
}

pub fn part1(input: String) -> usize {
    let (algorithm, mut state) = parse_input::<108>(input);
    for _ in 0..2 {
        state = do_step(&algorithm, state);
    }
    return state.points.count_lit();
}

pub fn part2(input: String) -> usize {
    let (algorithm, mut state) = parse_input::<300>(input);
    for _ in 0..50 {
        state = do_step(&algorithm, state);
    }
    return state.points.count_lit();
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

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
        let (actual_algorithm, actual_state) = parse_input::<5>(EXAMPLE_INPUT.to_string());
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
        let expected_lit_points = LitPoints([
            [true, false, false, true, false],
            [true, false, false, false, false],
            [true, true, false, false, true],
            [false, false, true, false, false],
            [false, false, true, true, true],
        ]);
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
            LitPoints([
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, true, false, false, true, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, true, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, true, true, false, false, true, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, true, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, true, true, true, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
            ]),
            Bounds::new((4, 8), (4, 8)),
            false,
        );

        state = do_step(&algorithm, state);
        assert_eq!(
            LitPoints([
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, true, true, false, true, true, false, false, false,
                    false,
                ],
                [
                    false, false, false, true, false, false, true, false, true, false, false,
                    false, false,
                ],
                [
                    false, false, false, true, true, false, true, false, false, true, false, false,
                    false,
                ],
                [
                    false, false, false, true, true, true, true, false, false, true, false, false,
                    false,
                ],
                [
                    false, false, false, false, true, false, false, true, true, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, true, true, false, false, true, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, true, false, true, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
            ]),
            state.points,
        );
        assert!(!state.outside_bounds);

        state = do_step(&algorithm, state);
        assert_eq!(
            state.points,
            LitPoints([
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, true, false,
                    false, false,
                ],
                [
                    false, false, false, true, false, false, true, false, true, false, false,
                    false, false,
                ],
                [
                    false, false, true, false, true, false, false, false, true, true, true, false,
                    false,
                ],
                [
                    false, false, true, false, false, false, true, true, false, true, false, false,
                    false,
                ],
                [
                    false, false, true, false, false, false, false, false, true, false, true,
                    false, false,
                ],
                [
                    false, false, false, true, false, true, true, true, true, true, false, false,
                    false,
                ],
                [
                    false, false, false, false, true, false, true, true, true, true, true, false,
                    false,
                ],
                [
                    false, false, false, false, false, true, true, false, true, true, false, false,
                    false,
                ],
                [
                    false, false, false, false, false, false, true, true, true, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
                [
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false,
                ],
            ])
        );
        assert!(!state.outside_bounds);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 35);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 3351);
    }
}
