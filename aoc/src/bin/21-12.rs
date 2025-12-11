puzzle_runner::register_chapter!(book = 2021, title = "Passage Pathing");

use std::collections::HashMap;

const NAME_START: &str = "start";
const NAME_END: &str = "end";

#[derive(Debug, Eq, PartialEq)]
enum NodeType {
    Special,
    Big,
    Small,
}
impl NodeType {
    pub fn get(name: &str) -> NodeType {
        if name == NAME_START || name == NAME_END {
            NodeType::Special
        } else if name == name.to_uppercase() {
            NodeType::Big
        } else {
            NodeType::Small
        }
    }
}

#[derive(Debug, PartialEq)]
struct Graph<'a> {
    edges: HashMap<&'a str, HashMap<&'a str, u32>>,
}
impl<'a> Graph<'a> {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, left: &'a str, right: &'a str) {
        assert!(
            !(NodeType::get(left) == NodeType::Big && NodeType::get(right) == NodeType::Big),
            "Big caves may not be directly connected as this would create infinite paths, but {left} and {right} are.",
        );

        self.edges.entry(left).or_default().increment_one(right);
        self.edges.entry(right).or_default().increment_one(left);
    }

    pub fn flatten_big_nodes(&mut self) {
        let clone = self.edges.clone();
        for node in clone.keys() {
            if NodeType::get(node) != NodeType::Big {
                continue;
            }

            let edges = self.edges.remove(node).unwrap();
            for left in edges.keys() {
                self.edges.get_mut(left).unwrap().remove(node);
                for right in edges.keys() {
                    self.edges.get_mut(left).unwrap().increment_one(right);
                }
            }
        }
    }

    pub fn get_connections(&self, name: &'a str) -> &HashMap<&'a str, u32> {
        self.edges.get(name).unwrap()
    }
}

fn parse_input(input: &str) -> Graph<'_> {
    parse!(input => [pairs split on '\n' with { left '-' right } => (left, right)]);

    let mut graph = Graph::new();
    for (left, right) in pairs {
        graph.add_connection(left, right);
    }
    graph
}

fn count_paths_to_end<'a>(
    graph: &'a Graph,
    path: &mut Vec<&'a str>,
    node: &'a str,
    did_small_double_visit: bool,
) -> u32 {
    let mut results = 0u32;
    for (connected_node, weight) in graph.get_connections(node) {
        if *connected_node == NAME_START {
            continue;
        }
        if *connected_node == NAME_END {
            results += weight;
            continue;
        }

        let mut did_small_double_visit = did_small_double_visit;
        if NodeType::get(connected_node) == NodeType::Small && path.contains(connected_node) {
            if did_small_double_visit {
                continue;
            }
            did_small_double_visit = true;
        }

        path.push(connected_node);
        results += weight * count_paths_to_end(graph, path, connected_node, did_small_double_visit);
        path.pop();
    }
    results
}

#[register_part]
fn part1(input: &str) -> u32 {
    let mut graph = parse_input(input);
    graph.flatten_big_nodes();
    count_paths_to_end(&graph, &mut Vec::new(), NAME_START, true)
}

#[register_part]
fn part2(input: &str) -> u32 {
    let mut graph = parse_input(input);
    graph.flatten_big_nodes();
    count_paths_to_end(&graph, &mut Vec::new(), NAME_START, false)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 10, part2 = 36)]
    static EXAMPLE_INPUT_1: &str = "
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    ";
    #[example_input(part1 = 19, part2 = 103)]
    static EXAMPLE_INPUT_2: &str = "
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
    #[example_input(part1 = 226, part2 = 3509)]
    static EXAMPLE_INPUT_3: &str = "
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
        let graph = parse_input(&EXAMPLE_INPUT_1);
        assert_eq!(graph.get_connections("start").len(), 2);
        assert_eq!(graph.get_connections("start").get("A"), Some(&1));
        assert_eq!(graph.get_connections("start").get("b"), Some(&1));
        assert_eq!(graph.get_connections("end").len(), 2);
        assert_eq!(graph.get_connections("end").get("b"), Some(&1));
        assert_eq!(graph.get_connections("end").get("A"), Some(&1));
        assert_eq!(graph.get_connections("A").len(), 4);
        assert_eq!(graph.get_connections("A").get("start"), Some(&1));
        assert_eq!(graph.get_connections("A").get("c"), Some(&1));
        assert_eq!(graph.get_connections("A").get("b"), Some(&1));
        assert_eq!(graph.get_connections("A").get("end"), Some(&1));
        assert_eq!(graph.get_connections("b").len(), 4);
        assert_eq!(graph.get_connections("b").get("start"), Some(&1));
        assert_eq!(graph.get_connections("b").get("A"), Some(&1));
        assert_eq!(graph.get_connections("b").get("d"), Some(&1));
        assert_eq!(graph.get_connections("b").get("end"), Some(&1));
        assert_eq!(graph.get_connections("c").len(), 1);
        assert_eq!(graph.get_connections("c").get("A"), Some(&1));
        assert_eq!(graph.get_connections("d").len(), 1);
        assert_eq!(graph.get_connections("d").get("b"), Some(&1));
    }

    #[test]
    fn example1_flatten() {
        let mut graph = parse_input(&EXAMPLE_INPUT_1);
        graph.flatten_big_nodes();
        assert_eq!(graph.get_connections("start").len(), 4);
        assert_eq!(graph.get_connections("start").get("b"), Some(&2));
        assert_eq!(graph.get_connections("start").get("c"), Some(&1));
        assert_eq!(graph.get_connections("start").get("start"), Some(&1));
        assert_eq!(graph.get_connections("start").get("end"), Some(&1));
        assert_eq!(graph.get_connections("end").len(), 4);
        assert_eq!(graph.get_connections("end").get("b"), Some(&2));
        assert_eq!(graph.get_connections("end").get("c"), Some(&1));
        assert_eq!(graph.get_connections("end").get("start"), Some(&1));
        assert_eq!(graph.get_connections("end").get("end"), Some(&1));
        assert_eq!(graph.get_connections("b").len(), 5);
        assert_eq!(graph.get_connections("b").get("b"), Some(&1));
        assert_eq!(graph.get_connections("b").get("c"), Some(&1));
        assert_eq!(graph.get_connections("b").get("d"), Some(&1));
        assert_eq!(graph.get_connections("b").get("start"), Some(&2));
        assert_eq!(graph.get_connections("b").get("end"), Some(&2));
        assert_eq!(graph.get_connections("c").len(), 4);
        assert_eq!(graph.get_connections("c").get("b"), Some(&1));
        assert_eq!(graph.get_connections("c").get("c"), Some(&1));
        assert_eq!(graph.get_connections("c").get("start"), Some(&1));
        assert_eq!(graph.get_connections("c").get("end"), Some(&1));
        assert_eq!(graph.get_connections("d").len(), 1);
        assert_eq!(graph.get_connections("d").get("b"), Some(&1));
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
}
