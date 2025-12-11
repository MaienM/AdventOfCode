puzzle_runner::register_chapter!(book = 2023, title = "Pulse Propagation");

use std::collections::{HashMap, HashSet, VecDeque};

use num::integer;

#[derive(Debug, PartialEq)]
struct Input<'a> {
    broadcaster: Vec<&'a str>,
    modules: HashMap<&'a str, Module<'a>>,
}
impl Input<'_> {
    fn run_cycle(&mut self) -> (usize, usize) {
        let mut instructions: VecDeque<_> = self
            .broadcaster
            .iter()
            .map(|k| ("broadcaster", *k, false))
            .collect();
        let mut low_count = 1;
        let mut high_count = 0;
        while let Some((source, target, pulse)) = instructions.pop_front() {
            if pulse {
                high_count += 1;
            } else {
                low_count += 1;
            }
            let Some(module) = self.modules.get_mut(&target) else {
                continue;
            };
            for (next_target, next_pulse) in module.pulse(pulse, source) {
                instructions.push_back((target, next_target, next_pulse));
            }
        }
        (low_count, high_count)
    }
}

#[derive(Debug, PartialEq)]
struct Module<'a> {
    inputs: HashSet<&'a str>,
    outputs: Vec<&'a str>,
    ty: ModuleType<'a>,
}
impl<'a> Module<'a> {
    fn pulse(&mut self, pulse: bool, from: &'a str) -> Vec<(&'a str, bool)> {
        match self.ty {
            ModuleType::FlipFlop(ref mut state) => {
                if pulse {
                    Vec::new()
                } else {
                    *state = !*state;
                    self.outputs.iter().map(|k| (*k, *state)).collect()
                }
            }
            ModuleType::Conjunction(ref mut input_states) => {
                *input_states.get_mut(from).unwrap() = pulse;
                let pulse = !input_states.values().all(|last| *last);
                self.outputs.iter().map(|k| (*k, pulse)).collect()
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum ModuleType<'a> {
    FlipFlop(bool),
    Conjunction(HashMap<&'a str, bool>),
}

fn parse_input(input: &str) -> Input<'_> {
    parse!(input =>
        [modules split on '\n' into (HashMap<_, _>) with
            {
                [ty take 1 match {
                    "%" | "b" => ModuleType::FlipFlop(false),
                    "&" => ModuleType::Conjunction(HashMap::new()),
                }]
                name
                " -> "
                [outputs split on ", "]
            } => {
                (
                    name,
                    Module {
                        outputs,
                        ty,
                        inputs: HashSet::new(),
                    },
                )
            }
        ]
    );

    // Get broadcaster.
    let Some(Module {
        outputs: broadcaster,
        ..
    }) = modules.remove("roadcaster")
    else {
        panic!("Failed to parse broadcaster in input.");
    };

    // Determine inputs for each module.
    let mut module_inputs: HashMap<_, _> = modules.keys().map(|k| (*k, HashSet::new())).collect();
    for (name, module) in &modules {
        for output in &module.outputs {
            if let Some(module_inputs) = module_inputs.get_mut(output) {
                module_inputs.insert(*name);
            }
        }
    }
    for name in &broadcaster {
        if let Some(module_inputs) = module_inputs.get_mut(name) {
            module_inputs.insert("broadcaster");
        }
    }
    for (name, module) in &mut modules {
        module.inputs = module_inputs.remove(name).unwrap();
        if let ModuleType::Conjunction(ref mut input_states) = module.ty {
            *input_states = module.inputs.iter().map(|k| (*k, false)).collect();
        }
    }

    Input {
        broadcaster,
        modules,
    }
}

fn calculate_counter_period(input: &Input, start: &str) -> usize {
    let module = input.modules.get(start).unwrap();
    let mut sum = 0;
    for name in &module.outputs {
        match input.modules[name].ty {
            ModuleType::FlipFlop(_) => sum += calculate_counter_period(input, name) * 2,
            ModuleType::Conjunction(_) => sum += 1,
        }
    }
    sum
}

#[register_part]
fn part1(input: &str) -> usize {
    let mut input = parse_input(input);
    let mut low_total = 0;
    let mut high_total = 0;
    for _ in 0..1000 {
        let (low, high) = input.run_cycle();
        low_total += low;
        high_total += high;
    }
    low_total * high_total
}

// Each of the targets of the broadcaster is a separate subgraph. Each of these subgraphs contains a long chain of flip-flops and a single central conjunction, which some (but not all) the flip-flops connect to. Together these elements function as a counter, with each consecutive flip-flop represening another bit of the number. Once all the bits that connect back to the conjunction are set to true the conjunction will send out a pulse and then reset the counter.
//
// The conjunctions of these subgraphs combine in another conjunction that leads to the target, which will receive a pulse when all counters reset at the same time.
#[register_part]
fn part2(input: &str) -> usize {
    let input = parse_input(input);
    input
        .broadcaster
        .iter()
        .map(|start| calculate_counter_period(&input, start))
        .reduce(integer::lcm)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use common_macros::{hash_map, hash_set};
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 32_000_000)]
    static EXAMPLE_INPUT_1: &str = "
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    ";

    #[example_input(part1 = 11_687_500)]
    static EXAMPLE_INPUT_2: &str = "
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
    ";

    #[test]
    fn example_parse_1() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = Input {
            broadcaster: vec!["a", "b", "c"],
            modules: hash_map![
                "a" => Module {
                    inputs: hash_set![
                        "broadcaster",
                        "inv",
                    ],
                    outputs: vec!["b"],
                    ty: ModuleType::FlipFlop(false),
                },
                "b" => Module {
                    inputs: hash_set![
                        "a",
                        "broadcaster",
                    ],
                    outputs: vec!["c"],
                    ty: ModuleType::FlipFlop(false),
                },
                "c" => Module {
                    inputs: hash_set![
                        "b",
                        "broadcaster",
                    ],
                    outputs: vec!["inv"],
                    ty: ModuleType::FlipFlop(false),
                },
                "inv" => Module {
                    inputs: hash_set!["c"],
                    outputs: vec!["a"],
                    ty: ModuleType::Conjunction(hash_map![
                        "c" => false,
                    ]),
                },
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_2() {
        let actual = parse_input(&EXAMPLE_INPUT_2);
        let expected = Input {
            broadcaster: vec!["a"],
            modules: hash_map![
                "a" => Module {
                    inputs: hash_set!["broadcaster"],
                    outputs: vec!["inv", "con"],
                    ty: ModuleType::FlipFlop(false),
                },
                "inv" => Module {
                    inputs: hash_set!["a"],
                    outputs: vec!["b"],
                    ty: ModuleType::Conjunction(hash_map![
                        "a" => false,
                    ]),
                },
                "b" => Module {
                    inputs: hash_set!["inv"],
                    outputs: vec!["con"],
                    ty: ModuleType::FlipFlop(false),
                },
                "con" => Module {
                    inputs: hash_set!["a", "b"],
                    outputs: vec!["output"],
                    ty: ModuleType::Conjunction(hash_map![
                        "a" => false,
                        "b" => false,
                    ]),
                },
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_cycle_1() {
        let mut input = parse_input(&EXAMPLE_INPUT_1);
        assert_eq!(input.run_cycle(), (8, 4));
    }

    #[test]
    fn example_cycle_2() {
        let mut input = parse_input(&EXAMPLE_INPUT_2);
        assert_eq!(input.run_cycle(), (4, 4));
        assert_eq!(input.run_cycle(), (4, 2));
        assert_eq!(input.run_cycle(), (5, 3));
        assert_eq!(input.run_cycle(), (4, 2));
    }
}
