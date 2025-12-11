puzzle_runner::register_chapter!(book = "2024", title = "LAN Party");

use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    parse!(input => {
        [pairs split on '\n' with
            { lhs '-' rhs }
            => (lhs, rhs)
        ]
    } => pairs)
}

#[register_part]
fn part1(input: &str) -> usize {
    let pairs = parse_input(input);
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for (lhs, rhs) in pairs {
        graph.entry(lhs).or_default().push(rhs);
        graph.entry(rhs).or_default().push(lhs);
    }
    graph
        .keys()
        .par_bridge()
        .filter(|k| k.starts_with('t'))
        .flat_map(|k| {
            graph
                .get(k)
                .unwrap()
                .iter()
                .filter(|k2| !k2.starts_with('t') || *k2 < k)
                .combinations(2)
                .collect::<Vec<_>>()
        })
        .filter(|keys| graph.get(keys[0]).unwrap().contains(keys[1]))
        .count()
}

#[register_part]
fn part2(input: &str) -> String {
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
        .par_iter()
        .filter(|(key, edges)| edges.iter().all(|k| k <= *key))
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

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
