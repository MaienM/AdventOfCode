aoc::setup!(title = "Snowverload");

use std::collections::{HashMap, HashSet};

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

pub fn part1(input: &str) -> usize {
    let graph = parse_input(input);

    // Flood fill starting at a random node, continuing until there are exactly three unvisited neighbors, all of which neighbor only a single of our currently covered nodes.
    // This will work when starting from most nodes, but we attempt all nodes until we get a success just in case the first one we try is one of the ones that doesn't work (e.g. a node that is directly adjacent to one of the cuts that should be made). In most cases this will only end up running for a single node.
    graph
        .iter()
        .find_map(|first| {
            let mut group = HashSet::from([first.0]);
            let mut unvisited: HashMap<_, _> = first.1.iter().map(|e| (e, 2)).collect();

            loop {
                let Some((node, count)) = unvisited.iter().max_by_key(|(_, c)| *c) else {
                    break;
                };
                if *count <= 1 && unvisited.len() == 3 {
                    break;
                }

                let node = *node;
                unvisited.remove(&node);
                group.insert(node);

                for edge in graph.get(node).unwrap() {
                    if !group.contains(edge) {
                        *unvisited.entry(edge).or_default() += 1;
                    }
                }
            }

            if unvisited.len() == 3 {
                let size = group.len();
                Some(size * (graph.len() - size))
            } else {
                None
            }
        })
        .unwrap()
}

pub fn part2(_input: &str) -> &'static str {
    "I did it!"
}

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use common_macros::{hash_map, hash_set};
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 54)]
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
