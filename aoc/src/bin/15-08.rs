puzzle_runner::register_chapter!(book = 2015, title = "Matchsticks");

fn count_unescape(input: &str) -> usize {
    let mut iter = input.chars();
    let mut len = 0;
    while let Some(chr) = iter.next() {
        len += 1;
        if chr == '\\' {
            match iter.next().unwrap() {
                'x' => {
                    iter.next();
                    iter.next();
                }
                '\\' | '"' => {}
                _ => {
                    len += 1;
                }
            }
        }
    }
    len - 2
}

fn count_escape(input: &str) -> usize {
    2 + input
        .chars()
        .map(|chr| match chr {
            '\\' | '"' => 2,
            _ => 1,
        })
        .sum::<usize>()
}

#[register_part]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.len() - count_unescape(line))
        .sum()
}

#[register_part]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| count_escape(line) - line.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 12, part2 = 19)]
    static EXAMPLE_INPUT: &str = r#"
        ""
        "abc"
        "aaa\"aaa"
        "\x27"
    "#;
}
