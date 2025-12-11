puzzle_runner::register_chapter!(book = 2024, title = "Warehouse Woes");

use puzzle_lib::{
    grid::FullGrid,
    point::{Direction2, Point2},
};

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

type GridSingle = FullGrid<TileSingle>;
type GridDouble = FullGrid<TileDouble>;
type Moves = Vec<Direction2>;

fn parse_input(input: &str) -> (GridSingle, Moves, Point2) {
    parse!(input => {
        [grid cells match {
            '#' => TileSingle::Wall,
            'O' => TileSingle::Box,
            '.' => TileSingle::Empty,
            '@' => index into start => TileSingle::Empty,
        }]
        "\n\n"
        [moves chars try match {
            '^' => Some(Direction2::North),
            '>' => Some(Direction2::East),
            'v' => Some(Direction2::South),
            '<' => Some(Direction2::West),
            '\n' => None,
        }]
    } => (grid, moves, start))
}

#[register_part]
fn part1(input: &str) -> usize {
    let (mut grid, moves, mut current) = parse_input(input);
    for mov in moves {
        let next = current + mov;
        match grid[next] {
            TileSingle::Wall => {}
            TileSingle::Box => {
                let mut check = next + mov;
                loop {
                    match grid[check] {
                        TileSingle::Wall => break,
                        TileSingle::Box => {
                            check += mov;
                        }
                        TileSingle::Empty => {
                            grid[check] = TileSingle::Box;
                            grid[next] = TileSingle::Empty;
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
    grid.into_iter_pairs()
        .map(|(point, tile)| {
            if tile == TileSingle::Box {
                point.x + point.y * 100
            } else {
                0
            }
        })
        .sum()
}

#[allow(unused)]
fn print(grid: &GridDouble, current: &Point2) {
    for (y, row) in grid.iter_rows().enumerate() {
        for (x, tile) in row.enumerate() {
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
#[register_part]
fn part2(input: &str) -> usize {
    let (grid, moves, mut current) = parse_input(input);
    let mut grid: GridDouble = grid
        .into_iter_rows()
        .map(|row| {
            row.into_iter().flat_map(|tile| match tile {
                TileSingle::Wall => [TileDouble::Wall, TileDouble::Wall],
                TileSingle::Box => [TileDouble::BoxLeft, TileDouble::BoxRight],
                TileSingle::Empty => [TileDouble::Empty, TileDouble::Empty],
            })
        })
        .collect();
    current = Point2::new(current.x * 2, current.y);
    'main: for mov in moves {
        let next = current + mov;
        match grid[next] {
            TileDouble::Wall => {}
            TileDouble::BoxLeft | TileDouble::BoxRight => match mov {
                Direction2::North | Direction2::South => {
                    let mut checks = vec![if grid[next] == TileDouble::BoxLeft {
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
                                grid[point] = TileDouble::Empty;
                                grid[point + Direction2::East] = TileDouble::Empty;
                                grid[next] = TileDouble::BoxLeft;
                                grid[next + Direction2::East] = TileDouble::BoxRight;
                            }
                            current = next;
                            continue 'main;
                        }

                        let mut nextchecks = Vec::new();
                        'checks: for check in checks {
                            let next = check + mov;
                            match grid[next] {
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
                            match grid[next + Direction2::East] {
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
                        match grid[check] {
                            TileDouble::Wall => break,
                            TileDouble::BoxLeft | TileDouble::BoxRight => {
                                check += mov;
                            }
                            TileDouble::Empty => {
                                let mut idx = next;
                                while idx != check {
                                    idx += mov;
                                    grid.swap(&idx, &next);
                                }
                                grid[next] = TileDouble::Empty;

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
    grid.into_iter_pairs()
        .map(|(point, tile)| {
            if tile == TileDouble::BoxLeft {
                point.x + point.y * 100
            } else {
                0
            }
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
            [
                [
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                ],
                [
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Box,
                    TileSingle::Empty,
                    TileSingle::Box,
                    TileSingle::Empty,
                    TileSingle::Wall,
                ],
                [
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Box,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Wall,
                ],
                [
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Box,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Wall,
                ],
                [
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Box,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Wall,
                ],
                [
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Box,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Wall,
                ],
                [
                    TileSingle::Wall,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Empty,
                    TileSingle::Wall,
                ],
                [
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                    TileSingle::Wall,
                ],
            ]
            .into(),
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
