use aoc::utils::parse;

enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}
impl From<char> for Color {
    fn from(value: char) -> Self {
        match value {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            _ => panic!(),
        }
    }
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    parse!(input => {
        [patterns split on ", "]
        "\n\n"
        [designs split on '\n']
    } => (patterns, designs))
}

fn try_design(design: &str, idx: usize, patterns: &[&str]) -> bool {
    if idx >= design.len() - 1 {
        return true;
    }
    for pattern in patterns {
        let l = idx + pattern.len();
        if l < design.len() && *pattern == &design[idx..l] && try_design(design, l, patterns) {
            return true;
        }
    }
    false
}

pub fn part1(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    designs
        .into_iter()
        .filter(|d| try_design(d, 0, &patterns))
        .count()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 6)]
    static EXAMPLE_INPUT: &str = "
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
    ";

    // #[test]
    // fn example_parse() {
    //     let actual = parse_input(&EXAMPLE_INPUT);
    //     let expected = vec![1, 2];
    //     assert_eq!(actual, expected);
    // }
}
