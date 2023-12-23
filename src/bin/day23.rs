use std::{collections::HashMap, mem, ops::Add};

use aoc::utils::{parse, point::Point2};

type Point = Point2<usize>;

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Open,
    OneWay(Direction),
}
impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Wall,
            '.' => Tile::Open,
            '^' => Tile::OneWay(Direction::North),
            '>' => Tile::OneWay(Direction::East),
            'v' => Tile::OneWay(Direction::South),
            '<' => Tile::OneWay(Direction::West),
            _ => panic!("Invalid tile {value:?}."),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Add<Point> for Direction {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        match self {
            Direction::North => Point::new(rhs.x, rhs.y.wrapping_sub(1)),
            Direction::East => Point::new(rhs.x + 1, rhs.y),
            Direction::South => Point::new(rhs.x, rhs.y + 1),
            Direction::West => Point::new(rhs.x.wrapping_sub(1), rhs.y),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    start: Point,
    end: Point,
    tiles: Vec<Vec<Tile>>,
}

fn parse_input(input: &str) -> Map {
    parse!(input => [tiles split on '\n' with [chars as Tile]]);

    Map {
        start: Point::new(1, 0),
        end: Point::new(tiles[0].len() - 2, tiles.len() - 1),
        tiles,
    }
}

type Graph = HashMap<Point, HashMap<Point, usize>>;

fn _to_graph(map: &Map, graph: &mut Graph, from_node: Point, from: Point, mut steps: usize) {
    let node = graph.entry(from_node).or_default();
    let mut prev = from;
    let mut curr = from;

    // Our starting point (either at the start of the maze or right after a junction) will always only have a single Tile::Open next to it.
    for neighbour in curr.neighbours_ortho() {
        if map.tiles[neighbour.y][neighbour.x] == Tile::Open {
            curr = neighbour;
            steps += 1;
            break;
        }
    }

    // As long as the neighbor that we didn't just come from remains a Tile::Open there are no branches and we can just follow the path.
    'step: loop {
        if curr == map.end {
            node.insert(curr, steps);
            return;
        }

        for neighbour in curr.neighbours_ortho() {
            if neighbour != prev {
                let tile = &map.tiles[neighbour.y][neighbour.x];
                if tile == &Tile::Wall {
                    continue;
                }

                mem::swap(&mut prev, &mut curr);
                curr = neighbour;
                steps += 1;
                if tile != &Tile::Open {
                    break 'step;
                }
                continue 'step;
            }
        }
        break;
    }

    // We've arrived at a junction, add the found path to the graph.
    if let Tile::OneWay(direction) = map.tiles[curr.y][curr.x] {
        curr = direction + curr;
        steps += 1;
    } else {
        panic!("Expected one-way tile at {curr:?}.");
    }
    node.insert(curr, steps);

    // Move into it, add it to the graph, anBranch for each possible result.
    for neighbour in curr.neighbours_ortho() {
        if neighbour == prev {
            continue;
        }
        #[allow(clippy::match_on_vec_items)]
        match map.tiles[neighbour.y][neighbour.x] {
            Tile::Wall => {}
            Tile::Open => panic!(
                "Open tile at {neighbour:?} next to junction tile {curr:?}, this should not happen."
            ),
            Tile::OneWay(direction) => {
                let next = direction + neighbour;
                if next != curr {
                    _to_graph(map, graph, curr, next, 2);
                }
            }
        }
    }
}

fn to_graph(map: &Map) -> Graph {
    let mut graph = Graph::new();
    _to_graph(map, &mut graph, map.start, map.start, 0);
    graph
}

fn make_graph_bidirectional(graph: &mut Graph) {
    for (start, edges) in graph.clone() {
        for (end, steps) in edges {
            graph.entry(end).or_default().insert(start, steps);
        }
    }
}

fn find_longest_path(graph: &Graph, path: &mut Vec<Point>, from: Point, to: Point) -> isize {
    if from == to {
        return 0;
    }
    if path.contains(&from) {
        return isize::MIN;
    }
    path.push(from);
    let result = graph
        .get(&from)
        .unwrap()
        .iter()
        .map(|(curr, steps)| *steps as isize + find_longest_path(graph, path, *curr, to))
        .max()
        .unwrap();
    path.pop();
    result
}

pub fn part1(input: &str) -> usize {
    let map = parse_input(input);
    let graph = to_graph(&map);
    find_longest_path(&graph, &mut Vec::new(), map.start, map.end) as usize
}

pub fn part2(input: &str) -> usize {
    let map = parse_input(input);
    let mut graph = to_graph(&map);
    make_graph_bidirectional(&mut graph);
    find_longest_path(&graph, &mut Vec::new(), map.start, map.end) as usize
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 94, test)]
    static EXAMPLE_INPUT: &str = "
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
    ";

    #[allow(clippy::too_many_lines)]
    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = Map {
            start: Point::new(1, 0),
            end: Point::new(21, 22),
            tiles: vec![
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::OneWay(Direction::East),
                    Tile::Open,
                    Tile::OneWay(Direction::East),
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::OneWay(Direction::South),
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::OneWay(Direction::South),
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::OneWay(Direction::East),
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::OneWay(Direction::South),
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::OneWay(Direction::South),
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::OneWay(Direction::East),
                    Tile::Open,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::OneWay(Direction::South),
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::OneWay(Direction::South),
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::OneWay(Direction::South),
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::OneWay(Direction::East),
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::OneWay(Direction::East),
                    Tile::Open,
                    Tile::OneWay(Direction::East),
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::OneWay(Direction::South),
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::OneWay(Direction::South),
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::OneWay(Direction::South),
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::OneWay(Direction::South),
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::OneWay(Direction::East),
                    Tile::Open,
                    Tile::OneWay(Direction::East),
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::OneWay(Direction::East),
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                    Tile::OneWay(Direction::South),
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Open,
                    Tile::Open,
                    Tile::Wall,
                ],
                vec![
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Open,
                    Tile::Wall,
                ],
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_graph_bidirectional() {
        let mut graph = hash_map![
            Point::new(0, 0) => hash_map![
                Point::new(1, 0) => 15,
                Point::new(2, 0) => 8,
            ],
            Point::new(2, 0) => hash_map![
                Point::new(1, 0) => 10,
            ],
        ];
        super::make_graph_bidirectional(&mut graph);
        let expected = hash_map![
            Point::new(0, 0) => hash_map![
                Point::new(1, 0) => 15,
                Point::new(2, 0) => 8,
            ],
            Point::new(1, 0) => hash_map![
                Point::new(0, 0) => 15,
                Point::new(2, 0) => 10,
            ],
            Point::new(2, 0) => hash_map![
                Point::new(0, 0) => 8,
                Point::new(1, 0) => 10,
            ],
        ];
        assert_eq!(graph, expected);
    }
}
