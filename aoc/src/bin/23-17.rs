puzzle_runner::register_chapter!(book = "2023", title = "Clumsy Crucible");

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use puzzle_lib::{
    grid::FullGrid,
    point::{Direction2, Point2},
};

type Point = Point2;
type Direction = Direction2;
type Grid = FullGrid<u8>;

fn parse_input(input: &str) -> Grid {
    parse!(input => { [grid cells as u8] } => grid)
}

#[derive(Eq, PartialEq, Clone)]
struct State {
    cost: usize,
    position: Point,
    direction: Direction,
    moved_straight: u8,
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn find_path(grid: &Grid, min_before_turn: u8, max_before_turn: u8) -> usize {
    let bounds = Point::new(grid.width(), grid.height());
    let end = Point::new(bounds.x - 1, bounds.y - 1);
    let mut visited: HashSet<(Point, Direction, u8)> = HashSet::new();
    let mut next: BinaryHeap<State> = BinaryHeap::new();
    next.push(State {
        cost: 0,
        position: Point::new(0, 0),
        direction: Direction::East,
        moved_straight: 0,
    });
    next.push(State {
        cost: 0,
        position: Point::new(0, 0),
        direction: Direction::South,
        moved_straight: 0,
    });
    let mut next_directions = Vec::with_capacity(3);
    while let Some(state) = next.pop() {
        if state.position == end {
            return state.cost;
        }
        if visited.contains(&(state.position, state.direction, state.moved_straight)) {
            continue;
        }
        visited.insert((state.position, state.direction, state.moved_straight));

        next_directions.clear();
        if state.position.y > 0 && state.direction != Direction::South {
            next_directions.push(Direction::North);
        }
        if state.position.y < end.x && state.direction != Direction::North {
            next_directions.push(Direction::South);
        }
        if state.position.x > 0 && state.direction != Direction::East {
            next_directions.push(Direction::West);
        }
        if state.position.x < end.y && state.direction != Direction::West {
            next_directions.push(Direction::East);
        }

        for direction in &next_directions {
            let moved_straight = if direction == &state.direction {
                if state.moved_straight >= max_before_turn {
                    continue;
                }
                state.moved_straight + 1
            } else {
                if state.moved_straight < min_before_turn {
                    continue;
                }
                1
            };

            let position = state.position + *direction;
            next.push(State {
                cost: state.cost + (grid[position] as usize),
                position,
                direction: *direction,
                moved_straight,
            });
        }
    }
    panic!("Unable to find path to end, this should never happen.");
}

#[register_part]
fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    find_path(&grid, 0, 3)
}

#[register_part]
fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    find_path(&grid, 4, 10)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 102, part2 = 94)]
    static EXAMPLE_INPUT: &str = "
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = [
            [2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3],
            [3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3],
            [3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4],
            [3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2],
            [4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6],
            [1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4],
            [4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6],
            [3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3],
            [4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7],
            [4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3],
            [1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3],
            [2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5],
            [4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3],
        ]
        .into();
        assert_eq!(actual, expected);
    }
}
