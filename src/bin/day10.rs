use aoc::point::Point2;

type Point = Point2<usize>;

#[derive(Debug, PartialEq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
    None,
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            'S' => Tile::Start,
            '.' => Tile::None,
            _ => panic!("Unknown character {value:?}."),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .split('\n')
        .map(|line| line.chars().map(Tile::from).collect())
        .collect()
}

pub fn part1(input: &str) -> usize {
    let map = parse_input(input);
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, tile)| {
                if tile == &Tile::Start {
                    Some(Point::new(x, y))
                } else {
                    None
                }
            })
        })
        .unwrap();
    let mut len = 1;
    let mut prev = start;
    let mut curr = start
        .neighbours_ortho()
        .into_iter()
        .find_map(|point| {
            let tile = &map[point.y][point.x];
            match tile {
                Tile::Vertical if point.y != start.y => {}
                Tile::Horizontal if point.x != start.x => {}
                Tile::NorthEast if point.y > start.y || point.x < start.x => {}
                Tile::NorthWest if point.y > start.y || point.x > start.x => {}
                Tile::SouthEast if point.y < start.y || point.x < start.x => {}
                Tile::SouthWest if point.y < start.y || point.x > start.x => {}
                _ => return None,
            };
            Some((point, tile))
        })
        .unwrap();
    while curr.0 != start {
        let (point, tile) = curr;
        let next = match tile {
            Tile::Vertical => {
                if prev.y < point.y {
                    Point::new(point.x, point.y + 1)
                } else {
                    Point::new(point.x, point.y - 1)
                }
            }
            Tile::Horizontal => {
                if prev.x < point.x {
                    Point::new(point.x + 1, point.y)
                } else {
                    Point::new(point.x - 1, point.y)
                }
            }
            Tile::NorthEast => {
                if prev.x == point.x {
                    Point::new(point.x + 1, point.y)
                } else {
                    Point::new(point.x, point.y - 1)
                }
            }
            Tile::NorthWest => {
                if prev.x == point.x {
                    Point::new(point.x - 1, point.y)
                } else {
                    Point::new(point.x, point.y - 1)
                }
            }
            Tile::SouthEast => {
                if prev.x == point.x {
                    Point::new(point.x + 1, point.y)
                } else {
                    Point::new(point.x, point.y + 1)
                }
            }
            Tile::SouthWest => {
                if prev.x == point.x {
                    Point::new(point.x - 1, point.y)
                } else {
                    Point::new(point.x, point.y + 1)
                }
            }
            _ => panic!("Ended up on {tile:?} at {point:?}, cannot proceed."),
        };
        prev = point;
        curr = (next, &map[next.y][next.x]);
        len += 1;
    }
    len / 2
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 4, test)]
    static EXAMPLE_INPUT_1: &str = "
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
    ";

    #[example_input(part1 = 8, test)]
    static EXAMPLE_INPUT_2: &str = "
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
    ";

    #[test]
    fn example_parse_1() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = vec![
            vec![Tile::None, Tile::None, Tile::None, Tile::None, Tile::None],
            vec![
                Tile::None,
                Tile::Start,
                Tile::Horizontal,
                Tile::SouthWest,
                Tile::None,
            ],
            vec![
                Tile::None,
                Tile::Vertical,
                Tile::None,
                Tile::Vertical,
                Tile::None,
            ],
            vec![
                Tile::None,
                Tile::NorthEast,
                Tile::Horizontal,
                Tile::NorthWest,
                Tile::None,
            ],
            vec![Tile::None, Tile::None, Tile::None, Tile::None, Tile::None],
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_2() {
        let actual = parse_input(&EXAMPLE_INPUT_2);
        let expected = vec![
            vec![
                Tile::None,
                Tile::None,
                Tile::SouthEast,
                Tile::SouthWest,
                Tile::None,
            ],
            vec![
                Tile::None,
                Tile::SouthEast,
                Tile::NorthWest,
                Tile::Vertical,
                Tile::None,
            ],
            vec![
                Tile::Start,
                Tile::NorthWest,
                Tile::None,
                Tile::NorthEast,
                Tile::SouthWest,
            ],
            vec![
                Tile::Vertical,
                Tile::SouthEast,
                Tile::Horizontal,
                Tile::Horizontal,
                Tile::NorthWest,
            ],
            vec![
                Tile::NorthEast,
                Tile::NorthWest,
                Tile::None,
                Tile::None,
                Tile::None,
            ],
        ];
        assert_eq!(actual, expected);
    }
}
