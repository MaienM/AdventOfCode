use std::collections::{HashMap, HashSet};

use aoc::utils::parse;
use itertools::Itertools;
use rayon::prelude::*;

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    parse!(input => {
        [pairs split on '\n' with
            { lhs '-' rhs }
            => (lhs, rhs)
        ]
    } => pairs)
}

pub fn part1(input: &str) -> usize {
    let pairs = parse_input(input);
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for (lhs, rhs) in pairs {
        graph.entry(lhs).or_default().push(rhs);
        graph.entry(rhs).or_default().push(lhs);
    }
    graph
        .keys()
        .combinations(3)
        .par_bridge()
        .filter(|keys| {
            let e0 = graph.get(keys[0]).unwrap();
            let e1 = graph.get(keys[1]).unwrap();
            keys.iter().any(|k| k.starts_with('t'))
                && e0.contains(keys[1])
                && e0.contains(keys[2])
                && e1.contains(keys[2])
        })
        .count()
}

pub fn part2(input: &str) -> String {
    let pairs = parse_input(input);
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (lhs, rhs) in pairs {
        graph.entry(lhs).or_default().insert(rhs);
        graph.entry(rhs).or_default().insert(lhs);
    }
    for (key, edges) in &mut graph {
        edges.insert(key);
    }
    let group = graph
        .iter()
        .par_bridge()
        .flat_map(|(_, edges)| {
            edges.iter().powerset().par_bridge().filter(|keys| {
                let keys: HashSet<_> = keys.iter().copied().copied().collect();
                keys.iter()
                    .all(|key| graph.get(*key).unwrap().intersection(&keys).count() == keys.len())
            })
        })
        .max_by_key(Vec::len)
        .unwrap();
    group.into_iter().sorted_unstable().join(",")
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 7, part2 = "co,de,ka,ta")]
    static EXAMPLE_INPUT: &str = "
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    ";
}
