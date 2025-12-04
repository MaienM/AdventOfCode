puzzle_runner::register_chapter!(book = "2024", title = "Linen Layout");

use std::collections::HashMap;

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

fn count_designs(cache: &mut HashMap<String, usize>, design: &str, patterns: &[&str]) -> usize {
    if design.is_empty() {
        return 1;
    }
    let dkey = design.to_owned();
    if cache.contains_key(&dkey) {
        return *cache.get(&dkey).unwrap();
    }
    let mut count = 0;
    for pattern in patterns {
        if pattern.len() <= design.len() && *pattern == &design[..pattern.len()] {
            count += count_designs(cache, &design[pattern.len()..], patterns);
        }
    }
    cache.insert(dkey, count);
    count
}

pub fn part2(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    let mut cache = HashMap::new();
    designs
        .into_iter()
        .map(|d| count_designs(&mut cache, d, &patterns))
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 6, part2 = 16)]
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
