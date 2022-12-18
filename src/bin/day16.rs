use aoc::runner::*;
use std::collections::BinaryHeap;
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

#[derive(Debug, Eq, PartialEq)]
struct ValveWithPaths<'a> {
    flow: u16,
    paths: HashMap<&'a str, u16>,
}

type ValvesWithPaths<'a> = HashMap<&'a str, ValveWithPaths<'a>>;

fn find_path<'a>(valves: &Valves<'a>, start: &'a str, target: &'a str) -> u16 {
    let mut paths: BinaryHeap<(u16, &'a str)> = BinaryHeap::new();
    paths.push((100, start));
    loop {
        let (cost, current) = paths.pop().unwrap();
        let valve = &valves[current];
        let cost = cost - 1;
        for tunneltarget in valve.tunnels.iter() {
            if &target == tunneltarget {
                return 100 - cost;
            }
            paths.push((cost, tunneltarget));
        }
    }
}

fn calculate_paths<'a>(valves: &Valves<'a>) -> ValvesWithPaths<'a> {
    let mut paths: HashMap<&'a str, HashMap<&'a str, u16>> =
        valves.keys().map(|k| (*k, HashMap::new())).collect();
    for (tname, tvalve) in valves {
        if tvalve.flow == 0 {
            // No point in ever routing to a zero-flow valve, these are only visited on the way to
            // something useful.
            continue;
        }
        for sname in valves.keys() {
            if sname == tname {
                // No need to route to yourself.
                continue;
            }
            paths
                .get_mut(sname)
                .unwrap()
                .insert(tname, find_path(valves, sname, tname));
        }
    }

    return valves
        .into_iter()
        .map(|(k, v)| {
            (
                *k,
                ValveWithPaths {
                    flow: v.flow,
                    paths: paths.remove(k).unwrap(),
                },
            )
        })
        .collect();
}

struct GlobalState<'a> {
    valves: &'a ValvesWithPaths<'a>,
    max_flow: u16,
    best_so_far: u16,
}

#[derive(Clone, Debug)]
struct State<'a, const C: usize> {
    actors: [(&'a str, u16); C],
    closed: HashSet<&'a str>,
    cycles: u16,
    flow: u16,
    total: u16,
}
impl<'a, const C: usize> State<'a, C> {
    pub fn is_done(&self) -> bool {
        return self.closed.is_empty();
    }

    pub fn is_dead(&self, gstate: &GlobalState) -> bool {
        let approx_best = self.total + self.cycles * (gstate.max_flow - self.flow);
        return approx_best <= gstate.best_so_far;
    }
}

fn start_actor_move<'a, const C: usize>(
    gstate: &mut GlobalState<'a>,
    state: &mut State<'a, C>,
    idx: usize,
    destination: &'a str,
    distance: u16,
) {
    let destination_valve = &gstate.valves[destination];
    state.actors[idx] = (destination, distance);
    state.closed.remove(destination);
    state.flow += destination_valve.flow;
    state.total += (state.cycles - distance - 1) * destination_valve.flow;
}

fn run_cycle_single_actor<'a, const C: usize>(
    gstate: &mut GlobalState<'a>,
    mut state: State<'a, C>,
    idx: usize,
) {
    if idx >= C {
        state.cycles -= 1;
        run_cycle_single_actor(gstate, state, 0);
        return;
    }

    match state.actors[idx] {
        (valve, 0) => {
            let valve = &gstate.valves[valve];
            for destination in state.closed.iter() {
                let distance = valve.paths[destination];
                if distance >= state.cycles {
                    gstate.best_so_far = u16::max(gstate.best_so_far, state.total);
                    continue;
                }

                let mut state = state.clone();
                start_actor_move(gstate, &mut state, idx, destination, distance);

                if state.is_done() {
                    gstate.best_so_far = u16::max(gstate.best_so_far, state.total);
                    continue;
                }
                if state.is_dead(&gstate) {
                    continue;
                }

                run_cycle_single_actor(gstate, state, idx + 1);
            }
        }
        _ => {
            state.actors[idx].1 -= 1;
            run_cycle_single_actor(gstate, state, idx + 1);
        }
    };
}

fn run_cycles<'a, const C: usize>(valves: &Valves<'a>, cycles: u16) -> u16 {
    let state = State {
        actors: [("AA", 0); C],
        closed: valves
            .into_iter()
            .filter(|(_k, v)| v.flow > 0)
            .map(|(k, _v)| *k)
            .collect(),
        cycles,
        flow: 0,
        total: 0,
    };
    let valves = calculate_paths(&valves);
    let mut global_state = GlobalState {
        valves: &valves,
        max_flow: valves.values().map(|v| v.flow).sum(),
        best_so_far: 0,
    };

    // Given that all actors are identical we want to avoid simulating both the case where the
    // targets after the first cycle are 1->A,2->B and 1->B,2->A, as these are identical.
    // TODO:

    run_cycle_single_actor(&mut global_state, state, 0);
    return global_state.best_so_far;
}

pub fn part1(input: String) -> u16 {
    let valves = parse_input(input.as_str());
    return run_cycles::<1>(&valves, 30);
}

pub fn part2(input: String) -> u16 {
    let valves = parse_input(input.as_str());
    return run_cycles::<2>(&valves, 26);
}

fn main() {
    run(part1, part2);
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

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 1_707);
    }
}
