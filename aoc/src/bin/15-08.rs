puzzle_lib::setup!(title = "Matchsticks");

fn count_characters(input: &str) -> usize {
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

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.len() - count_characters(line))
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 12)]
    static EXAMPLE_INPUT: &str = r#"
        ""
        "abc"
        "aaa\"aaa"
        "\x27"
    "#;
}
