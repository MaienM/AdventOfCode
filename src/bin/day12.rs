use std::collections::{HashMap, HashSet};

use aoc::*;

const NAME_START: &'static str = "start";
const NAME_END: &'static str = "end";

#[derive(Debug, Eq, PartialEq)]
enum NodeType {
    Special,
    Big,
    Small,
}
impl NodeType {
    pub fn get(name: &String) -> NodeType {
        if name == NAME_START || name == NAME_END {
            return NodeType::Special;
        } else if name == &name.to_uppercase() {
            return NodeType::Big;
        } else {
            return NodeType::Small;
        }
    }
}

#[derive(Debug, PartialEq)]
struct Graph {
    edges: HashMap<String, Vec<String>>,
}
impl Graph {
    pub fn new() -> Self {
        return Self {
            edges: HashMap::new(),
        };
    }

    pub fn add_connection(&mut self, left: &String, right: &String) {
        if NodeType::get(left) == NodeType::Big && NodeType::get(right) == NodeType::Big {
            panic!("Big caves may not be directly connected as this would create infinite paths, but {} and {} are.", left, right);
        }

        if !self.edges.contains_key(left) {
            self.edges.insert(left.clone(), Vec::new());
        }
        self.edges.get_mut(left).unwrap().push(right.clone());

        if !self.edges.contains_key(right) {
            self.edges.insert(right.clone(), Vec::new());
        }
        self.edges.get_mut(right).unwrap().push(left.clone());
    }

    pub fn get_connections(&self, name: &String) -> &Vec<String> {
        return &self.edges.get(name).unwrap();
    }
}

fn get_paths_from_to(
    graph: &Graph,
    name: &String,
    to: &String,
    visited: &mut HashSet<String>,
) -> Vec<Vec<String>> {
    let mut results: Vec<Vec<String>> = Vec::new();
    if visited.contains(name) {
        return results;
    }

    let is_small = NodeType::get(name) == NodeType::Small;
    if is_small {
        visited.insert(name.clone());
    }

    for connection in graph.get_connections(name) {
        if connection == to {
            results.push(vec![connection.clone()]);
        }
        if NodeType::get(connection) == NodeType::Special {
            continue;
        }

        for mut path in get_paths_from_to(graph, connection, to, visited) {
            path.push(name.clone());
            results.push(path);
        }
    }

    visited.remove(name);

    return results;
}

fn parse_input(input: String) -> Graph {
    let mut graph = Graph::new();
    for line in input.trim().split("\n").map(str::trim) {
        let mut parts = line.splitn(2, "-");
        graph.add_connection(
            &parts.next().unwrap().to_string(),
            &parts.next().unwrap().to_string(),
        );
    }
    return graph;
}

fn part1(input: String) -> i64 {
    let graph = parse_input(input);
    let mut visited: HashSet<String> = HashSet::new();
    let paths = get_paths_from_to(
        &graph,
        &NAME_START.to_string(),
        &NAME_END.to_string(),
        &mut visited,
    );
    return paths.len() as i64;
}

fn main() {
    run(part1, missing);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &'static str = "
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    ";
    const EXAMPLE_INPUT_2: &'static str = "
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    ";
    const EXAMPLE_INPUT_3: &'static str = "
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    ";

    #[test]
    fn example1_parse() {
        let graph = parse_input(EXAMPLE_INPUT_1.to_string());
        assert_eq!(graph.get_connections(&"start".to_string()), &vec!["A", "b"]);
        assert_eq!(graph.get_connections(&"end".to_string()), &vec!["A", "b"]);
        assert_eq!(
            graph.get_connections(&"A".to_string()),
            &vec!["start", "c", "b", "end"]
        );
        assert_eq!(
            graph.get_connections(&"b".to_string()),
            &vec!["start", "A", "d", "end"]
        );
        assert_eq!(graph.get_connections(&"c".to_string()), &vec!["A"]);
        assert_eq!(graph.get_connections(&"d".to_string()), &vec!["b"]);
    }

    #[test]
    fn example1_part1() {
        assert_eq!(part1(EXAMPLE_INPUT_1.to_string()), 10);
    }

    #[test]
    fn example2_part1() {
        assert_eq!(part1(EXAMPLE_INPUT_2.to_string()), 19);
    }

    #[test]
    fn example3_part1() {
        assert_eq!(part1(EXAMPLE_INPUT_3.to_string()), 226);
    }
}
