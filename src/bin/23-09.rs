use aoc::utils::parse;

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    parse!(input => {
        [lists split on '\n' with [split as isize]] 
    } => lists)
}

fn predict(sequence: &[isize]) -> (isize, isize) {
    let mut steps = Vec::new();
    let mut iter = sequence.iter();
    let mut cur = iter.next().unwrap();
    for next in iter {
        steps.push(next - cur);
        cur = next;
    }

    let step = if steps.iter().all(|v| v == &steps[0]) {
        (steps[0], steps[0])
    } else {
        predict(&steps)
    };
    (sequence[0] - step.0, cur + step.1)
}

pub fn part1(input: &str) -> isize {
    let lists = parse_input(input);
    lists.into_iter().map(|list| predict(&list).1).sum()
}

pub fn part2(input: &str) -> isize {
    let lists = parse_input(input);
    lists.into_iter().map(|list| predict(&list).0).sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 114, part2 = 2, test)]
    static EXAMPLE_INPUT: &str = "
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ];
        assert_eq!(actual, expected);
    }
}
