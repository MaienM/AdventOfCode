use aoc::runner::*;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct Valve<'a> {
    flow: u16,
    tunnels: Vec<&'a str>,
}

type Valves<'a> = HashMap<&'a str, Valve<'a>>;

fn parse_input<'a>(input: &'a str) -> Valves<'a> {
    return input
        .trim()
        .split("\n")
        .map(|line| {
            let mut parts = line.trim().splitn(10, " ");
            let name = parts.nth(1).unwrap();
            let flow = parts
                .nth(2)
                .unwrap()
                .strip_prefix("rate=")
                .unwrap()
                .strip_suffix(';')
                .unwrap()
                .parse()
                .unwrap();
            let tunnels = parts.nth(4).unwrap().split(", ").collect();
            return (name, Valve { flow, tunnels });
        })
        .collect();
}

#[derive(Debug)]
struct State<'a> {
    position: &'a str,
    open: HashSet<&'a str>,
    flow: u16,
    release: u16,
}

fn run_cycle<'a>(states: Vec<State<'a>>, valves: &Valves<'a>) -> Vec<State<'a>> {
    let mut results = Vec::new();
    for state in states {
        let valve = valves.get(state.position).unwrap();
        let release = state.release + state.flow;

        if valve.flow > 0 && !state.open.contains(state.position) {
            let mut open = state.open.clone();
            open.insert(state.position);
            results.push(State {
                position: state.position,
                open,
                flow: state.flow + valve.flow,
                release,
            });
        }

        for target in valve.tunnels.iter() {
            results.push(State {
                position: target,
                open: state.open.clone(),
                flow: state.flow,
                release,
            });
        }
    }
    return results;
}

fn cull<'a>(mut states: Vec<State<'a>>, rounds_remaining: u16) -> Vec<State<'a>> {
    states.sort_by_key(|state| Reverse(state.release + state.flow * rounds_remaining));
    return states.into_iter().take(500).collect();
}

pub fn part1(input: String) -> u16 {
    let valves = parse_input(input.as_str());
    let mut states = vec![State {
        position: "AA",
        open: HashSet::new(),
        flow: 0,
        release: 0,
    }];
    for i in 0..30 {
        states = run_cycle(states, &valves);
        states = cull(states, 30 - i);
    }
    return states[0].release;
}

fn main() {
    run(part1, missing::<i64>);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT);
        let mut expected = HashMap::new();
        expected.insert(
            "AA",
            Valve {
                flow: 0,
                tunnels: vec!["DD", "II", "BB"],
            },
        );
        expected.insert(
            "BB",
            Valve {
                flow: 13,
                tunnels: vec!["CC", "AA"],
            },
        );
        expected.insert(
            "CC",
            Valve {
                flow: 2,
                tunnels: vec!["DD", "BB"],
            },
        );
        expected.insert(
            "DD",
            Valve {
                flow: 20,
                tunnels: vec!["CC", "AA", "EE"],
            },
        );
        expected.insert(
            "EE",
            Valve {
                flow: 3,
                tunnels: vec!["FF", "DD"],
            },
        );
        expected.insert(
            "FF",
            Valve {
                flow: 0,
                tunnels: vec!["EE", "GG"],
            },
        );
        expected.insert(
            "GG",
            Valve {
                flow: 0,
                tunnels: vec!["FF", "HH"],
            },
        );
        expected.insert(
            "HH",
            Valve {
                flow: 22,
                tunnels: vec!["GG"],
            },
        );
        expected.insert(
            "II",
            Valve {
                flow: 0,
                tunnels: vec!["AA", "JJ"],
            },
        );
        expected.insert(
            "JJ",
            Valve {
                flow: 21,
                tunnels: vec!["II"],
            },
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 1_651);
    }
}
