aoc::setup!(title = "Warehouse Woes");

use aoc::point::{Direction2, Point2};

#[derive(Debug, PartialEq, Eq)]
enum TileSingle {
    Wall,
    Box,
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
enum TileDouble {
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
}

type MapSingle = Vec<Vec<TileSingle>>;
type MapDouble = Vec<Vec<TileDouble>>;
type Moves = Vec<Direction2>;

fn parse_move(chr: char) -> Option<Direction2> {
    match chr {
        '^' => Some(Direction2::North),
        '>' => Some(Direction2::East),
        'v' => Some(Direction2::South),
        '<' => Some(Direction2::West),
        _ => None,
    }
}

fn parse_input(input: &str) -> (MapSingle, Moves, Point2) {
    parse!(input =>
        map
        "\n\n"
        moves
    );

    let mut start = None;
    let map = map
        .split('\n')
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, c)| match c {
                    '#' => TileSingle::Wall,
                    'O' => TileSingle::Box,
                    '.' => TileSingle::Empty,
                    '@' => {
                        start = Some(Point2::new(x, y));
                        TileSingle::Empty
                    }
                    _ => panic!(),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let moves = moves.chars().filter_map(parse_move).collect();

    (map, moves, start.unwrap())
}

pub fn part1(input: &str) -> usize {
    let (mut map, moves, mut current) = parse_input(input);
    for mov in moves {
        let next = current + mov;
        match map[next.y][next.x] {
            TileSingle::Wall => continue,
            TileSingle::Box => {
                let mut check = next + mov;
                loop {
                    match map[check.y][check.x] {
                        TileSingle::Wall => break,
                        TileSingle::Box => {
                            check += mov;
                        }
                        TileSingle::Empty => {
                            map[check.y][check.x] = TileSingle::Box;
                            map[next.y][next.x] = TileSingle::Empty;
                            current = next;
                            break;
                        }
                    }
                }
            }
            TileSingle::Empty => {
                current = next;
            }
        }
    }
    map.into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, tile)| {
                    if tile == TileSingle::Box {
                        y * 100 + x
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

#[allow(unused)]
fn print(map: &MapDouble, current: &Point2) {
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            print!(
                "{}",
                match tile {
                    TileDouble::Wall => '#',
                    TileDouble::BoxLeft => '[',
                    TileDouble::BoxRight => ']',
                    TileDouble::Empty => {
                        if x == current.x && y == current.y {
                            '@'
                        } else {
                            '.'
                        }
                    }
                }
            );
        }
        println!();
    }
}

#[allow(clippy::too_many_lines)]
pub fn part2(input: &str) -> usize {
    let (map, moves, mut current) = parse_input(input);
    let mut map = map
        .into_iter()
        .map(|line| {
            line.into_iter()
                .flat_map(|tile| match tile {
                    TileSingle::Wall => [TileDouble::Wall, TileDouble::Wall],
                    TileSingle::Box => [TileDouble::BoxLeft, TileDouble::BoxRight],
                    TileSingle::Empty => [TileDouble::Empty, TileDouble::Empty],
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    current = Point2::new(current.x * 2, current.y);
    'main: for mov in moves {
        let next = current + mov;
        match map[next.y][next.x] {
            TileDouble::Wall => continue,
            TileDouble::BoxLeft | TileDouble::BoxRight => match mov {
                Direction2::North | Direction2::South => {
                    let mut checks = vec![if map[next.y][next.x] == TileDouble::BoxLeft {
                        next
                    } else {
                        next + Direction2::West
                    }];
                    let mut boxes = checks.clone();

                    loop {
                        if checks.is_empty() {
                            boxes.reverse();
                            for point in boxes {
                                let next = point + mov;
                                map[point.y][point.x] = TileDouble::Empty;
                                map[point.y][point.x + 1] = TileDouble::Empty;
                                map[next.y][next.x] = TileDouble::BoxLeft;
                                map[next.y][next.x + 1] = TileDouble::BoxRight;
                            }
                            current = next;
                            continue 'main;
                        }

                        let mut nextchecks = Vec::new();
                        'checks: for check in checks {
                            let next = check + mov;
                            match map[next.y][next.x] {
                                TileDouble::Empty => {}
                                TileDouble::BoxLeft => {
                                    nextchecks.push(next);
                                    continue 'checks;
                                }
                                TileDouble::BoxRight => {
                                    nextchecks.push(next + Direction2::West);
                                }
                                TileDouble::Wall => {
                                    continue 'main;
                                }
                            }
                            match map[next.y][next.x + 1] {
                                TileDouble::BoxLeft => {
                                    nextchecks.push(next + Direction2::East);
                                }
                                TileDouble::Wall => {
                                    continue 'main;
                                }
                                _ => {}
                            }
                        }
                        boxes.extend(nextchecks.clone());
                        checks = nextchecks;
                    }
                }
                Direction2::East | Direction2::West => {
                    let mut check = next + mov;
                    loop {
                        match map[check.y][check.x] {
                            TileDouble::Wall => break,
                            TileDouble::BoxLeft | TileDouble::BoxRight => {
                                check += mov;
                            }
                            TileDouble::Empty => {
                                if mov == Direction2::West {
                                    map[check.y].insert(next.x + 1, TileDouble::Empty);
                                    map[check.y].remove(check.x);
                                } else {
                                    map[check.y].insert(next.x, TileDouble::Empty);
                                    map[check.y].remove(check.x + 1);
                                }
                                current = next;
                                break;
                            }
                        }
                    }
                }
            },
            TileDouble::Empty => {
                current = next;
            }
        }
    }
    map.into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(|(x, tile)| {
                    if tile == TileDouble::BoxLeft {
                        y * 100 + x
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 10_092, part2 = 9021)]
    static EXAMPLE_INPUT_1: &str = "
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    ";

    #[example_input(part1 = 2028)]
    static EXAMPLE_INPUT_2: &str = "
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
    ";

    #[allow(clippy::too_many_lines)]
    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT_2);
        let expected = (
            vec![
                vec![
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                ],
                vec![
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Box,
                    TileSingle::Empty,
                    TileSingle::Box,
                    TileSingle::Empty,
                    TileSingle::Wall,
                ],
                vec![
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Box,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Wall,
                ],
                vec![
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Box,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Wall,
                ],
                vec![
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Box,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Wall,
                ],
                vec![
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Box,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Wall,
                ],
                vec![
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Wall,
                ],
                vec![
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                ],
            ],
            vec![
                Direction2::West,
                Direction2::North,
                Direction2::North,
                Direction2::East,
                Direction2::East,
                Direction2::East,
                Direction2::South,
                Direction2::South,
                Direction2::West,
                Direction2::South,
                Direction2::East,
                Direction2::East,
                Direction2::South,
                Direction2::West,
                Direction2::West,
            ],
            Point2::new(2, 2),
        );
        assert_eq!(actual, expected);
    }
}
