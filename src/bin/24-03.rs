use aoc::utils::parse;

#[derive(Debug, Eq, PartialEq)]
struct Mul(usize, usize);
impl TryFrom<&str> for Mul {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut input = value.split(')');
        let mut parts = input.next().ok_or("")?.splitn(2, ',');
        let left = parts.next().ok_or("")?.parse().map_err(|_| "")?;
        let right = parts.next().ok_or("")?.parse().map_err(|_| "")?;
        Ok(Self(left, right))
    }
}

fn parse_input(input: &str) -> Vec<Mul> {
    parse!(input => {
        [instructions split on "mul(" try as Mul]
    } => instructions)
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    input.into_iter().map(|Mul(l, r)| l * r).sum()
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 161)]
    static EXAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![Mul(2, 4), Mul(5, 5), Mul(11, 8), Mul(8, 5)];
        assert_eq!(actual, expected);
    }
}
