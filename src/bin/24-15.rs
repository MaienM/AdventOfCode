use std::mem;

use aoc::utils::{
    parse,
    point::{Direction2, Point2},
};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Obstacle,
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
enum TileDouble {
    Wall,
    ObstacleLeft,
    ObstacleRight,
    Empty,
}

type Map = Vec<Vec<Tile>>;
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

fn parse_input(input: &str) -> (Map, Moves, Point2) {
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
                    '#' => Tile::Wall,
                    'O' => Tile::Obstacle,
                    '.' => Tile::Empty,
                    '@' => {
                        start = Some(Point2::new(x, y));
                        Tile::Empty
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
            Tile::Wall => continue,
            Tile::Obstacle => {
                let mut check = next + mov;
                loop {
                    match map[check.y][check.x] {
                        Tile::Wall => break,
                        Tile::Obstacle => {
                            check += mov;
                        }
                        Tile::Empty => {
                            map[check.y][check.x] = Tile::Obstacle;
                            map[next.y][next.x] = Tile::Empty;
                            current = next;
                            break;
                        }
                    }
                }
            }
            Tile::Empty => {
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
                    if tile == Tile::Obstacle {
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
                    TileDouble::ObstacleLeft => '[',
                    TileDouble::ObstacleRight => ']',
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
                    Tile::Wall => [TileDouble::Wall, TileDouble::Wall],
                    Tile::Obstacle => [TileDouble::ObstacleLeft, TileDouble::ObstacleRight],
                    Tile::Empty => [TileDouble::Empty, TileDouble::Empty],
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    current = Point2::new(current.x * 2, current.y);
    'main: for mov in moves {
        let next = current + mov;
        match map[next.y][next.x] {
            TileDouble::Wall => continue,
            TileDouble::ObstacleLeft | TileDouble::ObstacleRight => match mov {
                Direction2::North | Direction2::South => {
                    let mut checks = vec![if map[next.y][next.x] == TileDouble::ObstacleLeft {
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
                                map[next.y][next.x] = TileDouble::ObstacleLeft;
                                map[next.y][next.x + 1] = TileDouble::ObstacleRight;
                            }
                            current = next;
                            continue 'main;
                        }

                        let mut nextchecks = Vec::new();
                        'checks: for check in checks {
                            let next = check + mov;
                            match map[next.y][next.x] {
                                TileDouble::Empty => {}
                                TileDouble::ObstacleLeft => {
                                    nextchecks.push(next);
                                    continue 'checks;
                                }
                                TileDouble::ObstacleRight => {
                                    nextchecks.push(next + Direction2::West);
                                }
                                TileDouble::Wall => {
                                    continue 'main;
                                }
                            }
                            match map[next.y][next.x + 1] {
                                TileDouble::Empty => {}
                                TileDouble::ObstacleLeft => {
                                    nextchecks.push(next + Direction2::East);
                                }
                                TileDouble::ObstacleRight => {}
                                TileDouble::Wall => {
                                    continue 'main;
                                }
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
                            TileDouble::ObstacleLeft | TileDouble::ObstacleRight => {
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
                    if tile == TileDouble::ObstacleLeft {
                        y * 100 + x
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

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

    // #[test]
    // fn example_parse() {
    //     let actual = parse_input(&EXAMPLE_INPUT);
    //     let expected = (
    //         hash_set![
    //         ],
    //     );
    //     assert_eq!(actual, expected);
    // }
}
