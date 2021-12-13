use aoc::runner::*;

fn parse_input(input: String) -> Vec<String> {
    return input
        .trim()
        .split("\n")
        .map(|line| line.trim().to_string())
        .collect();
}

#[inline]
fn get_matching_closing(chr: char) -> Option<char> {
    return match chr {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    };
}

fn part1(input: String) -> i64 {
    let lines = parse_input(input);
    let mut score = 0_i64;
    for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for chr in line.chars() {
            let closing = get_matching_closing(chr);
            if closing.is_some() {
                stack.push(closing.unwrap());
            } else {
                let expected = stack.pop().unwrap_or('!');
                if chr != expected {
                    score += match chr {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => {
                            panic!("Invalid character {}.", chr);
                        }
                    };
                    break;
                }
            }
        }
    }
    return score;
}

fn part2(input: String) -> i64 {
    let lines = parse_input(input);
    let mut scores: Vec<i64> = Vec::new();
    'lines: for line in lines {
        let mut stack: Vec<char> = Vec::new();
        for chr in line.chars() {
            let closing = get_matching_closing(chr);
            if closing.is_some() {
                stack.push(closing.unwrap());
            } else {
                let expected = stack.pop().unwrap_or('!');
                if chr != expected {
                    continue 'lines;
                }
            }
        }

        let mut score = 0_i64;
        for chr in stack.into_iter().rev() {
            score *= 5;
            score += match chr {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => {
                    panic!("Invalid character {}.", chr);
                }
            };
        }
        scores.push(score);
    }
    scores.sort();
    return scores[scores.len() / 2];
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "
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
            parse_input(EXAMPLE_INPUT.to_string()),
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

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 26397);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 288957);
    }
}
