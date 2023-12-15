use aoc::utils::parse;

fn parse_input(input: &str) -> Vec<&str> {
    parse!(input => {
        [steps split on ',']
    } => steps)
}

fn hash(value: &str) -> usize {
    let mut hash = 0;
    for chr in value.chars() {
        hash += chr as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

pub fn part1(input: &str) -> usize {
    let steps = parse_input(input);
    steps.into_iter().map(hash).sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 1320, test)]
    static EXAMPLE_INPUT: &str = "
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            "rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7",
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn hash() {
        assert_eq!(super::hash("HASH"), 52);
    }
}
