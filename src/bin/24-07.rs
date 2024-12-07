use aoc::utils::parse;

type Equation = (usize, Vec<usize>);

fn parse_input(input: &str) -> Vec<Equation> {
    parse!(input => {
        [equations split on '\n' with
            { [test as usize] ": " [numbers split as usize] }
            => (test, numbers)
        ]
    } => equations)
}

fn test_inner(equation: &Equation, running_total: usize, offset: usize, concat: bool) -> bool {
    if offset == equation.1.len() {
        return running_total == equation.0;
    }
    if running_total > equation.0 {
        return false;
    }

    let next = equation.1[offset];
    let offset = offset + 1;

    test_inner(equation, running_total * next, offset, concat)
        || test_inner(equation, running_total + next, offset, concat)
        || (concat
            && test_inner(
                equation,
                format!("{running_total}{next}").parse().unwrap(),
                offset,
                concat,
            ))
}

fn test(equation: &Equation, concat: bool) -> bool {
    test_inner(equation, equation.1[0], 1, concat)
}

pub fn part1(input: &str) -> usize {
    let equations = parse_input(input);
    equations
        .into_iter()
        .filter(|e| test(e, false))
        .map(|e| e.0)
        .sum()
}

pub fn part2(input: &str) -> usize {
    let equations = parse_input(input);
    equations
        .into_iter()
        .filter(|e| test(e, true))
        .map(|e| e.0)
        .sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 3749, part2 = 11_387)]
    static EXAMPLE_INPUT: &str = "
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            (190, vec![10, 19]),
            (3267, vec![81, 40, 27]),
            (83, vec![17, 5]),
            (156, vec![15, 6]),
            (7290, vec![6, 8, 6, 15]),
            (16_1011, vec![16, 10, 13]),
            (192, vec![17, 8, 14]),
            (21_037, vec![9, 7, 18, 13]),
            (292, vec![11, 6, 16, 20]),
        ];
        assert_eq!(actual, expected);
    }
}
