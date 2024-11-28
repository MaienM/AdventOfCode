use std::{collections::HashMap, mem};

use aoc::utils::{
    parse,
    point::{Direction2, Point2},
};
use rayon::prelude::*;

type Point = Point2<usize>;
type Direction = Direction2;

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

struct Graph {
    ids: HashMap<Point, usize>,
    graph: HashMap<usize, HashMap<usize, usize>>,
}
impl Graph {
    fn new() -> Self {
        Self {
            ids: HashMap::new(),
            graph: HashMap::new(),
        }
    }

    fn get_id(&mut self, point: Point) -> usize {
        let len = self.ids.len();
        *self
            .ids
            .entry(point)
            .or_insert_with(|| 2usize.pow(len as u32))
    }

    fn connect(&mut self, from: Point, to: Point, steps: usize) {
        let from = self.get_id(from);
        let to = self.get_id(to);
        self.graph.entry(from).or_default().insert(to, steps);
    }

    fn make_bidirectional(&mut self) {
        for (start, edges) in self.graph.clone() {
            for (end, steps) in edges {
                self.graph.entry(end).or_default().insert(start, steps);
            }
        }
    }
}

fn to_graph_inner(map: &Map, graph: &mut Graph, from_node: Point, from: Point, mut steps: usize) {
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
            graph.connect(from_node, curr, steps);
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
        curr += direction;
        steps += 1;
    } else {
        panic!("Expected one-way tile at {curr:?}.");
    }
    graph.connect(from_node, curr, steps);

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
                let next = neighbour + direction;
                if next != curr {
                    to_graph_inner(map, graph, curr, next, 2);
                }
            }
        }
    }
}

fn to_graph(map: &Map) -> Graph {
    let mut graph = Graph::new();
    to_graph_inner(map, &mut graph, map.start, map.start, 0);
    graph
}

fn find_longest_path_inner(
    graph: &Graph,
    abort_if: &[usize],
    mut visited: usize,
    from: usize,
    to: usize,
) -> isize {
    if from == to {
        return 0;
    }
    if visited & from > 0 {
        return isize::MIN;
    }
    for flag in abort_if {
        if visited & flag == *flag {
            return isize::MIN;
        }
    }
    visited |= from;
    let result = graph
        .graph
        .get(&from)
        .unwrap()
        .par_iter()
        .map(|(curr, steps)| {
            *steps as isize + find_longest_path_inner(graph, abort_if, visited, *curr, to)
        })
        .max()
        .unwrap();
    result
}

fn find_longest_path(graph: &mut Graph, from: Point, to: Point) -> isize {
    let from = graph.get_id(from);
    let to = graph.get_id(to);

    // Determine some states where the we can easily detect that there is no longer any path to the end.
    let mut abort_if = vec![graph.graph.get(&to).map_or(0, |e| e.keys().sum())];
    abort_if.retain(|v| *v > 0);

    find_longest_path_inner(graph, &abort_if, 0, from, to)
}

pub fn part1(input: &str) -> usize {
    let map = parse_input(input);
    let mut graph = to_graph(&map);
    find_longest_path(&mut graph, map.start, map.end) as usize
}

pub fn part2(input: &str) -> usize {
    let map = parse_input(input);
    let mut graph = to_graph(&map);
    graph.make_bidirectional();
    find_longest_path(&mut graph, map.start, map.end) as usize
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
        let mut graph = Graph {
            ids: hash_map![
                Point::new(0, 0) => 1,
                Point::new(1, 0) => 2,
                Point::new(2, 0) => 4,
            ],
            graph: hash_map![
                1 => hash_map![
                    2 => 15,
                    4 => 8,
                ],
                4 => hash_map![
                    2 => 10,
                ],
            ],
        };
        graph.make_bidirectional();
        let expected = hash_map![
            1 => hash_map![
                2 => 15,
                4 => 8,
            ],
            2 => hash_map![
                1 => 15,
                4 => 10,
            ],
            4 => hash_map![
                1 => 8,
                2 => 10,
            ],
        ];
        assert_eq!(graph.graph, expected);
    }
}
