puzzle_runner::register_chapter!(book = "2021", title = "Syntax Scoring");

fn parse_input(input: &str) -> Vec<&str> {
    parse!(input => { [lines split on '\n'] } => lines)
}

#[inline]
fn get_matching_closing(chr: char) -> Option<char> {
    match chr {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

pub fn part1(input: &str) -> u64 {
    let lines = parse_input(input);
    let mut score = 0u64;
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for chr in line.chars() {
            if let Some(closing) = get_matching_closing(chr) {
                stack.push(closing);
            } else {
                let expected = stack.pop().unwrap_or('!');
                if chr != expected {
                    score += match chr {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => {
                            panic!("Invalid character {chr}.");
                        }
                    };
                    break;
                }
            }
        }
    }
    score
}

pub fn part2(input: &str) -> u64 {
    let lines = parse_input(input);
    let mut scores: Vec<u64> = Vec::new();
    'lines: for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for chr in line.chars() {
            if let Some(closing) = get_matching_closing(chr) {
                stack.push(closing);
            } else {
                let expected = stack.pop().unwrap_or('!');
                if chr != expected {
                    continue 'lines;
                }
            }
        }

        let mut score = 0u64;
        for chr in stack.into_iter().rev() {
            score *= 5;
            score += match chr {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => {
                    panic!("Invalid character {chr}.");
                }
            };
        }
        scores.push(score);
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 26397, part2 = 288957)]
    static EXAMPLE_INPUT: &str = "
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    ";

    #[test]
    fn example_parse() {
        assert_eq!(
            parse_input(&EXAMPLE_INPUT),
            vec![
                "[({(<(())[]>[[{[]{<()<>>",
                "[(()[<>])]({[<{<<[]>>(",
                "{([(<{}[<>[]}>{[]{[(<()>",
                "(((({<>}<{<{<>}{[]{[]{}",
                "[[<[([]))<([[{}[[()]]]",
                "[{[{({}]{}}([{[{{{}}([]",
                "{<[[]]>}<{[{[{[]{()[[[]",
                "[<(<(<(<{}))><([]([]()",
                "<{([([[(<>()){}]>(<<{{",
                "<{([{{}}[<[[[<>{}]]]>[]]",
            ]
        );
    }
}
