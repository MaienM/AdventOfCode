puzzle_runner::register_chapter!(book = "2025", title = "Factory");

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

#[derive(Debug, Eq, PartialEq)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    requirements: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    parse!(input => {
        [machines split on '\n' with
            {
                '['
                [lights chars with |c| c == '#']
                "] "
                [buttons split on ' ' with
                    {
                        '('
                        [indexes split on ',' as usize]
                        ')'
                    }
                    => indexes
                ]
                " {"
                [requirements split on ',' as usize]
                '}'
            }
            => Machine { lights, buttons, requirements }
        ]
    } => machines)
}

fn find_fewest_presses(machine: &Machine) -> usize {
    let mut target = 0;
    for (idx, state) in machine.lights.iter().enumerate() {
        if *state {
            target |= 1 << idx;
        }
    }

    let mut seen = HashSet::new();
    seen.insert(0);
    let mut states = BinaryHeap::new();
    states.push((Reverse(0), 0, 100));
    while let Some((Reverse(presses), lights, last_idx)) = states.pop() {
        if lights == target {
            return presses;
        }

        for (idx, button) in machine.buttons.iter().enumerate() {
            if idx == last_idx {
                continue;
            }
            let mut lights = lights;
            for lidx in button {
                lights ^= 1 << lidx;
            }
            if !seen.insert(lights) {
                continue;
            }
            states.push((Reverse(presses + 1), lights, idx));
        }
    }
    panic!("Should never happen.");
}

pub fn part1(input: &str) -> usize {
    let machines = parse_input(input);
    machines.iter().map(find_fewest_presses).sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 7)]
    static EXAMPLE_INPUT: &str = "
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    ";
}
