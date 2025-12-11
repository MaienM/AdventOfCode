puzzle_runner::register_chapter!(book = "2024", title = "Historian Hysteria");

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

#[register_part]
fn part1(input: &str) -> usize {
    let (mut left, mut right) = parse_input(input);
    left.sort_unstable();
    right.sort_unstable();
    let mut diff = 0;
    for (l, r) in left.into_iter().zip(right.into_iter()) {
        diff += usize::max(l, r) - usize::min(l, r);
    }
    diff
}

#[register_part]
fn part2(input: &str) -> usize {
    let (left, right) = parse_input(input);
    let rcounts = right.into_iter().count_occurences();
    let mut score = 0;
    for l in left {
        score += l * *rcounts.get(&l).unwrap_or(&0);
    }
    score
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

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
