aoc::setup!(title = "");
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{collections::VecDeque, fmt::Debug, ops::RangeInclusive};


type Registers = [i64; 4];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Variable {
    W = 0,
    X = 1,
    Y = 2,
    Z = 3,
}
impl Variable {
    fn usize(self) -> usize {
        return self as usize;
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Value {
    Variable(Variable),
    Number(i16),
}
impl Value {
    fn get(&self, registers: &Registers) -> i64 {
        return match self {
            Value::Variable(v) => registers[v.usize()],
            Value::Number(n) => *n as i64,
        };
    }
}
impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable(var) => write!(f, "{:?}", var),
            Self::Number(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Instruction {
    Inp(Variable),
    Add(Variable, Value),
    Mul(Variable, Value),
    Div(Variable, Value),
    Mod(Variable, Value),
    Eql(Variable, Value),
    // Extra instructions not in the input
    Set(Variable, Value),
}
impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Inp(var) => write!(f, "imp {:?}", var),
            Instruction::Add(var, val) => write!(f, "add {:?} {:?}", var, val),
            Instruction::Mul(var, val) => write!(f, "mul {:?} {:?}", var, val),
            Instruction::Div(var, val) => write!(f, "div {:?} {:?}", var, val),
            Instruction::Mod(var, val) => write!(f, "mod {:?} {:?}", var, val),
            Instruction::Eql(var, val) => write!(f, "eql {:?} {:?}", var, val),
            Instruction::Set(var, val) => write!(f, "set {:?} {:?}", var, val),
        }
    }
}

fn parse_var(name: &str) -> Result<Variable, &'static str> {
    return match name {
        "w" => Ok(Variable::W),
        "x" => Ok(Variable::X),
        "y" => Ok(Variable::Y),
        "z" => Ok(Variable::Z),
        _ => Err("Invalid variable name."),
    };
}

fn parse_value(value: Option<&str>) -> Result<Value, String> {
    if value.is_none() {
        return Err("Missing argument.".to_string());
    }
    let value = value.unwrap();

    let asvar = parse_var(value);
    if asvar.is_ok() {
        return asvar.map(|v| Value::Variable(v)).map_err(str::to_string);
    }

    return value
        .parse()
        .map(|v| Value::Number(v))
        .map_err(|e| e.to_string());
}

fn parse_input(input: &str) -> Vec<Instruction> {
    return input
        .trim()
        .split("\n")
        .map(str::trim)
        .map(|line| {
            let mut parts = line.split(" ");
            let cmd = parts.next().unwrap();
            let var = parse_var(parts.next().unwrap()).unwrap();
            let value = parse_value(parts.next());

            return match cmd {
                "inp" => Instruction::Inp(var),
                "add" => Instruction::Add(var, value.unwrap()),
                "mul" => Instruction::Mul(var, value.unwrap()),
                "div" => Instruction::Div(var, value.unwrap()),
                "mod" => Instruction::Mod(var, value.unwrap()),
                "eql" => Instruction::Eql(var, value.unwrap()),
                _ => panic!("Invalid instruction {}.", cmd),
            };
        })
        .collect();
}

fn ranges_overlap(lhs: &RangeInclusive<i16>, rhs: &RangeInclusive<i16>) -> bool {
    return lhs.contains(rhs.start())
        || lhs.contains(rhs.end())
        || rhs.contains(lhs.start())
        || rhs.contains(lhs.end());
}

fn _optimize_instructions(instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut result = Vec::new();
    let mut bounds = [0..=0, 0..=0, 0..=0, 0..=0];
    for instruction in instructions {
        let instruction = match instruction {
            // Mathematical noops.
            Instruction::Add(_, Value::Number(0)) => continue,
            Instruction::Mul(_, Value::Number(1)) => continue,
            Instruction::Div(_, Value::Number(1)) => continue,

            // Noops on 0.
            Instruction::Mul(var, _) if bounds[var.usize()] == (0..=0) => continue,
            Instruction::Div(var, _) if bounds[var.usize()] == (0..=0) => continue,

            // Multiply by 0 = 0.
            Instruction::Mul(var, Value::Number(0)) => Instruction::Set(var, Value::Number(0)),

            // Adding to 0 is the same as setting.
            Instruction::Add(var, val) if bounds[var.usize()] == (0..=0) => {
                Instruction::Set(var, val)
            }

            // Variable uses where the variable can only have a single value at that point. (Mostly important for the usage tracking step.)
            Instruction::Add(var, Value::Variable(ovar)) if bounds[ovar.usize()].len() == 1 => {
                Instruction::Add(var, Value::Number(*bounds[ovar.usize()].start()))
            }
            Instruction::Mul(var, Value::Variable(ovar)) if bounds[ovar.usize()].len() == 1 => {
                Instruction::Mul(var, Value::Number(*bounds[ovar.usize()].start()))
            }
            Instruction::Div(var, Value::Variable(ovar)) if bounds[ovar.usize()].len() == 1 => {
                Instruction::Div(var, Value::Number(*bounds[ovar.usize()].start()))
            }
            Instruction::Mod(var, Value::Variable(ovar)) if bounds[ovar.usize()].len() == 1 => {
                Instruction::Mod(var, Value::Number(*bounds[ovar.usize()].start()))
            }
            Instruction::Eql(var, Value::Variable(ovar)) if bounds[ovar.usize()].len() == 1 => {
                Instruction::Eql(var, Value::Number(*bounds[ovar.usize()].start()))
            }
            Instruction::Set(var, Value::Variable(ovar)) if bounds[ovar.usize()].len() == 1 => {
                Instruction::Set(var, Value::Number(*bounds[ovar.usize()].start()))
            }

            // Noops through bounds tracking.
            Instruction::Set(var, Value::Number(num)) if bounds[var.usize()] == (num..=num) => {
                continue;
            }
            Instruction::Set(var, Value::Variable(ovar))
                if bounds[var.usize()] == bounds[ovar.usize()] =>
            {
                continue;
            }

            // Noop modulos.
            Instruction::Mod(var, Value::Number(num)) if !bounds[var.usize()].contains(&num) => {
                continue;
            }
            Instruction::Mod(var, Value::Variable(ovar))
                if bounds[var.usize()].end() < bounds[ovar.usize()].start() =>
            {
                continue;
            }

            // Comparisons that we can predict the result of using bounds tracking.
            Instruction::Eql(var, Value::Variable(ovar))
                if bounds[var.usize()] == bounds[ovar.usize()] =>
            {
                Instruction::Set(var, Value::Number(1))
            }
            Instruction::Eql(var, Value::Variable(ovar))
                if !ranges_overlap(&bounds[var.usize()], &bounds[ovar.usize()]) =>
            {
                Instruction::Set(var, Value::Number(0))
            }
            Instruction::Eql(var, Value::Number(num)) if bounds[var.usize()] == (num..=num) => {
                Instruction::Set(var, Value::Number(1))
            }
            Instruction::Eql(var, Value::Number(num)) if !bounds[var.usize()].contains(&num) => {
                Instruction::Set(var, Value::Number(0))
            }

            instruction => instruction,
        };

        // Update the bounds of the affected variable.
        match instruction {
            Instruction::Inp(var) => {
                bounds[var.usize()] = 1..=9;
            }
            Instruction::Add(var, Value::Number(num)) => {
                let b = bounds[var.usize()].clone();
                bounds[var.usize()] = (b.start() + num)..=(b.end() + num);
            }
            Instruction::Add(var, Value::Variable(ovar)) => {
                let b = bounds[var.usize()].clone();
                let ob = bounds[ovar.usize()].clone();
                bounds[var.usize()] = (b.start() + ob.start())..=(b.end() + ob.end());
            }
            Instruction::Mul(var, Value::Number(num)) => {
                let b = bounds[var.usize()].clone();
                bounds[var.usize()] = (b.start() * num)..=(b.end() * num);
            }
            Instruction::Mul(var, Value::Variable(ovar)) => {
                let b = bounds[var.usize()].clone();
                let ob = bounds[ovar.usize()].clone();
                bounds[var.usize()] = (b.start() * ob.start())..=(b.end() * ob.end());
            }
            Instruction::Div(var, Value::Number(num)) => {
                let b = bounds[var.usize()].clone();
                bounds[var.usize()] = (b.start() / num)..=(b.end() / num);
            }
            Instruction::Div(var, Value::Variable(ovar)) => {
                let b = bounds[var.usize()].clone();
                let ob = bounds[ovar.usize()].clone();
                bounds[var.usize()] = (b.start() / ob.start())..=(b.end() / ob.end());
            }
            Instruction::Mod(var, Value::Number(num)) => {
                let b = bounds[var.usize()].clone();
                bounds[var.usize()] = (b.start() % num)..=(b.end() % num);
            }
            Instruction::Mod(var, Value::Variable(ovar)) => {
                let b = bounds[var.usize()].clone();
                let ob = bounds[ovar.usize()].clone();
                bounds[var.usize()] = (b.start() % ob.end())..=(b.end() % ob.end());
            }
            Instruction::Eql(var, Value::Number(num)) => {
                let b = bounds[var.usize()].clone();
                if b == (num..=num) {
                    bounds[var.usize()] = 1..=1;
                } else if b.contains(&num) {
                    bounds[var.usize()] = 0..=1;
                } else {
                    bounds[var.usize()] = 0..=0;
                }
            }
            Instruction::Eql(var, Value::Variable(ovar)) => {
                let b = bounds[var.usize()].clone();
                let ob = bounds[ovar.usize()].clone();
                if b == ob {
                    bounds[var.usize()] = 1..=1;
                } else if ranges_overlap(&b, &ob) {
                    bounds[var.usize()] = 0..=1;
                } else {
                    bounds[var.usize()] = 0..=0;
                }
            }
            Instruction::Set(var, Value::Number(num)) => {
                bounds[var.usize()] = num..=num;
            }
            Instruction::Set(var, Value::Variable(ovar)) => {
                bounds[var.usize()] = bounds[ovar.usize()].clone();
            }
        }

        result.push(instruction);
    }
    return result;
}

fn _cull_unused_instructions(mut instructions: Vec<Instruction>) -> Vec<Instruction> {
    let iter = instructions.into_iter().rev();
    instructions = Vec::new();
    let mut used = [false, false, false, true];
    for instruction in iter {
        // If the result of the instruction is not used we can skip the instruction entirely.
        match instruction {
            Instruction::Add(var, _) if !used[var.usize()] => continue,
            Instruction::Mul(var, _) if !used[var.usize()] => continue,
            Instruction::Div(var, _) if !used[var.usize()] => continue,
            Instruction::Mod(var, _) if !used[var.usize()] => continue,
            Instruction::Eql(var, _) if !used[var.usize()] => continue,
            Instruction::Inp(var) if !used[var.usize()] => continue,
            Instruction::Set(var, _) if !used[var.usize()] => continue,
            _ => {}
        }

        match instruction {
            // Operations that set without using.
            Instruction::Inp(var) => {
                used[var.usize()] = false;
            }
            Instruction::Set(var, Value::Number(_)) => {
                used[var.usize()] = false;
            }

            // Set that uses another variable.
            Instruction::Set(var, Value::Variable(ovar)) => {
                used[var.usize()] = false;
                used[ovar.usize()] = true;
            }

            // Instructions that use.
            Instruction::Add(_, Value::Variable(var)) => {
                used[var.usize()] = true;
            }
            Instruction::Mul(_, Value::Variable(var)) => {
                used[var.usize()] = true;
            }
            Instruction::Div(_, Value::Variable(var)) => {
                used[var.usize()] = true;
            }
            Instruction::Mod(_, Value::Variable(var)) => {
                used[var.usize()] = true;
            }
            Instruction::Eql(_, Value::Variable(var)) => {
                used[var.usize()] = true;
            }

            _ => {}
        }
        instructions.push(instruction);
    }
    return instructions.into_iter().rev().collect();
}

fn optimize_instructions(mut instructions: Vec<Instruction>) -> Vec<Instruction> {
    let mut last = 0;
    loop {
        let mut s = DefaultHasher::new();
        instructions.hash(&mut s);
        let hash = s.finish();
        if hash == last {
            break;
        }
        last = hash;

        instructions = _optimize_instructions(instructions);
        instructions = _cull_unused_instructions(instructions);
    }
    return instructions;
}

fn apply_instructions(
    instructions: &Vec<Instruction>,
    registers: &mut Registers,
    mut input: VecDeque<u8>,
) {
    for instruction in instructions {
        match instruction {
            Instruction::Inp(var) => {
                registers[var.usize()] = input.pop_front().unwrap() as i64;
            }
            Instruction::Add(var, val) => {
                registers[var.usize()] += val.get(&registers);
            }
            Instruction::Mul(var, val) => {
                registers[var.usize()] *= val.get(&registers);
            }
            Instruction::Div(var, val) => {
                registers[var.usize()] /= val.get(&registers);
            }
            Instruction::Mod(var, val) => {
                registers[var.usize()] %= val.get(&registers);
            }
            Instruction::Eql(var, val) => {
                registers[var.usize()] = (registers[var.usize()] == val.get(&registers)) as i64;
            }
            Instruction::Set(var, val) => {
                registers[var.usize()] = val.get(&registers);
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let instructions = parse_input(input);
    let instructions = optimize_instructions(instructions);

    // Split the instructions into parts starting at each new input. This was we can avoid running things using only inputs 1-3 when changing 4 or later, and so on.
    const EMPTY: Vec<Instruction> = Vec::new();
    let mut instruction_segments: [Vec<Instruction>; 14] = [EMPTY; 14];
    let mut i = 0;
    instruction_segments[0].push(instructions[0].clone());
    for instruction in instructions.into_iter().skip(1) {
        match instruction {
            Instruction::Inp(_) => {
                i += 1;
            }
            _ => {}
        };
        instruction_segments[i].push(instruction);
    }

    let mut registers: [Registers; 15] = [[0; 4]; 15];
    let mut nums = [0; 14];
    for i in 0..14 {
        for n in range(9, 1) {
            registers[i + 1] = registers[i].clone();
            apply_instructions(
                &instruction_segments[i],
                &mut registers[i + 1],
                vec![n as u8].into(),
            );
            if registers[i + 1][3] == 0 {
                nums[i] = n;
                break;
            }
        }
    }

    let result: String = nums.into_iter().map(|n| n.to_string()).collect();
    return result.parse().unwrap();
}

#[cfg(test)]
mod tests {
    use aoc_runner::example_input;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 0, part2 = 0)]
    static EXAMPLE_INPUT: &str = "
        inp w
        add z w
        mod z 2
        div w 2
        add y w
        mod y 2
        div w 2
        add x w
        mod x 2
        div w 2
        mod w 2
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            Instruction::Inp(Variable::W),
            Instruction::Add(Variable::Z, Value::Variable(Variable::W)),
            Instruction::Mod(Variable::Z, Value::Number(2)),
            Instruction::Div(Variable::W, Value::Number(2)),
            Instruction::Add(Variable::Y, Value::Variable(Variable::W)),
            Instruction::Mod(Variable::Y, Value::Number(2)),
            Instruction::Div(Variable::W, Value::Number(2)),
            Instruction::Add(Variable::X, Value::Variable(Variable::W)),
            Instruction::Mod(Variable::X, Value::Number(2)),
            Instruction::Div(Variable::W, Value::Number(2)),
            Instruction::Mod(Variable::W, Value::Number(2)),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_apply() {
        let instructions = vec![
            Instruction::Inp(Variable::Z),
            Instruction::Inp(Variable::X),
            Instruction::Mul(Variable::Z, Value::Number(3)),
            Instruction::Eql(Variable::Z, Value::Variable(Variable::X)),
        ];
        let mut registers = [0, 0, 0, 0];
        apply_instructions(&instructions, &mut registers, vec![1, 3].into());
        assert_eq!(registers[3], 1);
        let mut registers = [0, 0, 0, 0];
        apply_instructions(&instructions, &mut registers, vec![1, 2].into());
        assert_eq!(registers[3], 0);

        let instructions = vec![
            Instruction::Inp(Variable::W),
            Instruction::Add(Variable::Z, Value::Variable(Variable::W)),
            Instruction::Mod(Variable::Z, Value::Number(2)),
            Instruction::Div(Variable::W, Value::Number(2)),
            Instruction::Add(Variable::Y, Value::Variable(Variable::W)),
            Instruction::Mod(Variable::Y, Value::Number(2)),
            Instruction::Div(Variable::W, Value::Number(2)),
            Instruction::Add(Variable::X, Value::Variable(Variable::W)),
            Instruction::Mod(Variable::X, Value::Number(2)),
            Instruction::Div(Variable::W, Value::Number(2)),
            Instruction::Mod(Variable::W, Value::Number(2)),
        ];
        let mut registers = [0, 0, 0, 0];
        apply_instructions(&instructions, &mut registers, vec![3].into());
        assert_eq!(registers, [0, 0, 1, 1]);
        let mut registers = [0, 0, 0, 0];
        apply_instructions(&instructions, &mut registers, vec![13].into());
        assert_eq!(registers, [1, 1, 0, 1]);
    }

    #[test]
    fn example_optimize() {
        let input = vec![
            Instruction::Inp(Variable::W),
            Instruction::Mul(Variable::X, Value::Number(0)),
            Instruction::Add(Variable::X, Value::Variable(Variable::Z)),
            Instruction::Mod(Variable::X, Value::Number(26)),
            Instruction::Div(Variable::Z, Value::Number(1)),
            Instruction::Add(Variable::X, Value::Number(13)),
            Instruction::Eql(Variable::X, Value::Variable(Variable::W)),
            Instruction::Eql(Variable::X, Value::Number(0)),
            Instruction::Mul(Variable::Y, Value::Number(0)),
            Instruction::Add(Variable::Y, Value::Number(25)),
            Instruction::Mul(Variable::Y, Value::Variable(Variable::X)),
            Instruction::Add(Variable::Y, Value::Number(1)),
            Instruction::Mul(Variable::Z, Value::Variable(Variable::Y)),
            Instruction::Mul(Variable::Y, Value::Number(0)),
            Instruction::Add(Variable::Y, Value::Variable(Variable::W)),
            Instruction::Add(Variable::Y, Value::Number(8)),
            Instruction::Mul(Variable::Y, Value::Variable(Variable::X)),
            Instruction::Add(Variable::Z, Value::Variable(Variable::Y)),
        ];
        let actual = optimize_instructions(input);
        let expected = vec![
            Instruction::Inp(Variable::W),
            Instruction::Set(Variable::Y, Value::Variable(Variable::W)),
            Instruction::Add(Variable::Y, Value::Number(8)),
            Instruction::Set(Variable::Z, Value::Variable(Variable::Y)),
        ];
        assert_eq!(actual, expected);
    }

    // There is no example input/output for part 1.
}
