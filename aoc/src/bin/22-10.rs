puzzle_lib::setup!(title = "Cathode-Ray Tube");

use std::convert::TryInto;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    AddX(i16),
    NoOp,
}
impl Instruction {
    fn len(&self) -> usize {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::NoOp => 1,
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    parse!(input => {
        [instructions split on '\n' with |line| {
            match &line[0..4] {
                "addx" => Instruction::AddX(line[5..].parse().unwrap()),
                "noop" => Instruction::NoOp,
                _ => panic!(),
            }
        }]
    } => instructions)
}

fn run_instructions(instructions: Vec<Instruction>, callback: &mut impl FnMut(usize, i16)) {
    let mut cycle = 1;
    let mut x = 1;
    for instruction in instructions {
        for _ in 0..instruction.len() {
            callback(cycle, x);
            cycle += 1;
        }
        match instruction {
            Instruction::AddX(change) => x += change,
            Instruction::NoOp => {}
        }
    }
}

pub fn part1(input: &str) -> i16 {
    let instructions = parse_input(input);
    let mut signal: i16 = 0;
    run_instructions(instructions, &mut |cycle, x| {
        if cycle % 40 == 20 {
            signal += TryInto::<i16>::try_into(cycle).unwrap() * x;
        }
    });
    signal
}

pub fn part2(input: &str) -> String {
    let instructions = parse_input(input);
    let mut output = String::new();
    run_instructions(instructions, &mut |cycle, x| {
        let pos = ((cycle - 1) % 40).try_into().unwrap();
        if x.checked_sub_unsigned(pos).unwrap().abs() <= 1 {
            output += "█";
        } else {
            output += " ";
        }
        if cycle % 40 == 0 {
            output += "\n";
        }
    });
    output.trim_end_matches('\n').to_string()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input(
        part1 = 13_140,
        part2 = "
            ██  ██  ██  ██  ██  ██  ██  ██  ██  ██  
            ███   ███   ███   ███   ███   ███   ███ 
            ████    ████    ████    ████    ████    
            █████     █████     █████     █████     
            ██████      ██████      ██████      ████
            ███████       ███████       ███████     
        "
    )]
    static EXAMPLE_INPUT: &str = "
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    ";

    #[test]
    #[allow(clippy::too_many_lines)]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Instruction::AddX(15),
            Instruction::AddX(-11),
            Instruction::AddX(6),
            Instruction::AddX(-3),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(-8),
            Instruction::AddX(13),
            Instruction::AddX(4),
            Instruction::NoOp,
            Instruction::AddX(-1),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(5),
            Instruction::AddX(-1),
            Instruction::AddX(-35),
            Instruction::AddX(1),
            Instruction::AddX(24),
            Instruction::AddX(-19),
            Instruction::AddX(1),
            Instruction::AddX(16),
            Instruction::AddX(-11),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(21),
            Instruction::AddX(-15),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(-3),
            Instruction::AddX(9),
            Instruction::AddX(1),
            Instruction::AddX(-3),
            Instruction::AddX(8),
            Instruction::AddX(1),
            Instruction::AddX(5),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(-36),
            Instruction::NoOp,
            Instruction::AddX(1),
            Instruction::AddX(7),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(2),
            Instruction::AddX(6),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(7),
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::AddX(-13),
            Instruction::AddX(13),
            Instruction::AddX(7),
            Instruction::NoOp,
            Instruction::AddX(1),
            Instruction::AddX(-33),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(2),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(8),
            Instruction::NoOp,
            Instruction::AddX(-1),
            Instruction::AddX(2),
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::AddX(17),
            Instruction::AddX(-9),
            Instruction::AddX(1),
            Instruction::AddX(1),
            Instruction::AddX(-3),
            Instruction::AddX(11),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(-13),
            Instruction::AddX(-19),
            Instruction::AddX(1),
            Instruction::AddX(3),
            Instruction::AddX(26),
            Instruction::AddX(-30),
            Instruction::AddX(12),
            Instruction::AddX(-1),
            Instruction::AddX(3),
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(-9),
            Instruction::AddX(18),
            Instruction::AddX(1),
            Instruction::AddX(2),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(9),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(-1),
            Instruction::AddX(2),
            Instruction::AddX(-37),
            Instruction::AddX(1),
            Instruction::AddX(3),
            Instruction::NoOp,
            Instruction::AddX(15),
            Instruction::AddX(-21),
            Instruction::AddX(22),
            Instruction::AddX(-6),
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::AddX(2),
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::AddX(-10),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::AddX(20),
            Instruction::AddX(1),
            Instruction::AddX(2),
            Instruction::AddX(2),
            Instruction::AddX(-6),
            Instruction::AddX(-11),
            Instruction::NoOp,
            Instruction::NoOp,
            Instruction::NoOp,
        ];
        assert_eq!(actual, expected);
    }
}
