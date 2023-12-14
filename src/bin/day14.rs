use aoc::utils::point::Point2;

type Point = Point2;

#[derive(Debug, PartialEq)]
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

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    input
        .split('\n')
        .map(|line| line.chars().map(Cell::from).collect())
        .collect()
}

fn slide_north(map: &mut [Vec<Cell>]) {
    let width = map[0].len();
    for x in 0..width {
        let mut rolling = 0;
        for y in (0..map.len()).rev() {
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
        for i in 0..rolling {
            map[i][x] = Cell::RoundRock;
        }
    }
}

fn calculate_load(map: Vec<Vec<Cell>>) -> usize {
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

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 136, test)]
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
}
