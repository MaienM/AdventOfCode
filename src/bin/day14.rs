use std::collections::HashMap;

use aoc::utils::parse;

type Map = Vec<Vec<Cell>>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Cell {
    RoundRock,
    CubeRock,
    Empty,
}
impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'O' => Cell::RoundRock,
            '#' => Cell::CubeRock,
            '.' => Cell::Empty,
            _ => panic!("Invalid map cell {value:?}."),
        }
    }
}

fn parse_input(input: &str) -> Map {
    parse!(input => {
        [map split on '\n' with [chars as Cell]]
    } => map)
}

fn slide_north(map: &mut Map) {
    let width = map[0].len();
    for x in 0..width {
        let mut rolling = 0;
        for y in (0..map.len()).rev() {
            #[allow(clippy::match_on_vec_items)]
            match map[y][x] {
                Cell::RoundRock => {
                    rolling += 1;
                    map[y][x] = Cell::Empty;
                }
                Cell::CubeRock => {
                    for i in 0..rolling {
                        map[y + i + 1][x] = Cell::RoundRock;
                    }
                    rolling = 0;
                }
                Cell::Empty => {}
            }
        }
        for row in map.iter_mut().take(rolling) {
            row[x] = Cell::RoundRock;
        }
    }
}

fn slide_south(map: &mut Map) {
    let width = map[0].len();
    for x in 0..width {
        let mut rolling = 0;
        for y in 0..map.len() {
            #[allow(clippy::match_on_vec_items)]
            match map[y][x] {
                Cell::RoundRock => {
                    rolling += 1;
                    map[y][x] = Cell::Empty;
                }
                Cell::CubeRock => {
                    for i in 0..rolling {
                        map[y - i - 1][x] = Cell::RoundRock;
                    }
                    rolling = 0;
                }
                Cell::Empty => {}
            }
        }
        for row in map.iter_mut().rev().take(rolling) {
            row[x] = Cell::RoundRock;
        }
    }
}

fn slide_east(map: &mut Map) {
    let width = map[0].len();
    for row in map {
        let mut rolling = 0;
        for x in 0..width {
            #[allow(clippy::match_on_vec_items)]
            match row[x] {
                Cell::RoundRock => {
                    rolling += 1;
                    row[x] = Cell::Empty;
                }
                Cell::CubeRock => {
                    for i in 0..rolling {
                        row[x - i - 1] = Cell::RoundRock;
                    }
                    rolling = 0;
                }
                Cell::Empty => {}
            }
        }
        for cell in row.iter_mut().rev().take(rolling) {
            *cell = Cell::RoundRock;
        }
    }
}

fn slide_west(map: &mut Map) {
    let width = map[0].len();
    for row in map {
        let mut rolling = 0;
        for x in (0..width).rev() {
            #[allow(clippy::match_on_vec_items)]
            match row[x] {
                Cell::RoundRock => {
                    rolling += 1;
                    row[x] = Cell::Empty;
                }
                Cell::CubeRock => {
                    for i in 0..rolling {
                        row[x + i + 1] = Cell::RoundRock;
                    }
                    rolling = 0;
                }
                Cell::Empty => {}
            }
        }
        for cell in row.iter_mut().take(rolling) {
            *cell = Cell::RoundRock;
        }
    }
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    for line in map {
        for cell in line {
            match cell {
                Cell::RoundRock => print!("O"),
                Cell::CubeRock => print!("#"),
                Cell::Empty => print!("."),
            }
        }
        println!();
    }
    println!();
}

fn cycle(map: &mut Map) {
    slide_north(map);
    slide_west(map);
    slide_south(map);
    slide_east(map);
}

fn calculate_load(map: Map) -> usize {
    map.into_iter()
        .rev()
        .enumerate()
        .map(|(y, row)| {
            (y + 1)
                * row
                    .into_iter()
                    .filter(|cell| cell == &Cell::RoundRock)
                    .count()
        })
        .sum()
}

pub fn part1(input: &str) -> usize {
    let mut map = parse_input(input);
    slide_north(&mut map);
    calculate_load(map)
}

pub fn part2(input: &str) -> usize {
    let mut map = parse_input(input);
    let mut cache: HashMap<Map, usize> = HashMap::new();
    cache.insert(map.clone(), 0);
    let cycles = 1_000_000_000;
    for i in 0..cycles {
        cycle(&mut map);
        let Some(first) = cache.get(&map) else {
            cache.insert(map.clone(), i);
            continue;
        };
        let first = *first;

        // We're in a loop, so figure out which loop member we will end on and use that.
        let remaining = cycles - i - 1;
        let loop_size = i - first;
        let steps = remaining % loop_size;
        map = cache
            .into_iter()
            .find(|(_, iteration)| *iteration == first + steps)
            .unwrap()
            .0;
        break;
    }
    calculate_load(map)
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 136, part2 = 64, test)]
    static EXAMPLE_INPUT: &str = "
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
    ";

    #[test]
    #[allow(clippy::too_many_lines)]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            vec![
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::RoundRock,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            vec![
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
            ],
            vec![
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn slide_north() {
        let mut map = vec![
            vec![
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::RoundRock,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            vec![
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
            ],
            vec![
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        ];
        super::slide_north(&mut map);
        let expected = vec![
            vec![
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
            ],
            vec![
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            vec![
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::RoundRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        ];
        assert_eq!(map, expected);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn cycle() {
        let mut map = vec![
            vec![
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::RoundRock,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            vec![
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
            ],
            vec![
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::CubeRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        ];

        let expected = vec![
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
            ],
            vec![
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
        ];
        super::cycle(&mut map);
        assert_eq!(map, expected);

        let expected = vec![
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
            ],
            vec![
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::CubeRock,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
        ];
        super::cycle(&mut map);
        assert_eq!(map, expected);

        let expected = vec![
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
            ],
            vec![
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::CubeRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
            ],
            vec![
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::RoundRock,
            ],
            vec![
                Cell::CubeRock,
                Cell::Empty,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::RoundRock,
                Cell::CubeRock,
                Cell::Empty,
                Cell::Empty,
                Cell::Empty,
                Cell::RoundRock,
            ],
        ];
        super::cycle(&mut map);
        assert_eq!(map, expected);
    }
}
