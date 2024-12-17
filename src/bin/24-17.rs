use aoc::utils::parse;

type Registers = [usize; 3];

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid opcode {value}."),
        }
    }
}

fn parse_input(input: &str) -> (Registers, Vec<u8>) {
    parse!(input =>
        "Register A: " [ra as usize] '\n'
        "Register B: " [rb as usize] '\n'
        "Register C: " [rc as usize] '\n'
        '\n'
        "Program: " [operations split on ',' as u8]
    );
    ([ra, rb, rc], operations)
}

fn combo(registers: &Registers, operand: u8) -> usize {
    match operand {
        0..=3 => operand as usize,
        4..=6 => registers[operand as usize - 4],
        _ => panic!("Reserved combo operand."),
    }
}

pub fn part1(input: &str) -> String {
    let (mut registers, operations) = parse_input(input);
    let mut idx = 0;
    let mut output = Vec::new();
    while idx < operations.len() {
        let instruction: Instruction = operations[idx].into();
        let operand = operations[idx + 1];
        idx += 2;

        match instruction {
            Instruction::Adv => {
                let operand = combo(&registers, operand);
                registers[0] /= 2_usize.pow(operand as u32);
            }
            Instruction::Bxl => {
                registers[1] ^= operand as usize;
            }
            Instruction::Bst => {
                let operand = combo(&registers, operand);
                registers[1] = operand % 8;
            }
            Instruction::Jnz => {
                if registers[0] != 0 {
                    idx = operand as usize;
                }
            }
            Instruction::Bxc => {
                registers[1] ^= registers[2];
            }
            Instruction::Out => {
                let operand = combo(&registers, operand);
                output.push((operand % 8).to_string());
            }
            Instruction::Bdv => {
                let operand = combo(&registers, operand);
                registers[1] = registers[0] / 2_usize.pow(operand as u32);
            }
            Instruction::Cdv => {
                let operand = combo(&registers, operand);
                registers[2] = registers[0] / 2_usize.pow(operand as u32);
            }
        }
    }
    output.join(",")
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = "4,6,3,5,6,3,5,2,1,0")]
    static EXAMPLE_INPUT: &str = "
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = ([729, 0, 0], vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(actual, expected);
    }
}
