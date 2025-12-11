puzzle_runner::register_chapter!(book = "2024", title = "Chronospatial Computer");

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

fn run(mut registers: Registers, operations: &[u8]) -> Vec<u8> {
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
                output.push((operand % 8) as u8);
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
    output
}

#[register_part]
fn part1(input: &str) -> String {
    let (registers, operations) = parse_input(input);
    let output = run(registers, &operations);
    output
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[register_part]
fn part2(input: &str) -> usize {
    let (mut registers, operations) = parse_input(input);

    // Each loop in the program register A will be shift by 3 (modulo 8). As a result of this each output number depends on exactly 3 bits of the input number. This means we can simply increment until the last number matches what we want, and then shift by 3 and start working on the next 3 bits for the second-to-last number, and so on.
    registers[0] = 1;
    loop {
        let output = run(registers, &operations);
        if output == operations[(operations.len() - output.len())..] {
            if output.len() == operations.len() {
                return registers[0];
            }
            registers[0] *= 8;
        } else {
            registers[0] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(part1 = "4,6,3,5,6,3,5,2,1,0")]
    static EXAMPLE_INPUT_1: &str = "
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    ";

    #[example_input(part2 = 117_440)]
    static EXAMPLE_INPUT_2: &str = "
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT_1);
        let expected = ([729, 0, 0], vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(actual, expected);
    }
}
