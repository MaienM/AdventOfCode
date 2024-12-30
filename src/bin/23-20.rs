use std::collections::{HashMap, HashSet, VecDeque};

use aoc::utils::parse;
use num::integer;

#[derive(Debug, PartialEq)]
struct Input {
    broadcaster: Vec<String>,
    modules: HashMap<String, Module>,
}
impl Input {
    fn run_cycle(&mut self) -> (usize, usize) {
        let mut instructions: VecDeque<_> = self
            .broadcaster
            .iter()
            .map(|k| ("broadcaster".to_owned(), k.clone(), false))
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
            for (next_target, next_pulse) in module.pulse(pulse, &source) {
                instructions.push_back((target.clone(), next_target.clone(), next_pulse));
            }
        }
        (low_count, high_count)
    }
}

#[derive(Debug, PartialEq)]
struct Module {
    inputs: HashSet<String>,
    outputs: Vec<String>,
    ty: ModuleType,
}
#[derive(Debug, PartialEq)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}
impl Module {
    fn pulse(&mut self, pulse: bool, from: &String) -> Vec<(String, bool)> {
        match self.ty {
            ModuleType::FlipFlop(ref mut state) => {
                if pulse {
                    Vec::new()
                } else {
                    *state = !*state;
                    self.outputs
                        .iter()
                        .map(|k| (k.to_owned(), *state))
                        .collect()
                }
            }
            ModuleType::Conjunction(ref mut input_states) => {
                *input_states.get_mut(from).unwrap() = pulse;
                let pulse = !input_states.values().all(|last| *last);
                self.outputs.iter().map(|k| (k.clone(), pulse)).collect()
            }
        }
    }
}

fn parse_input(input: &str) -> Input {
    parse!(input =>
        [modules split on '\n' into (HashMap<_, _>) with
            { nametype " -> " [outputs split on ", " as String] }
            => {
                let ty = match &nametype[0..1] {
                    "%" | "b" => ModuleType::FlipFlop(false),
                    "&" => ModuleType::Conjunction(HashMap::new()),
                    prefix => panic!("Invalid prefix {prefix:?}."),
                };
                (
                    nametype[1..].to_owned(),
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
    let mut module_inputs: HashMap<_, _> = modules
        .keys()
        .map(|k| (k.clone(), HashSet::new()))
        .collect();
    for (name, module) in &modules {
        for output in &module.outputs {
            if let Some(module_inputs) = module_inputs.get_mut(output) {
                module_inputs.insert(name.clone());
            }
        }
    }
    for name in &broadcaster {
        if let Some(module_inputs) = module_inputs.get_mut(name) {
            module_inputs.insert("broadcaster".to_owned());
        }
    }
    for (name, module) in &mut modules {
        module.inputs = module_inputs.remove(name).unwrap();
        if let ModuleType::Conjunction(ref mut input_states) = module.ty {
            *input_states = module.inputs.iter().map(|k| (k.clone(), false)).collect();
        };
    }

    Input {
        broadcaster,
        modules,
    }
}

fn calculate_counter_period(input: &Input, start: &String) -> usize {
    let module = input.modules.get(start).unwrap();
    let mut sum = 0;
    for name in &module.outputs {
        match input.modules.get(name).unwrap().ty {
            ModuleType::FlipFlop(_) => sum += calculate_counter_period(input, name) * 2,
            ModuleType::Conjunction(_) => sum += 1,
        }
    }
    sum
}

pub fn part1(input: &str) -> usize {
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
pub fn part2(input: &str) -> usize {
    let input = parse_input(input);
    input
        .broadcaster
        .iter()
        .map(|start| calculate_counter_period(&input, start))
        .reduce(integer::lcm)
        .unwrap()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use common_macros::{hash_map, hash_set};
    use pretty_assertions::assert_eq;

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
            broadcaster: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            modules: hash_map![
                "a".to_owned() => Module {
                    inputs: hash_set![
                        "broadcaster".to_owned(),
                        "inv".to_owned(),
                    ],
                    outputs: vec!["b".to_owned()],
                    ty: ModuleType::FlipFlop(false),
                },
                "b".to_owned() => Module {
                    inputs: hash_set![
                        "a".to_owned(),
                        "broadcaster".to_owned(),
                    ],
                    outputs: vec!["c".to_owned()],
                    ty: ModuleType::FlipFlop(false),
                },
                "c".to_owned() => Module {
                    inputs: hash_set![
                        "b".to_owned(),
                        "broadcaster".to_owned(),
                    ],
                    outputs: vec!["inv".to_owned()],
                    ty: ModuleType::FlipFlop(false),
                },
                "inv".to_owned() => Module {
                    inputs: hash_set!["c".to_owned()],
                    outputs: vec!["a".to_owned()],
                    ty: ModuleType::Conjunction(hash_map![
                        "c".to_owned() => false,
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
            broadcaster: vec!["a".to_owned()],
            modules: hash_map![
                "a".to_owned() => Module {
                    inputs: hash_set!["broadcaster".to_owned()],
                    outputs: vec!["inv".to_owned(), "con".to_owned()],
                    ty: ModuleType::FlipFlop(false),
                },
                "inv".to_owned() => Module {
                    inputs: hash_set!["a".to_owned()],
                    outputs: vec!["b".to_owned()],
                    ty: ModuleType::Conjunction(hash_map![
                        "a".to_owned() => false,
                    ]),
                },
                "b".to_owned() => Module {
                    inputs: hash_set!["inv".to_owned()],
                    outputs: vec!["con".to_owned()],
                    ty: ModuleType::FlipFlop(false),
                },
                "con".to_owned() => Module {
                    inputs: hash_set!["a".to_owned(), "b".to_owned()],
                    outputs: vec!["output".to_owned()],
                    ty: ModuleType::Conjunction(hash_map![
                        "a".to_owned() => false,
                        "b".to_owned() => false,
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
