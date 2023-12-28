use std::collections::{HashMap, HashSet};

use aoc::utils::parse;

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse_input(input: &str) -> Graph {
    parse!(input =>
        [map split on '\n' into (HashMap<_, _>) with
            { from ": " [to split into (HashSet<_>)] }
            => (from, to)
        ]
    );

    for (from, to) in map.clone() {
        for name in to {
            map.entry(name).or_default().insert(from);
        }
    }
    map
}

fn cut(graph: &mut Graph, edge: (&str, &str)) {
    graph.get_mut(edge.0).unwrap().remove(edge.1);
    graph.get_mut(edge.1).unwrap().remove(edge.0);
}

fn count_reachable_from(graph: &Graph, start: &str) -> usize {
    let mut seen = HashSet::new();
    let mut todo = Vec::from([start]);
    while let Some(node) = todo.pop() {
        for edge in graph.get(node).unwrap() {
            if seen.insert(edge) {
                todo.push(edge);
            }
        }
    }
    seen.len()
}

fn get_within<'a>(
    graph: &Graph<'a>,
    start: &'a str,
    from: &'a str,
    steps: u32,
) -> HashSet<&'a str> {
    let mut nodes = HashSet::new();
    let mut stack = Vec::from([(start, steps)]);
    while let Some((node, steps)) = stack.pop() {
        if steps == 0 || node == from || !nodes.insert(node) {
            continue;
        }
        for next in graph.get(node).unwrap() {
            stack.push((next, steps - 1));
        }
    }
    nodes
}

fn edge_connectivity(graph: &Graph, steps: u32, edge: (&str, &str)) -> usize {
    let left = get_within(graph, edge.0, edge.1, steps);
    let right = get_within(graph, edge.1, edge.0, steps);
    left.len() + right.len() - left.intersection(&right).count() * 2
}

pub fn part1(input: &str) -> usize {
    let mut graph = parse_input(input);

    let steps = graph.len().ilog(3) + 1;
    let mut connected = Vec::new();
    for (from, edges) in &graph {
        for to in edges {
            // Avoid processing each edge twice needlessly.
            if from > to {
                continue;
            }
            let edge = (*from, *to);
            connected.push((edge_connectivity(&graph, steps, edge), edge));
        }
    }
    connected.sort_unstable();

    let mut start = "will_be_set_in_loop";
    for (_, edge) in connected.into_iter().rev().take(3) {
        cut(&mut graph, edge);
        start = edge.0;
    }

    let size_a = count_reachable_from(&graph, start);
    size_a * (graph.len() - size_a)
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use common_macros::{hash_map, hash_set};
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 54, test)]
    static EXAMPLE_INPUT: &str = "
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = hash_map![
            "bvb" => hash_set!["cmg", "hfx", "ntq", "rhn", "xhk"],
            "cmg" => hash_set!["bvb", "lhk", "nvd", "qnr", "rzs"],
            "frs" => hash_set!["lhk", "lsr", "qnr", "rsh"],
            "hfx" => hash_set!["bvb", "ntq", "pzl", "rhn", "xhk"],
            "jqt" => hash_set!["ntq", "nvd", "rhn", "xhk"],
            "lhk" => hash_set!["cmg", "frs", "lsr", "nvd"],
            "lsr" => hash_set!["frs", "lhk", "pzl", "rsh", "rzs"],
            "ntq" => hash_set!["bvb", "hfx", "jqt", "xhk"],
            "nvd" => hash_set!["cmg", "jqt", "lhk", "pzl", "qnr"],
            "pzl" => hash_set!["hfx", "lsr", "nvd", "rsh"],
            "qnr" => hash_set!["cmg", "frs", "nvd", "rzs"],
            "rhn" => hash_set!["bvb", "hfx", "jqt", "xhk"],
            "rsh" => hash_set!["frs", "lsr", "pzl", "rzs"],
            "rzs" => hash_set!["cmg", "lsr", "qnr", "rsh"],
            "xhk" => hash_set!["bvb", "hfx", "jqt", "ntq", "rhn"],
        ];
        assert_eq!(actual, expected);
    }
}
