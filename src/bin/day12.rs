use std::collections::HashMap;

use aoc::runner::*;

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

fn count_paths_to_end(
    graph: &Graph,
    path: &mut Vec<String>,
    node: &String,
    did_small_double_visit: bool,
) -> i64 {
    let mut results = 0_i64;
    for connected_node in graph.get_connections(node) {
        if connected_node == NAME_START {
            continue;
        } else if connected_node == NAME_END {
            results += 1;
            continue;
        }

        let mut did_small_double_visit = did_small_double_visit;
        if NodeType::get(connected_node) == NodeType::Small && path.contains(connected_node) {
            if did_small_double_visit {
                continue;
            } else {
                did_small_double_visit = true;
            }
        }

        path.push(connected_node.clone());
        results += count_paths_to_end(graph, path, connected_node, did_small_double_visit);
        path.pop();
    }
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
    let mut path: Vec<String> = Vec::new();
    return count_paths_to_end(&graph, &mut path, &NAME_START.to_string(), true);
}

fn part2(input: String) -> i64 {
    let graph = parse_input(input);
    let mut path: Vec<String> = Vec::new();
    return count_paths_to_end(&graph, &mut path, &NAME_START.to_string(), false);
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
    fn nodetype() {
        assert_eq!(NodeType::get(&NAME_START.to_string()), NodeType::Special);
        assert_eq!(NodeType::get(&NAME_END.to_string()), NodeType::Special);
        assert_eq!(NodeType::get(&"A".to_string()), NodeType::Big);
        assert_eq!(NodeType::get(&"b".to_string()), NodeType::Small);
        assert_eq!(NodeType::get(&"JK".to_string()), NodeType::Big);
        assert_eq!(NodeType::get(&"hl".to_string()), NodeType::Small);
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

    #[test]
    fn example1_part2() {
        assert_eq!(part2(EXAMPLE_INPUT_1.to_string()), 36);
    }

    #[test]
    fn example2_part2() {
        assert_eq!(part2(EXAMPLE_INPUT_2.to_string()), 103);
    }

    #[test]
    fn example3_part2() {
        assert_eq!(part2(EXAMPLE_INPUT_3.to_string()), 3509);
    }
}
