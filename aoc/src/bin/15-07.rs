puzzle_runner::register_chapter!(book = "2015", title = "Some Assembly Required");

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Value<'a> {
    Static(u16),
    Gate(&'a str),
}
impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        if let Ok(value) = value.parse() {
            Value::Static(value)
        } else {
            Value::Gate(value)
        }
    }
}
impl<'a> Value<'a> {
    fn get(&self, wires: &HashMap<&'a str, u16>) -> Option<u16> {
        match self {
            Value::Static(value) => Some(*value),
            Value::Gate(wire) => wires.get(wire).copied(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Gate<'a> {
    Static(Value<'a>),
    Not(&'a str),
    And(Value<'a>, &'a str),
    Or(&'a str, &'a str),
    LShift(&'a str, u16),
    RShift(&'a str, u16),
}
impl<'a> From<Vec<&'a str>> for Gate<'a> {
    fn from(value: Vec<&'a str>) -> Self {
        match value.len() {
            1 => Gate::Static(value[0].into()),
            2 => Gate::Not(value[1]),
            3 if value[1] == "AND" => Gate::And(value[0].into(), value[2]),
            3 if value[1] == "OR" => Gate::Or(value[0], value[2]),
            3 if value[1] == "LSHIFT" => Gate::LShift(value[0], value[2].parse().unwrap()),
            3 if value[1] == "RSHIFT" => Gate::RShift(value[0], value[2].parse().unwrap()),
            _ => panic!("Invalid gate definition: {}", value.join(" ")),
        }
    }
}

type Instruction<'a> = (Gate<'a>, &'a str);

fn parse_input(input: &str) -> VecDeque<Instruction<'_>> {
    parse!(input => {
        [instructions split on '\n' into (VecDeque<_>) with
            {
                [gate split try into (Gate)]
                " -> "
                wire
            }
            => (gate, wire)
        ]
    } => instructions)
}

fn solve(mut instructions: VecDeque<Instruction<'_>>) -> HashMap<&str, u16> {
    let mut wires: HashMap<&str, u16> = HashMap::new();
    while let Some((gate, wire)) = instructions.pop_front() {
        let value = match gate {
            Gate::Static(ref value) => value.get(&wires),
            Gate::Not(rhs) => wires.get(rhs).map(|rhs| rhs ^ u16::MAX),
            Gate::And(ref lhs, rhs) => lhs
                .get(&wires)
                .and_then(|lhs| wires.get(rhs).map(|rhs| lhs & rhs)),
            Gate::Or(lhs, rhs) => wires
                .get(lhs)
                .and_then(|lhs| wires.get(rhs).map(|rhs| lhs | rhs)),
            Gate::LShift(lhs, rhs) => wires.get(lhs).map(|lhs| lhs << rhs),
            Gate::RShift(lhs, rhs) => wires.get(lhs).map(|lhs| lhs >> rhs),
        };
        if let Some(value) = value {
            wires.insert(wire, value);
        } else {
            instructions.push_back((gate, wire));
        }
    }
    wires
}

pub fn part1(input: &str) -> u16 {
    let instructions = parse_input(input);
    let wires = solve(instructions);
    wires["a"]
}

pub fn part2(input: &str) -> u16 {
    let instructions = parse_input(input);
    let mut instructions_without_b: VecDeque<Instruction> = instructions
        .iter()
        .filter(|(_, w)| w != &"b")
        .copied()
        .collect();
    let wires = solve(instructions);
    instructions_without_b.push_back((Gate::Static(Value::Static(wires["a"])), "b"));
    let wires = solve(instructions_without_b);
    wires["a"]
}

#[cfg(test)]
mod tests {
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;
    use puzzle_runner::example_input;

    use super::*;

    #[example_input]
    static EXAMPLE_INPUT: &str = "
        123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = vec![
            (Gate::Static(Value::Static(123)), "x"),
            (Gate::Static(Value::Static(456)), "y"),
            (Gate::And(Value::Gate("x"), "y"), "d"),
            (Gate::Or("x", "y"), "e"),
            (Gate::LShift("x", 2), "f"),
            (Gate::RShift("y", 2), "g"),
            (Gate::Not("x"), "h"),
            (Gate::Not("y"), "i"),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_solve() {
        let instructions = parse_input(&EXAMPLE_INPUT);
        let gates = solve(instructions);
        let expected = hash_map!(
            "d" => 72,
            "e" => 507,
            "f" => 492,
            "g" => 114,
            "h" => 65412,
            "i" => 65079,
            "x" => 123,
            "y" => 456,
        );
        assert_eq!(gates, expected);
    }
}
