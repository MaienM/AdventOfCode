use std::{collections::HashMap, ops::RangeInclusive};

use aoc::utils::parse;

#[derive(Debug, PartialEq)]
enum Outcome<'a> {
    Result(bool),
    GoTo(&'a str),
}
impl<'a> From<&'a str> for Outcome<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => Outcome::Result(true),
            "R" => Outcome::Result(false),
            _ => Outcome::GoTo(value),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Var {
    X,
    A,
    M,
    S,
}
impl From<&str> for Var {
    fn from(value: &str) -> Self {
        match value {
            "x" => Var::X,
            "a" => Var::A,
            "m" => Var::M,
            "s" => Var::S,
            _ => panic!("Unknown variable {value:?}."),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Op {
    Gt,
    Lt,
}
impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            ">" => Op::Gt,
            "<" => Op::Lt,
            _ => panic!("Unknown operator {value:?}."),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Instruction<'a> {
    var: Var,
    op: Op,
    val: u16,
    outcome: Outcome<'a>,
}
impl<'a> From<&'a str> for Instruction<'a> {
    fn from(value: &'a str) -> Self {
        parse!(value => condition ':' [outcome as Outcome]);
        Instruction {
            var: condition[0..1].into(),
            op: condition[1..2].into(),
            val: condition[2..].parse().unwrap(),
            outcome,
        }
    }
}
impl Instruction<'_> {
    fn matches(&self, part: &Part) -> bool {
        match (&self.var, &self.op) {
            (Var::X, Op::Gt) => part.x > self.val,
            (Var::X, Op::Lt) => part.x < self.val,
            (Var::A, Op::Gt) => part.a > self.val,
            (Var::A, Op::Lt) => part.a < self.val,
            (Var::M, Op::Gt) => part.m > self.val,
            (Var::M, Op::Lt) => part.m < self.val,
            (Var::S, Op::Gt) => part.s > self.val,
            (Var::S, Op::Lt) => part.s < self.val,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Workflow<'a> {
    instructions: Vec<Instruction<'a>>,
    fallback: Outcome<'a>,
}
impl<'a> From<&'a str> for Workflow<'a> {
    fn from(value: &'a str) -> Self {
        let mut parts: Vec<_> = value.split(',').collect();
        let fallback = parts.pop().unwrap().into();
        Self {
            instructions: parts.into_iter().map(Into::into).collect(),
            fallback,
        }
    }
}
impl Workflow<'_> {
    fn outcome(&self, part: &Part) -> &Outcome {
        for instruction in &self.instructions {
            if instruction.matches(part) {
                return &instruction.outcome;
            }
        }
        &self.fallback
    }
}

#[derive(Debug, PartialEq)]
struct Part {
    x: u16,
    a: u16,
    m: u16,
    s: u16,
}
impl From<&str> for Part {
    fn from(value: &str) -> Self {
        parse!(value => {
            "{x=" [x as u16] ",m=" [m as u16] ",a=" [a as u16] ",s=" [s as u16] "}"
        } => Self { x, a, m, s })
    }
}

#[derive(Debug, PartialEq)]
struct Input<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part>,
}

fn parse_input(input: &str) -> Input {
    parse!(input => {
        [workflows split on '\n' into (HashMap<_, _>) with
            { name '{' [workflow as Workflow] '}' }
            => (name, workflow)
        ]
        "\n\n"
        [parts split on '\n' as Part]
    } => Input { workflows, parts })
}

pub fn part1(input: &str) -> usize {
    let input = parse_input(input);
    input
        .parts
        .into_iter()
        .filter_map(|part| {
            let mut current = "in";
            loop {
                let workflow = input.workflows.get(current).unwrap();
                match workflow.outcome(&part) {
                    Outcome::Result(true) => {
                        return Some(
                            part.x as usize + part.m as usize + part.a as usize + part.s as usize,
                        )
                    }
                    Outcome::Result(false) => return None,
                    Outcome::GoTo(next) => current = next,
                }
            }
        })
        .sum()
}

fn count_accepted(
    input: &Input,
    current: &str,
    mut x: RangeInclusive<u16>,
    mut m: RangeInclusive<u16>,
    mut a: RangeInclusive<u16>,
    mut s: RangeInclusive<u16>,
) -> usize {
    let workflow = input.workflows.get(current).unwrap();
    let mut sum = 0;
    for instruction in &workflow.instructions {
        let (mut match_x, mut match_m, mut match_a, mut match_s) =
            (x.clone(), m.clone(), a.clone(), s.clone());
        #[allow(clippy::range_minus_one)]
        match (&instruction.var, &instruction.op) {
            (Var::X, Op::Gt) => {
                match_x = (instruction.val + 1)..=(*x.end());
                x = (*x.start())..=(instruction.val);
            }
            (Var::X, Op::Lt) => {
                match_x = (*x.start())..=(instruction.val - 1);
                x = (instruction.val)..=(*x.end());
            }
            (Var::M, Op::Gt) => {
                match_m = (instruction.val + 1)..=(*m.end());
                m = (*m.start())..=(instruction.val);
            }
            (Var::M, Op::Lt) => {
                match_m = (*m.start())..=(instruction.val - 1);
                m = (instruction.val)..=(*m.end());
            }
            (Var::A, Op::Gt) => {
                match_a = (instruction.val + 1)..=(*a.end());
                a = (*a.start())..=(instruction.val);
            }
            (Var::A, Op::Lt) => {
                match_a = (*a.start())..=(instruction.val - 1);
                a = (instruction.val)..=(*a.end());
            }
            (Var::S, Op::Gt) => {
                match_s = (instruction.val + 1)..=(*s.end());
                s = (*s.start())..=(instruction.val);
            }
            (Var::S, Op::Lt) => {
                match_s = (*s.start())..=(instruction.val - 1);
                s = (instruction.val)..=(*s.end());
            }
        };
        sum += match instruction.outcome {
            Outcome::Result(true) => match_x.len() * match_m.len() * match_a.len() * match_s.len(),
            Outcome::Result(false) => 0,
            Outcome::GoTo(next) => count_accepted(input, next, match_x, match_m, match_a, match_s),
        };
    }
    sum += match workflow.fallback {
        Outcome::Result(true) => x.len() * m.len() * a.len() * s.len(),
        Outcome::Result(false) => 0,
        Outcome::GoTo(next) => count_accepted(input, next, x, m, a, s),
    };
    sum
}

pub fn part2(input: &str) -> usize {
    let input = parse_input(input);
    count_accepted(&input, "in", 1..=4000, 1..=4000, 1..=4000, 1..=4000)
}

aoc::cli::single::generate_main!();

#[cfg(test)]
mod tests {
    use aoc_derive::example_input;
    use common_macros::hash_map;
    use pretty_assertions::assert_eq;

    use super::*;

    #[example_input(part1 = 19_114, part2 = 167_409_079_868_000)]
    static EXAMPLE_INPUT: &str = "
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    ";

    #[allow(clippy::too_many_lines)]
    #[test]
    fn example_parse() {
        let actual = parse_input(&EXAMPLE_INPUT);
        let expected = Input {
            workflows: hash_map![
                "px" => Workflow {
                    instructions: vec![
                        Instruction { var: Var::A, op: Op::Lt, val: 2006, outcome: Outcome::GoTo("qkq")},
                        Instruction { var: Var::M, op: Op::Gt, val: 2090, outcome: Outcome::Result(true) },
                    ],
                    fallback: Outcome::GoTo("rfg"),
                },
                "pv" => Workflow {
                    instructions: vec![
                        Instruction { var: Var::A, op: Op::Gt, val: 1716, outcome: Outcome::Result(false) },
                    ],
                    fallback: Outcome::Result(true),
                },
                "lnx" => Workflow {
                    instructions: vec![
                        Instruction { var: Var::M, op: Op::Gt, val: 1548, outcome: Outcome::Result(true) },
                    ],
                    fallback: Outcome::Result(true),
                },
                "rfg" => Workflow {
                    instructions: vec![
                        Instruction { var: Var::S, op: Op::Lt, val: 537, outcome: Outcome::GoTo("gd")},
                        Instruction { var: Var::X, op: Op::Gt, val: 2440, outcome: Outcome::Result(false) },
                    ],
                    fallback: Outcome::Result(true),
                },
                "qs" => Workflow {
                    instructions: vec![
                        Instruction { var: Var::S, op: Op::Gt, val: 3448, outcome: Outcome::Result(true) },
                    ],
                    fallback: Outcome::GoTo("lnx"),
                },
                "qkq" => Workflow {
                    instructions: vec![
                        Instruction { var: Var::X, op: Op::Lt, val: 1416, outcome: Outcome::Result(true) },
                    ],
                    fallback: Outcome::GoTo("crn"),
                },
                "crn" => Workflow {
                    instructions: vec![
                        Instruction { var: Var::X, op: Op::Gt, val: 2662, outcome: Outcome::Result(true) },
                    ],
                    fallback: Outcome::Result(false),
                },
                "in" => Workflow {
                    instructions: vec![
                        Instruction { var: Var::S, op: Op::Lt, val: 1351, outcome: Outcome::GoTo("px")},
                    ],
                    fallback: Outcome::GoTo("qqz"),
                },
                "qqz" => Workflow {
                    instructions: vec![
                        Instruction { var: Var::S, op: Op::Gt, val: 2770, outcome: Outcome::GoTo("qs")},
                        Instruction { var: Var::M, op: Op::Lt, val: 1801, outcome: Outcome::GoTo("hdj")},
                    ],
                    fallback: Outcome::Result(false),
                },
                "gd" => Workflow {
                    instructions: vec![
                        Instruction { var: Var::A, op: Op::Gt, val: 3333, outcome: Outcome::Result(false) },
                    ],
                    fallback: Outcome::Result(false),
                },
                "hdj" => Workflow {
                    instructions: vec![
                        Instruction { var: Var::M, op: Op::Gt, val: 838, outcome: Outcome::Result(true) },
                    ],
                    fallback: Outcome::GoTo("pv"),
                },
            ],

            parts: vec![
                Part {
                    x: 787,
                    m: 2655,
                    a: 1222,
                    s: 2876,
                },
                Part {
                    x: 1679,
                    m: 44,
                    a: 2067,
                    s: 496,
                },
                Part {
                    x: 2036,
                    m: 264,
                    a: 79,
                    s: 2244,
                },
                Part {
                    x: 2461,
                    m: 1339,
                    a: 466,
                    s: 291,
                },
                Part {
                    x: 2127,
                    m: 1623,
                    a: 2188,
                    s: 1013,
                },
            ],
        };
        assert_eq!(actual, expected);
    }
}
