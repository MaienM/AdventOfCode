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
    pub fn get(name: &str) -> NodeType {
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
struct Graph<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
}
impl<'a> Graph<'a> {
    pub fn new() -> Self {
        return Self {
            edges: HashMap::new(),
        };
    }

    pub fn add_connection(&mut self, left: &'a str, right: &'a str) {
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

    pub fn get_connections(&self, name: &'a str) -> &Vec<&'a str> {
        return &self.edges.get(name).unwrap();
    }
}

fn count_paths_to_end<'a>(
    graph: &'a Graph,
    mut path: Vec<&'a str>,
    node: &'a str,
    did_small_double_visit: bool,
) -> (Vec<&'a str>, i64) {
    let mut results = 0_i64;
    for connected_node in graph.get_connections(node) {
        if *connected_node == NAME_START {
            continue;
        } else if *connected_node == NAME_END {
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
        let (new_path, new_result) =
            count_paths_to_end(graph, path, connected_node, did_small_double_visit);
        results += new_result;
        path = new_path;
        path.pop();
    }
    return (path, results);
}

fn parse_input<'a>(input: &'a String) -> Graph<'a> {
    let mut graph = Graph::new();
    for line in input.trim().split("\n").map(str::trim) {
        let mut parts = line.splitn(2, "-");
        graph.add_connection(&parts.next().unwrap(), &parts.next().unwrap());
    }
    return graph;
}

fn part1(input: String) -> i64 {
    let graph = parse_input(&input);
    let path: Vec<&str> = Vec::new();
    return count_paths_to_end(&graph, path, NAME_START, true).1;
}

fn part2(input: String) -> i64 {
    let graph = parse_input(&input);
    let path: Vec<&str> = Vec::new();
    return count_paths_to_end(&graph, path, NAME_START, false).1;
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

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
        let input = EXAMPLE_INPUT_1.to_string();
        let graph = parse_input(&input);
        assert_eq!(graph.get_connections("start"), &vec!["A", "b"]);
        assert_eq!(graph.get_connections("end"), &vec!["A", "b"]);
        assert_eq!(graph.get_connections("A"), &vec!["start", "c", "b", "end"]);
        assert_eq!(graph.get_connections("b"), &vec!["start", "A", "d", "end"]);
        assert_eq!(graph.get_connections("c"), &vec!["A"]);
        assert_eq!(graph.get_connections("d"), &vec!["b"]);
    }

    #[test]
    fn nodetype() {
        assert_eq!(NodeType::get(NAME_START), NodeType::Special);
        assert_eq!(NodeType::get(NAME_END), NodeType::Special);
        assert_eq!(NodeType::get("A"), NodeType::Big);
        assert_eq!(NodeType::get("b"), NodeType::Small);
        assert_eq!(NodeType::get("JK"), NodeType::Big);
        assert_eq!(NodeType::get("hl"), NodeType::Small);
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
