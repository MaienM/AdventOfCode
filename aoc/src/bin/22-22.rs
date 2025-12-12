puzzle_runner::register_chapter!(book = 2022, title = "Monkey Map");

use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use puzzle_lib::point::Point2;

type Point = Point2;
type BlockPoint = Point2<isize>;

fn get_block(point: &Point, block_size: usize) -> BlockPoint {
    BlockPoint::new(
        (point.x / block_size) as isize,
        (point.y / block_size) as isize,
    )
}

// An important observation about the grid is that it is comprised of 6 square sections of the same
// size. To optimize this grid we store the size of these blocks and their relative coordinates,
// and these two bits are enough to figure out whether a given point is simply on the grid or
// requires wrapping.
#[derive(Debug, Eq, PartialEq)]
struct Grid {
    walls: HashSet<Point>,
    block_size: usize,
    blocks: [BlockPoint; 6],
}
impl Grid {
    fn get_starting_point(&self) -> Point {
        for x in (self.block_size * self.blocks[0].x as usize + 1)
            ..(self.block_size * (self.blocks[0].x as usize + 1))
        {
            let point = Point::new(x, 0);
            if !self.walls.contains(&point) {
                return point;
            }
        }
        never!();
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Action {
    Move(u8),
    TurnLeft,
    TurnRight,
}
type Actions = Vec<Action>;

fn parse_input(input: &str) -> (Grid, Actions) {
    parse!(input =>
        [grid split on '\n']
        "\n\n"
        path
    );

    let mut walls = HashSet::new();
    let block_size = grid.iter().map(|l| l.trim().len()).min().unwrap();
    let mut blocks = HashSet::new();
    for (y, line) in grid.into_iter().enumerate() {
        for (x, c) in line.char_indices() {
            if c == '#' {
                let point = Point::new(x, y);
                blocks.insert(get_block(&point, block_size));
                walls.insert(point);
            }
        }
    }
    let mut grid = Grid {
        walls,
        block_size,
        blocks: blocks
            .into_iter()
            .collect::<Vec<BlockPoint>>()
            .try_into()
            .unwrap(),
    };
    grid.blocks.sort_by_key(|block| block.x);
    grid.blocks.sort_by_key(|block| block.y);

    let mut actions = Vec::new();
    for c in path.trim().chars() {
        match c {
            'L' => actions.push(Action::TurnLeft),
            'R' => actions.push(Action::TurnRight),
            '0'..='9' => {
                let num = c.to_digit(10).unwrap() as u8;
                match actions.last_mut() {
                    Option::Some(Action::Move(prev)) => {
                        *prev = *prev * 10 + num;
                    }
                    _ => {
                        actions.push(Action::Move(num));
                    }
                }
            }
            _ => panic!("Invalid path char {c:?}"),
        }
    }

    (grid, actions)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Rotation {
    Zero = 0,
    Quarter = 90,
    Half = 180,
    ThreeQuarters = 270,
}
impl From<i16> for Rotation {
    fn from(value: i16) -> Self {
        match (value + 3600) % 360 {
            n if n == Rotation::Zero as i16 => Rotation::Zero,
            n if n == Rotation::Quarter as i16 => Rotation::Quarter,
            n if n == Rotation::Half as i16 => Rotation::Half,
            n if n == Rotation::ThreeQuarters as i16 => Rotation::ThreeQuarters,
            n => panic!("Invalid rotation {n}."),
        }
    }
}
impl Add for Rotation {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        (self as i16 + other as i16).into()
    }
}
impl Sub for Rotation {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        (self as i16 - other as i16).into()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up = 3,
    Down = 1,
    Left = 2,
    Right = 0,
}
impl Add<Rotation> for Direction {
    type Output = Self;
    fn add(self, other: Rotation) -> Self {
        match (self as u8 + (other as u16 / 90) as u8) % 4 {
            n if n == Direction::Up as u8 => Direction::Up,
            n if n == Direction::Down as u8 => Direction::Down,
            n if n == Direction::Left as u8 => Direction::Left,
            n if n == Direction::Right as u8 => Direction::Right,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Edge {
    block: BlockPoint,
    rotation: Rotation,
}

#[derive(Debug, Eq, PartialEq)]
struct Face {
    top: Edge,
    bottom: Edge,
    left: Edge,
    right: Edge,
}

type Directions = [Face; 6];

#[derive(Debug, Eq, PartialEq)]
struct State {
    grid: Grid,
    position: Point,
    direction: Direction,
    directions: Directions,
}
impl State {
    fn apply(&mut self, action: &Action) {
        match action {
            Action::Move(distance) => {
                for _ in 0..*distance {
                    if !self.move_() {
                        break;
                    }
                }
            }
            Action::TurnLeft => {
                self.direction = self.direction + Rotation::ThreeQuarters;
            }
            Action::TurnRight => {
                self.direction = self.direction + Rotation::Quarter;
            }
        }
    }

    fn get_current_directions(&self) -> &Face {
        let block = get_block(&self.position, self.grid.block_size);
        let idx = self
            .grid
            .blocks
            .iter()
            .enumerate()
            .find(|(_, point)| point == &&block)
            .unwrap()
            .0;
        &self.directions[idx]
    }

    fn wrap(&self, point: Point, edge: &Edge) -> (Point, Direction) {
        let block = self.grid.block_size;

        let offset = Point::new(point.x % block, point.y % block);
        let offset = match edge.rotation {
            Rotation::Zero => offset,
            Rotation::Quarter => Point::new(block - offset.y - 1, offset.x),
            Rotation::Half => Point::new(block - offset.x - 1, block - offset.y - 1),
            Rotation::ThreeQuarters => Point::new(offset.y, block - offset.x - 1),
        };
        let point = Point::new(
            edge.block.x as usize * block + offset.x,
            edge.block.y as usize * block + offset.y,
        );

        let direction = self.direction + edge.rotation;

        (point, direction)
    }

    fn move_(&mut self) -> bool {
        let block = self.grid.block_size;
        let (nextposition, nextdirection) = match self.direction {
            Direction::Up => {
                if self.position.y.is_multiple_of(block) {
                    self.wrap(
                        Point::new(self.position.x, self.position.y + block - 1),
                        &self.get_current_directions().top,
                    )
                } else {
                    (
                        Point::new(self.position.x, self.position.y - 1),
                        self.direction,
                    )
                }
            }
            Direction::Down => {
                if (self.position.y + 1).is_multiple_of(block) {
                    self.wrap(
                        Point::new(self.position.x, self.position.y + 1),
                        &self.get_current_directions().bottom,
                    )
                } else {
                    (
                        Point::new(self.position.x, self.position.y + 1),
                        self.direction,
                    )
                }
            }
            Direction::Left => {
                if self.position.x.is_multiple_of(block) {
                    self.wrap(
                        Point::new(self.position.x + block - 1, self.position.y),
                        &self.get_current_directions().left,
                    )
                } else {
                    (
                        Point::new(self.position.x - 1, self.position.y),
                        self.direction,
                    )
                }
            }
            Direction::Right => {
                if (self.position.x + 1).is_multiple_of(block) {
                    self.wrap(
                        Point::new(self.position.x + 1, self.position.y),
                        &self.get_current_directions().right,
                    )
                } else {
                    (
                        Point::new(self.position.x + 1, self.position.y),
                        self.direction,
                    )
                }
            }
        };
        if !self.grid.walls.contains(&nextposition) {
            self.position = nextposition;
            self.direction = nextdirection;
            return true;
        }
        false
    }
}

fn process(grid: Grid, actions: Actions, directions: Directions) -> usize {
    let mut state = State {
        position: grid.get_starting_point(),
        grid,
        direction: Direction::Right,
        directions,
    };
    for action in actions {
        state.apply(&action);
    }
    (state.position.y + 1) * 1000 + (state.position.x + 1) * 4 + (state.direction as usize)
}

fn map_faces_grid(grid: &Grid) -> Directions {
    let blocks = grid.blocks;
    blocks
        .iter()
        .map(|block| {
            let top = if block.y > 0 && blocks.contains(&BlockPoint::new(block.x, block.y - 1)) {
                BlockPoint::new(block.x, block.y - 1)
            } else {
                *blocks
                    .iter()
                    .filter(|b| b.x == block.x)
                    .max_by_key(|b| b.y)
                    .unwrap()
            };
            let bottom = if blocks.contains(&BlockPoint::new(block.x, block.y + 1)) {
                BlockPoint::new(block.x, block.y + 1)
            } else {
                *blocks
                    .iter()
                    .filter(|b| b.x == block.x)
                    .min_by_key(|b| b.y)
                    .unwrap()
            };
            let left = if block.x > 0 && blocks.contains(&BlockPoint::new(block.x - 1, block.y)) {
                BlockPoint::new(block.x - 1, block.y)
            } else {
                *blocks
                    .iter()
                    .filter(|b| b.y == block.y)
                    .max_by_key(|b| b.x)
                    .unwrap()
            };
            let right = if blocks.contains(&BlockPoint::new(block.x + 1, block.y)) {
                BlockPoint::new(block.x + 1, block.y)
            } else {
                *blocks
                    .iter()
                    .filter(|b| b.y == block.y)
                    .min_by_key(|b| b.x)
                    .unwrap()
            };
            Face {
                top: Edge {
                    block: top,
                    rotation: Rotation::Zero,
                },
                bottom: Edge {
                    block: bottom,
                    rotation: Rotation::Zero,
                },
                left: Edge {
                    block: left,
                    rotation: Rotation::Zero,
                },
                right: Edge {
                    block: right,
                    rotation: Rotation::Zero,
                },
            }
        })
        .collect::<Vec<Face>>()
        .try_into()
        .unwrap()
}

// Paths that can be taken to get to the face that is on the top edge of the starting face, and the
// rotation that this results in. For every path these also exists one that is mirrored along the X
// axis which has a rotation that turns the opposite direction; this is handled in find_cube_edge.
const N: (isize, isize) = (0, 0);
const GRID_PATHS: [([(isize, isize); 5], Rotation); 13] = [
    ([N, N, N, N, (0, -1)], Rotation::Zero),
    ([N, N, (0, 1), (0, 2), (0, 3)], Rotation::Zero),
    ([N, N, N, (1, 0), (1, -1)], Rotation::Quarter),
    ([N, N, (1, 0), (2, 0), (2, -1)], Rotation::Half),
    ([N, N, (-1, 0), (-2, 0), (-2, -1)], Rotation::Half),
    ([N, (-1, 0), (-2, 0), (-3, 0), (-3, -1)], Rotation::Quarter),
    ([N, (-1, 0), (-1, 1), (-2, 1), (-3, 1)], Rotation::Quarter),
    ([N, N, (0, 1), (1, 1), (2, 1)], Rotation::Half),
    ([N, (0, 1), (0, 2), (-1, 2), (-1, 3)], Rotation::Quarter),
    ([N, (0, 1), (-1, 1), (-1, 2), (-2, 2)], Rotation::Quarter),
    ([(0, 1), (1, 1), (1, 2), (1, 3), (2, 3)], Rotation::Zero),
    ([(1, 0), (1, 1), (2, 1), (2, 2), (3, 2)], Rotation::Zero),
    ([(1, 0), (1, 1), (1, 2), (2, 2), (2, 3)], Rotation::Zero),
];

fn find_cube_edge(grid: &Grid, start: &BlockPoint, direction: Direction) -> Edge {
    let mut block = *start;
    for (offsets, target_rotation) in GRID_PATHS {
        'paths: for (xmul, rotation) in
            [(1, target_rotation), (-1, Rotation::Zero - target_rotation)]
        {
            for (x, y) in offsets {
                block = match direction {
                    Direction::Up => BlockPoint::new(start.x + x * xmul, start.y + y),
                    Direction::Down => BlockPoint::new(start.x - x * xmul, start.y - y),
                    Direction::Left => BlockPoint::new(start.x + y, start.y - x * xmul),
                    Direction::Right => BlockPoint::new(start.x - y, start.y + x * xmul),
                };
                if !grid.blocks.contains(&block) {
                    continue 'paths;
                }
            }

            return Edge { block, rotation };
        }
    }
    panic!("No results found for {direction:?} from {start:?}.");
}

fn map_faces_cube(grid: &Grid) -> Directions {
    let blocks = grid.blocks;

    blocks
        .iter()
        .map(|block| {
            let top = find_cube_edge(grid, block, Direction::Up);
            let bottom = find_cube_edge(grid, block, Direction::Down);
            let left = find_cube_edge(grid, block, Direction::Left);
            let right = find_cube_edge(grid, block, Direction::Right);
            Face {
                top,
                bottom,
                left,
                right,
            }
        })
        .collect::<Vec<Face>>()
        .try_into()
        .unwrap()
}

#[register_part]
fn part1(input: &str) -> usize {
    let (grid, actions) = parse_input(input);
    let directions = map_faces_grid(&grid);
    process(grid, actions, directions)
}

#[register_part]
fn part2(input: &str) -> usize {
    let (grid, actions) = parse_input(input);
    let directions = map_faces_cube(&grid);
    process(grid, actions, directions)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 6032, part2 = 5031)]
    static EXAMPLE_INPUT: &str = "
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.

        10R5L5R10L4R5L5
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let grid = Grid {
            walls: vec![
                Point::new(11, 0),
                Point::new(9, 1),
                Point::new(8, 2),
                Point::new(3, 4),
                Point::new(11, 4),
                Point::new(8, 5),
                Point::new(2, 6),
                Point::new(7, 6),
                Point::new(10, 7),
                Point::new(11, 8),
                Point::new(13, 9),
                Point::new(9, 10),
                Point::new(14, 11),
            ]
            .into_iter()
            .collect(),
            block_size: 4,
            blocks: [
                BlockPoint::new(2, 0),
                BlockPoint::new(0, 1),
                BlockPoint::new(1, 1),
                BlockPoint::new(2, 1),
                BlockPoint::new(2, 2),
                BlockPoint::new(3, 2),
            ],
        };
        let actions = vec![
            Action::Move(10),
            Action::TurnRight,
            Action::Move(5),
            Action::TurnLeft,
            Action::Move(5),
            Action::TurnRight,
            Action::Move(10),
            Action::TurnLeft,
            Action::Move(4),
            Action::TurnRight,
            Action::Move(5),
            Action::TurnLeft,
            Action::Move(5),
        ];
        assert_eq!(actual, (grid, actions));
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn example_map_faces_grid() {
        let grid = Grid {
            walls: HashSet::new(),
            block_size: 4,
            blocks: [
                BlockPoint::new(2, 0),
                BlockPoint::new(0, 1),
                BlockPoint::new(1, 1),
                BlockPoint::new(2, 1),
                BlockPoint::new(2, 2),
                BlockPoint::new(3, 2),
            ],
        };
        let expected = [
            Face {
                // 1 @ (2, 0)
                top: Edge {
                    block: BlockPoint::new(2, 2),
                    rotation: Rotation::Zero,
                },
                bottom: Edge {
                    block: BlockPoint::new(2, 1),
                    rotation: Rotation::Zero,
                },
                left: Edge {
                    block: BlockPoint::new(2, 0),
                    rotation: Rotation::Zero,
                },
                right: Edge {
                    block: BlockPoint::new(2, 0),
                    rotation: Rotation::Zero,
                },
            },
            Face {
                // 2 @ (0, 1)
                top: Edge {
                    block: BlockPoint::new(0, 1),
                    rotation: Rotation::Zero,
                },
                bottom: Edge {
                    block: BlockPoint::new(0, 1),
                    rotation: Rotation::Zero,
                },
                left: Edge {
                    block: BlockPoint::new(2, 1),
                    rotation: Rotation::Zero,
                },
                right: Edge {
                    block: BlockPoint::new(1, 1),
                    rotation: Rotation::Zero,
                },
            },
            Face {
                // 3 @ (1, 1)
                top: Edge {
                    block: BlockPoint::new(1, 1),
                    rotation: Rotation::Zero,
                },
                bottom: Edge {
                    block: BlockPoint::new(1, 1),
                    rotation: Rotation::Zero,
                },
                left: Edge {
                    block: BlockPoint::new(0, 1),
                    rotation: Rotation::Zero,
                },
                right: Edge {
                    block: BlockPoint::new(2, 1),
                    rotation: Rotation::Zero,
                },
            },
            Face {
                // 4 @ (2, 1)
                top: Edge {
                    block: BlockPoint::new(2, 0),
                    rotation: Rotation::Zero,
                },
                bottom: Edge {
                    block: BlockPoint::new(2, 2),
                    rotation: Rotation::Zero,
                },
                left: Edge {
                    block: BlockPoint::new(1, 1),
                    rotation: Rotation::Zero,
                },
                right: Edge {
                    block: BlockPoint::new(0, 1),
                    rotation: Rotation::Zero,
                },
            },
            Face {
                // 5 @ (2, 2)
                top: Edge {
                    block: BlockPoint::new(2, 1),
                    rotation: Rotation::Zero,
                },
                bottom: Edge {
                    block: BlockPoint::new(2, 0),
                    rotation: Rotation::Zero,
                },
                left: Edge {
                    block: BlockPoint::new(3, 2),
                    rotation: Rotation::Zero,
                },
                right: Edge {
                    block: BlockPoint::new(3, 2),
                    rotation: Rotation::Zero,
                },
            },
            Face {
                // 6 @ (3, 2)
                top: Edge {
                    block: BlockPoint::new(3, 2),
                    rotation: Rotation::Zero,
                },
                bottom: Edge {
                    block: BlockPoint::new(3, 2),
                    rotation: Rotation::Zero,
                },
                left: Edge {
                    block: BlockPoint::new(2, 2),
                    rotation: Rotation::Zero,
                },
                right: Edge {
                    block: BlockPoint::new(2, 2),
                    rotation: Rotation::Zero,
                },
            },
        ];
        let actual = map_faces_grid(&grid);
        assert_eq!(expected, actual);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn example_map_faces_cube() {
        let grid = Grid {
            walls: HashSet::new(),
            block_size: 4,
            blocks: [
                BlockPoint::new(2, 0),
                BlockPoint::new(0, 1),
                BlockPoint::new(1, 1),
                BlockPoint::new(2, 1),
                BlockPoint::new(2, 2),
                BlockPoint::new(3, 2),
            ],
        };
        let expected = [
            Face {
                // 1 @ (2, 0)
                top: Edge {
                    block: BlockPoint::new(0, 1),
                    rotation: Rotation::Half,
                },
                bottom: Edge {
                    block: BlockPoint::new(2, 1),
                    rotation: Rotation::Zero,
                },
                left: Edge {
                    block: BlockPoint::new(1, 1),
                    rotation: Rotation::ThreeQuarters,
                },
                right: Edge {
                    block: BlockPoint::new(3, 2),
                    rotation: Rotation::Half,
                },
            },
            Face {
                // 2 @ (0, 1)
                top: Edge {
                    block: BlockPoint::new(2, 0),
                    rotation: Rotation::Half,
                },
                bottom: Edge {
                    block: BlockPoint::new(2, 2),
                    rotation: Rotation::Half,
                },
                left: Edge {
                    block: BlockPoint::new(3, 2),
                    rotation: Rotation::Quarter,
                },
                right: Edge {
                    block: BlockPoint::new(1, 1),
                    rotation: Rotation::Zero,
                },
            },
            Face {
                // 3 @ (1, 1)
                top: Edge {
                    block: BlockPoint::new(2, 0),
                    rotation: Rotation::Quarter,
                },
                bottom: Edge {
                    block: BlockPoint::new(2, 2),
                    rotation: Rotation::ThreeQuarters,
                },
                left: Edge {
                    block: BlockPoint::new(0, 1),
                    rotation: Rotation::Zero,
                },
                right: Edge {
                    block: BlockPoint::new(2, 1),
                    rotation: Rotation::Zero,
                },
            },
            Face {
                // 4 @ (2, 1)
                top: Edge {
                    block: BlockPoint::new(2, 0),
                    rotation: Rotation::Zero,
                },
                bottom: Edge {
                    block: BlockPoint::new(2, 2),
                    rotation: Rotation::Zero,
                },
                left: Edge {
                    block: BlockPoint::new(1, 1),
                    rotation: Rotation::Zero,
                },
                right: Edge {
                    block: BlockPoint::new(3, 2),
                    rotation: Rotation::Quarter,
                },
            },
            Face {
                // 5 @ (2, 2)
                top: Edge {
                    block: BlockPoint::new(2, 1),
                    rotation: Rotation::Zero,
                },
                bottom: Edge {
                    block: BlockPoint::new(0, 1),
                    rotation: Rotation::Half,
                },
                left: Edge {
                    block: BlockPoint::new(1, 1),
                    rotation: Rotation::Quarter,
                },
                right: Edge {
                    block: BlockPoint::new(3, 2),
                    rotation: Rotation::Zero,
                },
            },
            Face {
                // 6 @ (3, 2)
                top: Edge {
                    block: BlockPoint::new(2, 1),
                    rotation: Rotation::ThreeQuarters,
                },
                bottom: Edge {
                    block: BlockPoint::new(0, 1),
                    rotation: Rotation::ThreeQuarters,
                },
                left: Edge {
                    block: BlockPoint::new(2, 2),
                    rotation: Rotation::Zero,
                },
                right: Edge {
                    block: BlockPoint::new(2, 0),
                    rotation: Rotation::Half,
                },
            },
        ];
        let actual = map_faces_cube(&grid);
        assert_eq!(expected, actual);
    }
}
