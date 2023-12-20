use std::collections::{HashMap, VecDeque};

use aoc::utils::parse;

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

    fn is_starting_state(&self) -> bool {
        self.modules.values().all(Module::is_starting_state)
    }
}

#[derive(Debug, PartialEq)]
enum Module {
    FlipFlop {
        outputs: Vec<String>,
        state: bool,
    },
    Conjunction {
        outputs: Vec<String>,
        inputs: HashMap<String, bool>,
    },
}
impl Module {
    fn pulse(&mut self, pulse: bool, from: &String) -> Vec<(String, bool)> {
        match self {
            Module::FlipFlop {
                outputs,
                ref mut state,
            } => {
                if pulse {
                    Vec::new()
                } else {
                    *state = !*state;
                    outputs.iter().map(|k| (k.to_owned(), *state)).collect()
                }
            }
            Module::Conjunction { outputs, inputs } => {
                *inputs.get_mut(from).unwrap() = pulse;
                let pulse = !inputs.values().all(|last| *last);
                outputs.iter().map(|k| (k.clone(), pulse)).collect()
            }
        }
    }

    fn is_starting_state(&self) -> bool {
        match self {
            Module::FlipFlop { state, .. } => !state,
            Module::Conjunction { inputs, .. } => inputs.values().all(|last| !*last),
        }
    }
}

fn parse_input(input: &str) -> Input {
    parse!(input =>
        [modules split on '\n' into (HashMap<_, _>) with
            { nametype " -> " [outputs split on ", " as String] }
            => {
                let module = match &nametype[0..1] {
                    "%" | "b" => Module::FlipFlop {
                        outputs,
                        state: false,
                    },
                    "&" => Module::Conjunction {
                        outputs,
                        inputs: HashMap::new(),
                    },
                    prefix => panic!("Invalid prefix {prefix:?}."),
                };
                (nametype[1..].to_owned(), module)
            }
        ]
    );

    let Module::FlipFlop {
        outputs: broadcaster,
        ..
    } = modules.remove(&"roadcaster".to_owned()).unwrap()
    else {
        panic!("Failed to parse broadcaster in input.");
    };

    let mut module_inputs: HashMap<_, _> =
        modules.keys().map(|k| (k.clone(), Vec::new())).collect();
    for (name, module) in &modules {
        let (Module::FlipFlop { outputs, .. } | Module::Conjunction { outputs, .. }) = module;
        for output in outputs {
            if let Some(module_inputs) = module_inputs.get_mut(output) {
                module_inputs.push(name.clone());
            }
        }
    }
    for (name, module) in &mut modules {
        let Module::Conjunction { ref mut inputs, .. } = module else {
            continue;
        };
        *inputs = module_inputs
            .remove(name)
            .unwrap()
            .into_iter()
            .map(|k| (k.clone(), false))
            .collect();
    }

    Input {
        broadcaster,
        modules,
    }
}

pub fn part1(input: &str) -> usize {
    let mut input = parse_input(input);
    let mut pulses = Vec::new();
    let mut cycle = 0;
    loop {
        cycle += 1;
        pulses.push(input.run_cycle());
        if input.is_starting_state() || cycle >= 1000 {
            break;
        }
    }
    let mut low_total = 0;
    let mut high_total = 0;
    for (idx, (low, high)) in pulses.into_iter().enumerate() {
        let repetitions = (999 + cycle - idx) / cycle;
        low_total += low * repetitions;
        high_total += high * repetitions;
    }
    low_total * high_total
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 32_000_000, test)]
    static EXAMPLE_INPUT_1: &str = "
        broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a
    ";

    #[example_input(part1 = 11_687_500, test)]
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
                "a".to_owned() => Module::FlipFlop {
                    outputs: vec!["b".to_owned()],
                    state: false,
                },
                "b".to_owned() => Module::FlipFlop {
                    outputs: vec!["c".to_owned()],
                    state: false,
                },
                "c".to_owned() => Module::FlipFlop {
                    outputs: vec!["inv".to_owned()],
                    state: false,
                },
                "inv".to_owned() => Module::Conjunction {
                    outputs: vec!["a".to_owned()],
                    inputs: hash_map![
                        "c".to_owned() => false,
                    ],
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
                "a".to_owned() => Module::FlipFlop {
                    outputs: vec!["inv".to_owned(), "con".to_owned()],
                    state: false,
                },
                "inv".to_owned() => Module::Conjunction {
                    outputs: vec!["b".to_owned()],
                    inputs: hash_map![
                        "a".to_owned() => false,
                    ],
                },
                "b".to_owned() => Module::FlipFlop {
                    outputs: vec!["con".to_owned()],
                    state: false,
                },
                "con".to_owned() => Module::Conjunction {
                    outputs: vec!["output".to_owned()],
                    inputs: hash_map![
                        "a".to_owned() => false,
                        "b".to_owned() => false,
                    ],
                },
            ],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_cycle_1() {
        let mut input = Input {
            broadcaster: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            modules: hash_map![
                "a".to_owned() => Module::FlipFlop {
                    outputs: vec!["b".to_owned()],
                    state: false,
                },
                "b".to_owned() => Module::FlipFlop {
                    outputs: vec!["c".to_owned()],
                    state: false,
                },
                "c".to_owned() => Module::FlipFlop {
                    outputs: vec!["inv".to_owned()],
                    state: false,
                },
                "inv".to_owned() => Module::Conjunction {
                    outputs: vec!["a".to_owned()],
                    inputs: hash_map![
                        "c".to_owned() => false,
                    ],
                },
            ],
        };
        assert!(input.is_starting_state());
        assert_eq!(input.run_cycle(), (8, 4));
        assert!(input.is_starting_state());
    }

    #[test]
    fn example_cycle_2() {
        let mut input = Input {
            broadcaster: vec!["a".to_owned()],
            modules: hash_map![
                "a".to_owned() => Module::FlipFlop {
                    outputs: vec!["inv".to_owned(), "con".to_owned()],
                    state: false,
                },
                "inv".to_owned() => Module::Conjunction {
                    outputs: vec!["b".to_owned()],
                    inputs: hash_map![
                        "a".to_owned() => false,
                    ],
                },
                "b".to_owned() => Module::FlipFlop {
                    outputs: vec!["con".to_owned()],
                    state: false,
                },
                "con".to_owned() => Module::Conjunction {
                    outputs: vec!["output".to_owned()],
                    inputs: hash_map![
                        "a".to_owned() => false,
                        "b".to_owned() => false,
                    ],
                },
            ],
        };
        assert!(input.is_starting_state());
        assert_eq!(input.run_cycle(), (4, 4));
        assert!(!input.is_starting_state());
        assert_eq!(input.run_cycle(), (4, 2));
        assert!(!input.is_starting_state());
        assert_eq!(input.run_cycle(), (5, 3));
        assert!(!input.is_starting_state());
        assert_eq!(input.run_cycle(), (4, 2));
        assert!(input.is_starting_state());
    }
}
