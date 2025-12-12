puzzle_runner::register_chapter!(book = 2025, title = "Factory");

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use good_lp::{
    Expression, ProblemVariables, Solution as _, SolverModel as _, default_solver, variable,
};

#[derive(Debug, Eq, PartialEq)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u16>,
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
                [joltages split on ',' as u16]
                '}'
            }
            => Machine { lights, buttons, joltages }
        ]
    } => machines)
}

fn find_fewest_presses_lights(machine: &Machine) -> usize {
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
    never!();
}

fn find_fewest_presses_joltages(machine: &Machine) -> usize {
    let mut vars = ProblemVariables::new();

    // Add variables for the buttons.
    let button_vars: Vec<_> = machine
        .buttons
        .iter()
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();

    // Setup problem with minimal presses as the goal.
    let mut problem = vars
        .minimise(button_vars.iter().sum::<Expression>())
        .using(default_solver);

    // Add expression for each joltage (sum of the presses of the buttons == joltage).
    for (jidx, joltage) in machine.joltages.iter().enumerate() {
        let expr = machine
            .buttons
            .iter()
            .enumerate()
            .filter_map(|(bidx, button)| {
                if button.contains(&jidx) {
                    Some(button_vars[bidx])
                } else {
                    None
                }
            })
            .sum::<Expression>();
        problem = problem.with(expr.eq(*joltage));
    }

    // Solve.
    let solution = problem.solve().unwrap();
    button_vars
        .into_iter()
        .map(|v| solution.value(v))
        .sum::<f64>()
        .round() as usize
}

#[register_part]
fn part1(input: &str) -> usize {
    let machines = parse_input(input);
    machines.par_iter().map(find_fewest_presses_lights).sum()
}

#[register_part]
fn part2(input: &str) -> usize {
    let machines = parse_input(input);
    machines.iter().map(find_fewest_presses_joltages).sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 7, part2 = 33)]
    static EXAMPLE_INPUT: &str = "
        [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    ";
}
