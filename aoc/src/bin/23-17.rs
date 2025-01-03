aoc::setup!();

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use aoc::point::{Direction2, Point2};

type Point = Point2;
type Direction = Direction2;
type Map = Vec<Vec<u8>>;

fn parse_input(input: &str) -> Map {
    parse!(input => {
        [map split on '\n' with [chars as u8]]
    } => map)
}

#[derive(Eq, PartialEq, Clone)]
struct State {
    cost: usize,
    position: Point,
    direction: Direction,
    moved_straight: u8,
    #[cfg(feature = "visual")]
    path: Vec<Point>,
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

fn find_path(map: &Map, min_before_turn: u8, max_before_turn: u8) -> usize {
    let bounds = Point::new(map[0].len(), map.len());
    let end = Point::new(bounds.x - 1, bounds.y - 1);
    let mut visited: HashSet<(Point, Direction, u8)> = HashSet::new();
    let mut next: BinaryHeap<State> = BinaryHeap::new();
    next.push(State {
        cost: 0,
        position: Point::new(0, 0),
        direction: Direction::East,
        moved_straight: 0,
        #[cfg(feature = "visual")]
        path: vec![Point::new(0, 0)],
    });
    next.push(State {
        cost: 0,
        position: Point::new(0, 0),
        direction: Direction::South,
        moved_straight: 0,
        #[cfg(feature = "visual")]
        path: vec![Point::new(0, 0)],
    });
    let mut next_directions = Vec::with_capacity(3);
    #[cfg(feature = "visual")]
    let tx = VISUAL_CHANNEL.clone();
    while let Some(state) = next.pop() {
        if state.position == end {
            #[cfg(feature = "visual")]
            tx.send(visual::Info::Done(state.clone())).unwrap();
            return state.cost;
        }
        if visited.contains(&(state.position, state.direction, state.moved_straight)) {
            continue;
        }
        visited.insert((state.position, state.direction, state.moved_straight));
        #[cfg(feature = "visual")]
        tx.send(visual::Info::Step(state.clone())).unwrap();

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
                cost: state.cost + (map[position.y][position.x] as usize),
                position,
                direction: *direction,
                moved_straight,
                #[cfg(feature = "visual")]
                path: {
                    let mut p = state.path.clone();
                    p.push(position);
                    p
                },
            });
        }
    }
    panic!("Unable to find path to end, this should never happen.");
}

pub fn part1(input: &str) -> usize {
    let map = parse_input(input);
    find_path(&map, 0, 3)
}

pub fn part2(input: &str) -> usize {
    let map = parse_input(input);
    find_path(&map, 4, 10)
}

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use pretty_assertions::assert_eq;

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
        let expected = vec![
            vec![2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3],
            vec![3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3],
            vec![3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4],
            vec![3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2],
            vec![4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6],
            vec![1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4],
            vec![4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6],
            vec![3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3],
            vec![4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7],
            vec![4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3],
            vec![1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3],
            vec![2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5],
            vec![4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3],
        ];
        assert_eq!(actual, expected);
    }
}

#[aoc_runner::visual]
pub mod visual {
    use aoc_runner::visual::{Config, ToRenderable, Visual};
    use raqote::{DrawOptions, Point, SolidSource, Source};

    pub(super) enum Info {
        Step(super::State),
        Done(super::State),
    }

    struct RenderConfig {
        step: usize,
        build: u8,
        drop: u8,
    }

    struct Tile {
        strength: u8,
    }

    #[derive(ToRenderable)]
    struct Renderer {
        config: RenderConfig,
        map: Vec<Vec<Tile>>,
        result: Option<Vec<super::Point>>,
    }
    impl Visual for Renderer {
        type Info = Info;

        fn update<I>(&mut self, info: I)
        where
            I: Iterator<Item = Self::Info>,
        {
            if let Some(path) = &self.result {
                for (y, row) in self.map.iter_mut().enumerate() {
                    for (x, tile) in row.iter_mut().enumerate() {
                        tile.strength = if path.contains(&super::Point::new(x, y)) {
                            tile.strength.saturating_add(self.config.drop)
                        } else {
                            tile.strength.saturating_sub(self.config.drop)
                        };
                    }
                }
                return;
            }

            for row in &mut self.map {
                for tile in row {
                    tile.strength = tile.strength.saturating_sub(self.config.drop);
                }
            }

            for info in info.take(self.config.step) {
                match info {
                    Info::Step(state) => {
                        let strength = &mut self.map[state.position.y][state.position.x].strength;
                        *strength = strength.saturating_add(self.config.build);
                    }
                    Info::Done(state) => {
                        self.result = Some(state.path);
                    }
                }
            }
        }

        fn draw(&self, dt: &mut raqote::DrawTarget, _config: &Config) {
            let cs = 8.0;

            for (y, row) in self.map.iter().enumerate() {
                for (x, tile) in row.iter().enumerate() {
                    let point = Point::new(x as f32 * cs, y as f32 * cs);
                    dt.fill_rect(
                        point.x + 1.0,
                        point.y + 1.0,
                        cs - 2.0,
                        cs - 2.0,
                        &Source::Solid(SolidSource::from_unpremultiplied_argb(
                            0xff,
                            0xff,
                            0xff - tile.strength,
                            0xff,
                        )),
                        &DrawOptions::new(),
                    );
                }
            }
        }
    }
    impl Renderer {
        fn new(input: &str, config: RenderConfig) -> Self {
            let map = super::parse_input(input)
                .into_iter()
                .map(|row| row.into_iter().map(|_| Tile { strength: 0 }).collect())
                .collect();
            Self {
                config,
                map,
                result: None,
            }
        }
    }

    pub fn part1(input: &str) -> impl ToRenderable {
        Renderer::new(
            input,
            RenderConfig {
                step: 2000,
                build: 24,
                drop: 32,
            },
        )
    }

    pub fn part2(input: &str) -> impl ToRenderable {
        Renderer::new(
            input,
            RenderConfig {
                step: 5000,
                build: 16,
                drop: 48,
            },
        )
    }
}
