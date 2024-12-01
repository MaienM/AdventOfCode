use aoc::utils::parse;

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut leftlist = Vec::new();
    let mut rightlist = Vec::new();
    for line in input.split('\n') {
        parse!(line =>
            [left as usize] "   " [right as usize]
        );
        leftlist.push(left);
        rightlist.push(right);
    }
    (leftlist, rightlist)
}

pub fn part1(input: &str) -> usize {
    let (mut left, mut right) = parse_input(input);
    left.sort_unstable();
    right.sort_unstable();
    let mut diff = 0;
    for (l, r) in left.into_iter().zip(right.into_iter()) {
        diff += usize::max(l, r) - usize::min(l, r);
    }
    diff
}

pub fn part2(input: &str) -> usize {
    let (left, right) = parse_input(input);
    let mut score = 0;
    for l in left {
        score += l * right.iter().filter(|r| *r == &l).count();
    }
    score
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 11, part2 = 31)]
    static EXAMPLE_INPUT: &str = "
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(actual, expected);
    }
}
