aoc::setup!(title = "PLACEHOLDER");

fn parse_input(input: &str) -> Vec<usize> {
    parse!(input => {
        [num split on '\n' as usize]
    } => num)
}

pub fn part1(input: &str) -> usize {
    let _ = parse_input(input);
    1
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 1)]
    static EXAMPLE_INPUT: &str = "
        1
        2
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![1, 2];
        assert_eq!(actual, expected);
    }
}
