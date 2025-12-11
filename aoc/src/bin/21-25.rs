puzzle_runner::register_chapter!(book = 2021, title = "Sea Cucumber");

use std::hash::{DefaultHasher, Hash, Hasher};

use puzzle_lib::{
    grid::{FullGrid, WrappingGrid},
    point::{Direction2, Point2},
};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Tile {
    Empty,
    East,
    South,
}

type Grid = FullGrid<Tile>;

fn parse_input(input: &str) -> Grid {
    parse!(input => {
        [grid cells match {
            '.' => Tile::Empty,
            '>' => Tile::East,
            'v' => Tile::South,
        }]
    } => grid)
}

fn run<G>(grid: &mut G)
where
    G: PointDataCollection<Point2<usize>, Tile>,
{
    for (tile, direction) in [
        (Tile::East, Direction2::East),
        (Tile::South, Direction2::South),
    ] {
        let moving = grid
            .iter_pairs()
            .filter_map(|(p, t)| {
                if *t == tile && grid[*p + direction] == Tile::Empty {
                    Some(*p)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for point in moving {
            grid[point] = Tile::Empty;
            grid[point + direction] = tile;
        }
    }
}

#[register_part]
fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let mut grid = WrappingGrid::from(grid);

    let mut last_hash = 0;
    for i in 1.. {
        run(&mut grid);

        let mut hasher = DefaultHasher::new();
        grid.hash(&mut hasher);
        let hash = hasher.finish();

        if hash == last_hash {
            return i;
        }
        last_hash = hash;
    }
    panic!("Should never happen.");
}

#[register_part]
fn part2(_input: &str) -> &'static str {
    "I did it!"
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 58)]
    static EXAMPLE_INPUT: &str = "
        v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>
    ";

    #[allow(clippy::too_many_lines)]
    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = [
            [
                Tile::South,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::East,
                Tile::East,
                Tile::Empty,
                Tile::South,
                Tile::South,
                Tile::East,
            ],
            [
                Tile::Empty,
                Tile::South,
                Tile::South,
                Tile::East,
                Tile::East,
                Tile::Empty,
                Tile::South,
                Tile::South,
                Tile::Empty,
                Tile::Empty,
            ],
            [
                Tile::East,
                Tile::East,
                Tile::Empty,
                Tile::East,
                Tile::South,
                Tile::East,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::South,
            ],
            [
                Tile::East,
                Tile::East,
                Tile::South,
                Tile::East,
                Tile::East,
                Tile::Empty,
                Tile::East,
                Tile::Empty,
                Tile::South,
                Tile::Empty,
            ],
            [
                Tile::South,
                Tile::East,
                Tile::South,
                Tile::Empty,
                Tile::South,
                Tile::South,
                Tile::Empty,
                Tile::South,
                Tile::Empty,
                Tile::Empty,
            ],
            [
                Tile::East,
                Tile::Empty,
                Tile::East,
                Tile::East,
                Tile::Empty,
                Tile::Empty,
                Tile::South,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
            ],
            [
                Tile::Empty,
                Tile::South,
                Tile::South,
                Tile::Empty,
                Tile::Empty,
                Tile::East,
                Tile::Empty,
                Tile::East,
                Tile::South,
                Tile::Empty,
            ],
            [
                Tile::South,
                Tile::Empty,
                Tile::South,
                Tile::Empty,
                Tile::Empty,
                Tile::East,
                Tile::East,
                Tile::South,
                Tile::Empty,
                Tile::South,
            ],
            [
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::South,
                Tile::Empty,
                Tile::Empty,
                Tile::South,
                Tile::Empty,
                Tile::East,
            ],
        ]
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_run_small() {
        let parse = |v: &str| parse_input(&v.dedent());

        let mut grid = WrappingGrid::from(parse(
            "
                ...>...
                .......
                ......>
                v.....>
                ......>
                .......
                ..vvv..
            ",
        ));

        run(&mut grid);
        assert_eq!(
            *grid,
            parse(
                "
                    ..vv>..
                    .......
                    >......
                    v.....>
                    >......
                    .......
                    ....v..
                "
            )
        );

        run(&mut grid);
        assert_eq!(
            *grid,
            parse(
                "
                    ....v>.
                    ..vv...
                    .>.....
                    ......>
                    v>.....
                    .......
                    .......
                "
            )
        );

        run(&mut grid);
        assert_eq!(
            *grid,
            parse(
                "
                    ......>
                    ..v.v..
                    ..>v...
                    >......
                    ..>....
                    v......
                    .......
                "
            )
        );

        run(&mut grid);
        assert_eq!(
            *grid,
            parse(
                "
                    >......
                    ..v....
                    ..>.v..
                    .>.v...
                    ...>...
                    .......
                    v......
                "
            )
        );
    }
}
