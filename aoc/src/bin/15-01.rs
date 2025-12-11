puzzle_runner::register_chapter!(book = "2015", title = "Not Quite Lisp");

fn parse_input(input: &str) -> Vec<i16> {
    parse!(input => {
        [moves chars match { '(' => 1, ')' => -1 }]
    } => moves)
}

#[register_part]
fn part1(input: &str) -> i16 {
    let moves = parse_input(input);
    moves.into_iter().sum()
}

#[register_part]
fn part2(input: &str) -> usize {
    let moves = parse_input(input);
    let mut floor = 0;
    for (i, mov) in moves.into_iter().enumerate() {
        floor += mov;
        if floor < 0 {
            return i + 1;
        }
    }
    panic!("Should never happen.");
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 0)]
    static EXAMPLE_INPUT_1: &str = "(())";

    #[example_input(part1 = 3)]
    static EXAMPLE_INPUT_2: &str = "(()(()(";

    #[example_input(part1 = "-3", part2 = 1)]
    static EXAMPLE_INPUT_3: &str = ")())())";

    #[test]
    fn example_parse_1() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = vec![1, 1, -1, -1];
        assert_eq!(actual, expected);
    }
}
