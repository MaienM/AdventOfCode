puzzle_runner::register_chapter!(book = 2025, title = "Medicine for Rudolph");

use std::collections::HashSet;

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

#[register_part]
fn part1(input: &str) -> usize {
    let (replacements, molecule) = parse_input(input);
    let mut options = HashSet::new();
    for replacement in replacements {
        for (idx, _) in molecule.match_indices(replacement.0) {
            let (left, right) = molecule.split_at(idx + replacement.0.len());
            let result = format!("{}{}{}", &left[..idx], replacement.1, right);
            options.insert(result);
        }
    }
    options.len()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 4)]
    static EXAMPLE_INPUT_1: &str = "
        H => HO
        H => OH
        O => HH

        HOH
    ";

    #[example_input(part1 = 7)]
    static EXAMPLE_INPUT_2: &str = "
        H => HO
        H => OH
        O => HH

        HOHOHO
    ";
}
