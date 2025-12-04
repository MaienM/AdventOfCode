puzzle_runner::register_chapter!(book = "2024", title = "Mull It Over");

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Mul(usize, usize),
    Conditional(bool),
}
impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tmp = value.rsplitn(2, '(');
        let args = tmp.next().unwrap();
        let prefix = tmp.next().ok_or("")?;

        if prefix.ends_with("do") && args.is_empty() {
            Ok(Self::Conditional(true))
        } else if prefix.ends_with("don't") && args.is_empty() {
            Ok(Self::Conditional(false))
        } else if prefix.ends_with("mul") {
            let mut parts = args.splitn(2, ',');
            let left = parts.next().ok_or("")?.parse().map_err(|_| "")?;
            let right = parts.next().ok_or("")?.parse().map_err(|_| "")?;
            Ok(Self::Mul(left, right))
        } else {
            Err("")
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    parse!(input => { [instructions split on ')' try as Instruction] } => instructions)
}

pub fn part1(input: &str) -> usize {
    let instructions = parse_input(input);
    instructions
        .into_iter()
        .map(|i| match i {
            Instruction::Mul(l, r) => l * r,
            Instruction::Conditional(_) => 0,
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut result = 0;
    let mut enabled = true;
    for instruction in instructions {
        match instruction {
            Instruction::Mul(l, r) => {
                if enabled {
                    result += l * r;
                }
            }
            Instruction::Conditional(e) => {
                enabled = e;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = 161)]
    static EXAMPLE_INPUT_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[example_input(part2 = 48)]
    static EXAMPLE_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn example_parse_1() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = vec![
            Instruction::Mul(2, 4),
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Mul(8, 5),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_parse_2() {
        let actual = parse_input(&EXAMPLE_INPUT_2);
        let expected = vec![
            Instruction::Mul(2, 4),
            Instruction::Conditional(false),
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Conditional(true),
            Instruction::Mul(8, 5),
        ];
        assert_eq!(actual, expected);
    }
}
