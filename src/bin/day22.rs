use aoc::grid::Point;
use aoc::runner::*;
use std::collections::HashSet;
use std::ops::Range;

#[derive(Debug, Eq, PartialEq)]
struct Grid {
    walls: HashSet<Point>,
    xbounds: Vec<Range<usize>>,
    ybounds: Vec<Range<usize>>,
}
impl Grid {
    fn get_starting_point(&self) -> Point {
        let bounds = self.xbounds.get(0).unwrap();
        for x in bounds.clone() {
            let point = Point::new(x, 0);
            if !self.walls.contains(&point) {
                return point;
            }
        }
        panic!();
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Facing {
    Up = 3,
    Down = 1,
    Left = 2,
    Right = 0,
}
impl Facing {
    fn from_number(num: u8) -> Facing {
        return match num {
            n if n == Facing::Up as u8 => Facing::Up,
            n if n == Facing::Down as u8 => Facing::Down,
            n if n == Facing::Left as u8 => Facing::Left,
            n if n == Facing::Right as u8 => Facing::Right,
            _ => panic!(),
        };
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    grid: Grid,
    position: Point,
    facing: Facing,
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
                self.facing = Facing::from_number((self.facing as u8 + 3) % 4);
            }
            Action::TurnRight => {
                self.facing = Facing::from_number((self.facing as u8 + 1) % 4);
            }
        }
    }

    fn move_(&mut self) -> bool {
        let next = match self.facing {
            Facing::Up => {
                let bounds = &self.grid.ybounds[self.position.x];
                if self.position.y == bounds.start {
                    Point::new(self.position.x, bounds.end - 1)
                } else {
                    Point::new(self.position.x, self.position.y - 1)
                }
            }
            Facing::Down => {
                let bounds = &self.grid.ybounds[self.position.x];
                if self.position.y == bounds.end - 1 {
                    Point::new(self.position.x, bounds.start)
                } else {
                    Point::new(self.position.x, self.position.y + 1)
                }
            }
            Facing::Left => {
                let bounds = &self.grid.xbounds[self.position.y];
                if self.position.x == bounds.start {
                    Point::new(bounds.end - 1, self.position.y)
                } else {
                    Point::new(self.position.x - 1, self.position.y)
                }
            }
            Facing::Right => {
                let bounds = &self.grid.xbounds[self.position.y];
                if self.position.x == bounds.end - 1 {
                    Point::new(bounds.start, self.position.y)
                } else {
                    Point::new(self.position.x + 1, self.position.y)
                }
            }
        };
        if !self.grid.walls.contains(&next) {
            self.position = next;
            return true;
        }
        return false;
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Action {
    Move(u8),
    TurnLeft,
    TurnRight,
}
type Actions = Vec<Action>;

fn parse_input(input: String) -> (Grid, Actions) {
    let [grid, path]: [&str; 2] = input
        .splitn(2, "\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();

    let mut walls = HashSet::new();
    let mut xbounds = Vec::new();
    for (y, line) in grid.split("\n").enumerate() {
        let xstart = line.len() - line.trim().len();
        let xend = line.len();
        xbounds.push(xstart..xend);

        for (x, c) in line.char_indices() {
            if c == '#' {
                walls.insert(Point::new(x, y));
            }
        }
    }
    let mut ybounds = Vec::new();
    for x in 0..(xbounds.iter().map(|r| r.end).max().unwrap()) {
        let ystart = xbounds
            .iter()
            .enumerate()
            .find(|(_, r)| r.contains(&x))
            .unwrap()
            .0;
        let yend = xbounds
            .iter()
            .enumerate()
            .rev()
            .find(|(_, r)| r.contains(&x))
            .unwrap()
            .0;
        ybounds.push(ystart..(yend + 1));
    }
    let grid = Grid {
        walls,
        xbounds,
        ybounds,
    };

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
            _ => panic!("Invalid path char {:?}", c),
        }
    }

    return (grid, actions);
}

pub fn part1(input: String) -> usize {
    let (grid, actions) = parse_input(input);
    let mut state = State {
        position: grid.get_starting_point(),
        grid,
        facing: Facing::Right,
    };
    for action in actions {
        state.apply(&action);
    }
    return (state.position.y + 1) * 1000 + (state.position.x + 1) * 4 + (state.facing as usize);
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
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
            xbounds: vec![
                8..12,
                8..12,
                8..12,
                8..12,
                0..12,
                0..12,
                0..12,
                0..12,
                8..16,
                8..16,
                8..16,
                8..16,
            ],
            ybounds: vec![
                4..8,
                4..8,
                4..8,
                4..8,
                4..8,
                4..8,
                4..8,
                4..8,
                0..12,
                0..12,
                0..12,
                0..12,
                8..12,
                8..12,
                8..12,
                8..12,
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
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 6_032);
    }
}
