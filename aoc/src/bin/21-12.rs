use std::collections::HashMap;

use aoc::counter::Counter;
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
    edges: HashMap<&'a str, HashMap<&'a str, u32>>,
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
            self.edges.insert(left.clone(), HashMap::new());
        }
        self.edges.get_mut(left).unwrap().count(right.clone(), 1);

        if !self.edges.contains_key(right) {
            self.edges.insert(right.clone(), HashMap::new());
        }
        self.edges.get_mut(right).unwrap().count(left.clone(), 1);
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
                    self.edges.get_mut(left).unwrap().count(right, 1);
                }
            }
        }
    }

    pub fn get_connections(&self, name: &'a str) -> &HashMap<&'a str, u32> {
        return &self.edges.get(name).unwrap();
    }
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
        } else if *connected_node == NAME_END {
            results += weight;
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
        results += weight * count_paths_to_end(graph, path, connected_node, did_small_double_visit);
        path.pop();
    }
    return results;
}

fn parse_input<'a>(input: &'a String) -> Graph<'a> {
    let mut graph = Graph::new();
    for line in input.trim().split("\n").map(str::trim) {
        let mut parts = line.splitn(2, "-");
        graph.add_connection(&parts.next().unwrap(), &parts.next().unwrap());
    }
    return graph;
}

pub fn part1(input: String) -> u32 {
    let mut graph = parse_input(&input);
    graph.flatten_big_nodes();
    return count_paths_to_end(&graph, &mut Vec::new(), NAME_START, true);
}

pub fn part2(input: String) -> u32 {
    let mut graph = parse_input(&input);
    graph.flatten_big_nodes();
    return count_paths_to_end(&graph, &mut Vec::new(), NAME_START, false);
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
        let input = EXAMPLE_INPUT_1.to_string();
        let mut graph = parse_input(&input);
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
