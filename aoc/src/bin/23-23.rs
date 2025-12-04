puzzle_runner::register_chapter!(book = "2023", title = "A Long Walk");

use std::{collections::HashMap, mem};

use puzzle_lib::{
    grid::FullGrid,
    point::{Direction2, Point2},
};

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Open,
    OneWay(Direction2),
}

type Grid = FullGrid<Tile>;

fn parse_input(input: &str) -> Grid {
    parse!(input => {
        [grid cells match {
            '#' => Tile::Wall,
            '.' => Tile::Open,
            '^' => Tile::OneWay(Direction2::North),
            '>' => Tile::OneWay(Direction2::East),
            'v' => Tile::OneWay(Direction2::South),
            '<' => Tile::OneWay(Direction2::West),
        }]
    } => grid)
}

struct Graph {
    ids: HashMap<Point2, usize>,
    graph: HashMap<usize, HashMap<usize, usize>>,
}
impl Graph {
    fn new() -> Self {
        Self {
            ids: HashMap::new(),
            graph: HashMap::new(),
        }
    }

    fn get_id(&mut self, point: Point2) -> usize {
        let len = self.ids.len();
        *self
            .ids
            .entry(point)
            .or_insert_with(|| 2usize.pow(len as u32))
    }

    fn connect(&mut self, from: Point2, to: Point2, steps: usize) {
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

fn to_graph_inner(
    grid: &Grid,
    graph: &mut Graph,
    from_node: Point2,
    from: Point2,
    to: Point2,
    mut steps: usize,
) {
    let mut prev = from;
    let mut curr = from;

    // Our starting point (either at the start of the maze or right after a junction) will always only have a single Tile::Open next to it.
    for neighbour in curr.neighbours_ortho() {
        if grid[neighbour] == Tile::Open {
            curr = neighbour;
            steps += 1;
            break;
        }
    }

    // As long as the neighbor that we didn't just come from remains a Tile::Open there are no branches and we can just follow the path.
    'step: loop {
        if curr == to {
            graph.connect(from_node, curr, steps);
            return;
        }

        for neighbour in curr.neighbours_ortho() {
            if neighbour != prev {
                let tile = &grid[neighbour];
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
    if let Tile::OneWay(direction2) = grid[curr] {
        curr += direction2;
        steps += 1;
    } else {
        panic!("Expected one-way tile at {curr:?}.");
    }
    graph.connect(from_node, curr, steps);

    // Move into it, add it to the graph, and branch for each possible result.
    for neighbour in curr.neighbours_ortho() {
        if neighbour == prev {
            continue;
        }
        match grid[neighbour] {
            Tile::Wall => {}
            Tile::Open => panic!(
                "Open tile at {neighbour:?} next to junction tile {curr:?}, this should not happen."
            ),
            Tile::OneWay(direction2) => {
                let next = neighbour + direction2;
                if next != curr {
                    to_graph_inner(grid, graph, curr, next, to, 2);
                }
            }
        }
    }
}

fn to_graph(grid: &Grid, start: Point2, end: Point2) -> Graph {
    let mut graph = Graph::new();
    to_graph_inner(grid, &mut graph, start, start, end, 0);
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
    graph
        .graph
        .get(&from)
        .unwrap()
        .par_iter()
        .map(|(curr, steps)| {
            *steps as isize + find_longest_path_inner(graph, abort_if, visited, *curr, to)
        })
        .max()
        .unwrap()
}

fn find_longest_path(graph: &mut Graph, from: Point2, to: Point2) -> isize {
    let from = graph.get_id(from);
    let to = graph.get_id(to);

    // Determine some states where the we can easily detect that there is no longer any path to the end.
    let mut abort_if = vec![graph.graph.get(&to).map_or(0, |e| e.keys().sum())];
    abort_if.retain(|v| *v > 0);

    find_longest_path_inner(graph, &abort_if, 0, from, to)
}

pub fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let start = Point2::new(1, 0);
    let end = Point2::new(grid.width() - 2, grid.height() - 1);
    let mut graph = to_graph(&grid, start, end);
    find_longest_path(&mut graph, start, end) as usize
}

pub fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let start = Point2::new(1, 0);
    let end = Point2::new(grid.width() - 2, grid.height() - 1);
    let mut graph = to_graph(&grid, start, end);
    graph.make_bidirectional();
    find_longest_path(&mut graph, start, end) as usize
}

#[cfg(test)]
mod tests {
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 94)]
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
        let expected = [
            [
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
            [
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
            [
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
            [
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
                Tile::OneWay(Direction2::East),
                Tile::Open,
                Tile::OneWay(Direction2::East),
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
            [
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::OneWay(Direction2::South),
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Open,
                Tile::Wall,
                Tile::OneWay(Direction2::South),
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
            [
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Open,
                Tile::OneWay(Direction2::East),
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
            [
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::OneWay(Direction2::South),
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
            [
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
            [
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
            [
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
            [
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
                Tile::OneWay(Direction2::South),
                Tile::Wall,
            ],
            [
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
                Tile::OneWay(Direction2::East),
                Tile::Open,
                Tile::Wall,
            ],
            [
                Tile::Wall,
                Tile::Open,
                Tile::Wall,
                Tile::Open,
                Tile::Wall,
                Tile::OneWay(Direction2::South),
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::OneWay(Direction2::South),
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Open,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::OneWay(Direction2::South),
                Tile::Wall,
            ],
            [
                Tile::Wall,
                Tile::Open,
                Tile::Open,
                Tile::Open,
                Tile::Wall,
                Tile::Open,
                Tile::OneWay(Direction2::East),
                Tile::Open,
                Tile::Wall,
                Tile::Open,
                Tile::Open,
                Tile::Open,
                Tile::OneWay(Direction2::East),
                Tile::Open,
                Tile::OneWay(Direction2::East),
                Tile::Open,
                Tile::Wall,
                Tile::Open,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Open,
                Tile::Wall,
            ],
            [
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::OneWay(Direction2::South),
                Tile::Wall,
                Tile::Open,
                Tile::Wall,
                Tile::Open,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::OneWay(Direction2::South),
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
            [
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
            [
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
            [
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
            [
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
                Tile::OneWay(Direction2::South),
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::OneWay(Direction2::South),
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
            ],
            [
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
                Tile::OneWay(Direction2::East),
                Tile::Open,
                Tile::OneWay(Direction2::East),
                Tile::Open,
                Tile::Wall,
                Tile::Open,
                Tile::OneWay(Direction2::East),
                Tile::Open,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
            ],
            [
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
                Tile::OneWay(Direction2::South),
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
            ],
            [
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
            [
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
        ]
        .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn make_graph_bidirectional() {
        let mut graph = Graph {
            ids: hash_map![
                Point2::new(0, 0) => 1,
                Point2::new(1, 0) => 2,
                Point2::new(2, 0) => 4,
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
