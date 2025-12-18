puzzle_runner::register_chapter!(title = "Medicine for Rudolph");

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn parse_input(input: &str) -> (Vec<(&str, &str)>, &str) {
    parse!(input => {
        [replacements split on '\n' with
            {
                lhs
                " => "
                rhs
            }
            => (lhs, rhs)
        ]
        "\n\n"
        molecule
    } => (replacements, molecule))
}

#[inline]
fn for_each_replacement_option<F, R>(molecule: &str, from: &str, to: &str, mut f: F)
where
    F: FnMut(String) -> R,
{
    for (idx, _) in molecule.match_indices(from) {
        let (left, right) = molecule.split_at(idx + from.len());
        let result = format!("{}{}{}", &left[..idx], to, right);
        f(result);
    }
}

#[register_part]
fn part1(input: &str) -> usize {
    let (replacements, molecule) = parse_input(input);
    let mut options = HashSet::new();
    for (from, to) in replacements {
        for_each_replacement_option(molecule, from, to, |v| options.insert(v));
    }
    options.len()
}

#[register_part]
fn part2(input: &str) -> usize {
    let (replacements, molecule) = parse_input(input);
    let mut seen = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(molecule.len()), Reverse(0), molecule.to_owned()));
    while let Some((_, Reverse(steps), molecule)) = heap.pop() {
        if molecule == "e" {
            return steps;
        }
        for (from, to) in &replacements {
            for_each_replacement_option(&molecule, to, from, |v| {
                if seen.insert(v.clone()) {
                    heap.push((Reverse(v.len()), Reverse(steps + 1), v));
                }
            });
        }
    }
    never!();
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 4, part2 = 3)]
    static EXAMPLE_INPUT_1: &str = "
        H => HO
        H => OH
        O => HH
        e => H
        e => O

        HOH
    ";

    #[example_input(part1 = 7, part2 = 6)]
    static EXAMPLE_INPUT_2: &str = "
        H => HO
        H => OH
        O => HH
        e => H
        e => O

        HOHOHO
    ";
}
